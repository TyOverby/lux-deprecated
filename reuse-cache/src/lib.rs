use std::rc::Rc;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};

#[derive(Clone)]
pub struct ReuseCache<T> {
    all: Rc<Vec<RefCell<(bool, Option<T>)>>>
}

pub struct Item<T> {
    parent_cache: Option<ReuseCache<T>>,
    idx: usize,
    item: Option<T>,
    poisoned: bool
}

impl <T> ReuseCache<T> {
    pub fn new<F: FnMut() -> T>(count: usize, mut init: F) -> ReuseCache<T> {
        let mut v = Vec::new();
        v.extend((0 .. count).map(|_| RefCell::new((false, Some(init())))));
        ReuseCache { all: Rc::new(v) }
    }

    pub fn get(&self) -> Option<Item<T>> {
        for (i, slot) in self.all.iter().enumerate() {
            if !slot.borrow().0 && slot.borrow().1.is_some() {
                return Some(Item {
                    parent_cache: Some(ReuseCache{all: self.all.clone()}),
                    idx: i,
                    item: slot.borrow_mut().1.take(),
                    poisoned: false
                })
            }
        }

        None
    }

    pub fn get_or(&self, v: T) -> Item<T> {
        self.get().unwrap_or(Item::from_value(v))
    }

    pub fn get_or_else<F: FnOnce() -> T>(&self, f: F) -> Item<T> {
        self.get().unwrap_or_else(|| {
            Item::from_value(f())
        })
    }

    /// Removes poison from all the internal items
    pub fn clean_all(&self) {
        for slot in self.all.iter() {
            *(&mut slot.borrow_mut().0) = false;
        }
    }
}

impl <T> Item<T> {
    /// This will not be placed back in a cache.
    pub fn from_value(value: T) -> Item<T> {
        Item {
            parent_cache: None,
            idx: 0,
            item: Some(value),
            poisoned: false
        }
    }

    pub fn replace(&mut self, new: T) -> T {
        let old = self.item.take().unwrap();
        self.item = Some(new);
        old
    }

    pub fn poison(mut self) {
        self.poisoned = true;
    }
}

impl <T> Deref for Item<T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.item.as_ref().unwrap()
    }
}

impl <T> DerefMut for Item<T> {
    fn deref_mut(&mut self) -> &mut T {
        self.item.as_mut().unwrap()
    }
}

impl <T> Drop for Item<T> {
    fn drop(&mut self) {
        let it = self.item.take();
        if let Some(pc) = self.parent_cache.take() {
            *(pc.all.get(self.idx).unwrap().borrow_mut()) = (self.poisoned, it);
        }
    }
}

#[test]
fn test_empty() {
    let rc = ReuseCache::new(0, || 0u32);
    assert!(rc.get().is_none())
}

#[test]
fn test_single() {
    let rc = ReuseCache::new(1, || 5u32);
    assert!(&*rc.get().unwrap() == &5u32)
}

#[test]
fn test_reuse() {
    let rc = ReuseCache::new(1, || 5u32);

    {
        let mut it = rc.get().unwrap();
        *it = 10u32;
    }

    {
        let it = rc.get().unwrap();
        assert!(&*it == &10u32)
    }
}

#[test]
fn test_taken() {
    let rc = ReuseCache::new(1, || 5u32);
    let it1 = rc.get();
    assert!(it1.is_some());
    let it2 = rc.get();
    assert!(it2.is_none());
}

#[test]
fn test_replace() {
    let rc = ReuseCache::new(1, || 5u32);
    {
        let mut it = rc.get().unwrap();
        assert!(it.replace(4) == 5);
    }

    {
        let it = rc.get().unwrap();
        assert!(*it == 4)
    }
}

#[test]
fn test_poison() {
    let rc = ReuseCache::new(1, || 5u32);

    {
        let mut it = rc.get().unwrap();
        *it = 10u32;
        it.poison()
    }

    {
        assert!(rc.get().is_none());
    }
}

#[test]
fn test_unpoison() {
    let rc = ReuseCache::new(1, || 5u32);

    {
        let mut it = rc.get().unwrap();
        *it = 10u32;
        it.poison()
    }

    rc.clean_all();

    {
        assert!(rc.get().is_some());
    }
}
