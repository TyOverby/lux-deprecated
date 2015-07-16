use std::collections::VecDeque;
use std::collections::vec_deque::{Iter, IterMut};
use std::path::PathBuf;

use self::keycodes::*;

use super::types::Float;

pub mod keycodes {
    //! A keycode is a platform independent way to refer to
    //! keys on the keyboard.
    pub use glutin::VirtualKeyCode;
    pub use glutin::VirtualKeyCode::*;
}

/// An iterator for windowing events.
///
/// It contains a `VecDeque<Event>` internally
/// that it pulls items from during iteration.
pub struct EventIterator {
    backing: VecDeque<Event>
}

// TODO: pub use glutin::MouseScrollDelta when it derives PartialEq
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MouseScrollDelta {
    LineDelta(f32, f32),
    PixelDelta(f32, f32)
}

/// An even coming from an Interactive object.
#[derive(Debug, PartialEq, Clone)]
pub enum Event {
    /// The mouse moved to this position.
    MouseMoved((i32, i32)),
    /// The mouse wheel moved by this delta.
    MouseWheel(MouseScrollDelta),
    /// This mouse button was pushed down.
    MouseDown(MouseButton),
    /// This mouse button was released.
    MouseUp(MouseButton),
    /// This key was pressed.
    ///
    /// The keycode `u8` is always given, which can sometimes be translated
    /// into a `char` and can sometimes be translated to a `VirtualKeyCode`.
    KeyPressed(u8, Option<char>, Option<keycodes::VirtualKeyCode>),
    /// This key was released.
    KeyReleased(u8, Option<char>, Option<keycodes::VirtualKeyCode>),
    /// The window was resized to this size.
    WindowResized((u32, u32)),
    /// The window was moved to this position on the screen.
    WindowMoved((i32, i32)),
    /// A file has been dragged-and-dropped into the screen.
    FileDropped(PathBuf)
}

/// A handy enumeration for the buttons on a mouse.
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum MouseButton {
    /// The left mouse button.
    Left,
    /// The right mouse button.
    Right,
    /// The middle mouse button.
    Middle,
    /// Any other unnamed mouse button.
    Other(u8)
}

/// A trait for objects that are interactive to the user.
/// The only known impelementation for this trait is the glutin Window.
pub trait Interactive {
    /// Returns true if the window is not yet closed.
    ///
    /// This function borrows self as &mut because it must
    /// process events before determining if it has closed before.
    ///
    /// `was_open` is a similar function, but without the event processing.
    /// However, you should prefer is_open if at all possible.
    fn is_open(&mut self) -> bool;

    /// Returns true if the window wasn't closed the last time that input was
    /// polled.
    fn was_open(&self) -> bool;

    /// Returns the title of the object.
    fn title(&self) -> &str;

    /// Sets the title of the object.  If the object is a window,
    /// this title will be used to decorate the window.
    fn set_title(&mut self, title: &str);

    /// Sets the size of the window if possible.
    fn set_size(&mut self, width: u32, height: u32);

    /// Returns the size of the window as an unsigned integer.
    fn get_size_u(&self) -> (u32, u32);

    /// Returns the size of the window.
    fn get_size(&self) -> (f32, f32) {
        let (x, y) = self.get_size_u();
        (x as f32, y as f32)
    }

    /// Returns the width of the window.
    fn width(&self) -> f32 {
        match self.get_size() {
            (w, _) => w
        }
    }

    /// Returns the height of the window.
    fn height(&self) -> f32 {
        match self.get_size() {
            (_, h) => h
        }
    }

    /// Returns the width of the window as an unsigned integer.
    fn width_u(&self) -> u32 {
        match self.get_size_u() {
            (w, _) => w
        }
    }

    /// Returns the height of the window as an unsigned integer.
    fn height_u(&self) -> u32 {
        match self.get_size_u() {
            (_, h) => h
        }
    }

    // Events

    /// Returns true if the operating system has given this object focus.
    fn is_focused(&self) -> bool;

    /// Returns true if any mouse button is down.
    fn is_mouse_down(&self) -> bool;

    /// Returns the current position of the mouse.
    ///
    /// This function returns the position in floating point units
    /// for usability.  Use `mouse_pos_int` if you want integer units.
    fn mouse_pos(&self) -> (Float, Float);

    /// Returns the current position of the mouse in integer units.
    fn mouse_pos_i(&self) -> (i32, i32);

    /// Returns the x coordinate of the mouse.
    fn mouse_x(&self) -> Float {
        match self.mouse_pos() {
            (x, _) => x
        }
    }

    /// Returns the x coordinate of the mouse in integer units.
    fn mouse_x_i(&self) -> i32 {
        match self.mouse_pos_i() {
            (x, _) => x
        }
    }

    /// Returns the y coordinate of the mouse.
    fn mouse_y(&self) -> Float {
        match self.mouse_pos() {
            (_, y) => y
        }
    }


    /// Returns the y coordinate of the mouse in integer units.
    fn mouse_y_i(&self) -> i32 {
        match self.mouse_pos_i() {
            (y, _) => y
        }
    }

    /// Returns true if a given key is currently being pressed.
    fn is_key_pressed<K: AbstractKey>(&self, k: K) -> bool;

    /// Consumes all unseen events and returns them in an iterator.
    fn events(&mut self) -> EventIterator;
}

impl EventIterator {
    /// Returns true if this event iterator contains no events
    pub fn is_empty(&self) -> bool {
        self.backing.is_empty()
    }

    /// Constructs an `EventIterator` from a `VecDeque`.
    pub fn from_deque(v: VecDeque<Event>) -> EventIterator {
        EventIterator {
            backing: v
        }
    }

    /// Convertes this `EventIterator` back into a `VecDeque`.
    pub fn into_deque(self) -> VecDeque<Event> {
        self.backing
    }

    /// Returns an iterator over the events contained inside without
    /// removing them.
    pub fn as_ref(&self) -> Iter<Event> {
        self.backing.iter()
    }

    /// Returns a mutable iterator over the events contained
    /// inside without removing them.
    pub fn as_mut(&mut self) -> IterMut<Event> {
        self.backing.iter_mut()
    }
}

impl Iterator for EventIterator {
    type Item = Event;
    fn next(&mut self) -> Option<Event> {
        self.backing.pop_front()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.backing.iter().size_hint()
    }
}

impl DoubleEndedIterator for EventIterator {
    fn next_back(&mut self)  -> Option<Event> {
        self.backing.pop_back()
    }
}

/// A conversion trait for representing the different ways that a key
/// can be represented.
pub trait AbstractKey {
    /// Converts an abstract key into a set of concrete key implementations.
    fn to_key(self) -> (Option<u8>, Option<char>, Option<VirtualKeyCode>);
}

impl AbstractKey for u8 {
    fn to_key(self) -> (Option<u8>, Option<char>, Option<VirtualKeyCode>) {
        (Some(self), None, None)
    }
}

impl AbstractKey for char {
    fn to_key(self) -> (Option<u8>, Option<char>, Option<VirtualKeyCode>) {
        (None, Some(self), None)
    }
}

impl AbstractKey for VirtualKeyCode {
    fn to_key(self) -> (Option<u8>, Option<char>, Option<VirtualKeyCode>) {
        (None, None, Some(self))
    }
}

pub fn keycode_to_char(vk: VirtualKeyCode) -> Option<char> {
    Some(match vk {
    Key0 | Numpad0 => '0',
    Key1 | Numpad1 => '1',
    Key2 | Numpad2 => '2',
    Key3 | Numpad3 => '3',
    Key4 | Numpad4 => '4',
    Key5 | Numpad5 => '5',
    Key6 | Numpad6 => '6',
    Key7 | Numpad7 => '7',
    Key8 | Numpad8 => '8',
    Key9 | Numpad9 => '9',
    A => 'a',
    B => 'b',
    C => 'c',
    D => 'd',
    E => 'e',
    F => 'f',
    G => 'g',
    H => 'h',
    I => 'i',
    J => 'j',
    K => 'k',
    L => 'l',
    M => 'm',
    N => 'n',
    O => 'o',
    P => 'p',
    Q => 'q',
    R => 'r',
    S => 's',
    T => 't',
    U => 'u',
    V => 'v',
    W => 'w',
    X => 'x',
    Y => 'y',
    Z => 'z',
    Return => '\n',
    Space => ' ',
    Add => '+',
    Apostrophe => '\'',
    At => '@',
    Backslash => '\\',
    Colon => ':',
    Comma => ',',
    Divide => '/',
    Equals => '=',
    Grave => '`',
    LBracket => '[',
    Minus => '-',
    Multiply => '*',
    NumpadComma => ',',
    NumpadEnter => '\n',
    NumpadEquals => '=',
    Period => '.',
    RBracket => ']',
    Semicolon => ';',
    Subtract => '-',
    Tab => '\t',
    Underline => '_',

    Apps | Ax | Slash => return None,
    _ => return None
    })
}
