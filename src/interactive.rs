use std::vec::IntoIter;
use self::keycodes::*;

pub mod keycodes {
    pub use glutin::VirtualKeyCode;
    pub use glutin::VirtualKeyCode::{Key1, Key2, Key3, Key4, Key5, Key6, Key7, Key8, Key9, Key0, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, Escape, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, Snapshot, Scroll, Pause, Insert, Home, Delete, End, PageDown, PageUp, Left, Up, Right, Down, Back, Return, Space, Numlock, Numpad0, Numpad1, Numpad2, Numpad3, Numpad4, Numpad5, Numpad6, Numpad7, Numpad8, Numpad9, AbntC1, AbntC2, Add, Apostrophe, Apps, At, Ax, Backslash, Calculator, Capital, Colon, Comma, Convert, Decimal, Divide, Equals, Grave, Kana, Kanji, LAlt, LBracket, LControl, LMenu, LShift, LWin, Mail, MediaSelect, MediaStop, Minus, Multiply, Mute, MyComputer, NextTrack, NoConvert, NumpadComma, NumpadEnter, NumpadEquals, OEM102, Period, Playpause, Power, Prevtrack, RAlt, RBracket, RControl, RMenu, RShift, RWin, Semicolon, Slash, Sleep, Stop, Subtract, Sysrq, Tab, Underline, Unlabeled, VolumeDown, VolumeUp, Wake, Webback, WebFavorites, WebForward, WebHome, WebRefresh, WebSearch, WebStop, Yen };
}

/// An even coming from an Interactive object.
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum Event {
    /// The mouse moved to this position.
    MouseMoved((i32, i32)),
    /// The mouse wheel moved by this delta.
    MouseWheel(i32),
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
    WindowMoved((i32, i32))
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
    OtherMouseButton(u8)
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

    fn was_open(&self) -> bool;

    /// Returns the title of the object.
    fn title(&self) -> &str;

    /// Sets the title of the object.  If the object is a window,
    /// this title will be used to decorate the window.
    fn set_title(&mut self, title: &str);

    /// Sets the size of the window if possible.
    fn set_size(&mut self, width: u32, height: u32);

    /// Returns the size of the window.
    fn get_size(&self) -> (u32, u32);

    // Events

    /// Returns true if the operating system has given this object focus.
    fn is_focused(&self) -> bool;

    /// Returns true if any mouse button is down.
    fn mouse_down(&self) -> bool;

    /// Returns the current position of the mouse.
    ///
    /// This function returns the position in floating point units
    /// for usability.  Use `mouse_pos_int` if you want integer units.
    fn mouse_pos(&self) -> (f32, f32);

    /// Returns the current position of the mouse in integer units.
    fn mouse_pos_int(&self) -> (i32, i32);

    /// Returns the x coordinate of the mouse.
    fn mouse_x(&self) -> f32 {
        match self.mouse_pos() {
            (x, _) => x
        }
    }

    /// Returns the y coordinate of the mouse.
    fn mouse_y(&self) -> f32 {
        match self.mouse_pos() {
            (_, y) => y
        }
    }



    /// Returns true if a given key is currently being pressed.
    fn is_key_pressed<K: AbstractKey>(&self, k: K) -> bool;

    /// Consumes all unseen events and returns them in an iterator.
    fn events(&mut self) -> IntoIter<Event>;
}

/// A conversion trait for representing the different ways that a key
/// can be represented.
pub trait AbstractKey {
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
