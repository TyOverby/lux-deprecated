use std::vec::MoveItems;
use self::keycodes::*;

pub mod keycodes {
   pub use glutin::{VirtualKeyCode, Key1, Key2, Key3, Key4, Key5, Key6, Key7, Key8, Key9, Key0, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, Escape, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, Snapshot, Scroll, Pause, Insert, Home, Delete, End, PageDown, PageUp, Left, Up, Right, Down, Back, Return, Space, Numlock, Numpad0, Numpad1, Numpad2, Numpad3, Numpad4, Numpad5, Numpad6, Numpad7, Numpad8, Numpad9, AbntC1, AbntC2, Add, Apostrophe, Apps, At, Ax, Backslash, Calculator, Capital, Colon, Comma, Convert, Decimal, Divide, Equals, Grave, Kana, Kanji, LAlt, LBracket, LControl, LMenu, LShift, LWin, Mail, MediaSelect, MediaStop, Minus, Multiply, Mute, MyComputer, NextTrack, NoConvert, NumpadComma, NumpadEnter, NumpadEquals, OEM102, Period, Playpause, Power, Prevtrack, RAlt, RBracket, RControl, RMenu, RShift, RWin, Semicolon, Slash, Sleep, Stop, Subtract, Sysrq, Tab, Underline, Unlabeled, VolumeDown, VolumeUp, Wake, Webback, WebFavorites, WebForward, WebHome, WebRefresh, WebSearch, WebStop, Yen };
}

#[deriving(Show, Eq, PartialEq, Hash)]
pub enum LuxEvent {
    MouseMoved((i32, i32)),
    MouseWheel(i32),
    MouseDown(MouseButton),
    MouseUp(MouseButton),
    KeyPressed(u8, Option<char>, Option<keycodes::VirtualKeyCode>),
    KeyReleased(u8, Option<char>, Option<keycodes::VirtualKeyCode>),
    WindowResized((u32, u32)),
    WindowMoved((i32, i32))
}

#[deriving(Show, Eq, PartialEq, Hash)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    OtherMouseButton(u8)
}

pub trait LuxWindow {
    fn is_open(&self) -> bool;
    fn title(&self) -> &str;
    fn set_title(&mut self, title: &str);
    fn set_size(&mut self, width: u32, height: u32);
    fn get_size(&self) -> (u32, u32);

    // Events
    fn is_focused(&self) -> bool;
    fn mouse_down(&self) -> bool;
    fn mouse_pos(&self) -> (i32, i32);
    fn mouse_x(&self) -> i32 {
        match self.mouse_pos() {
            (x, _) => x
        }
    }
    fn mouse_y(&self) -> i32 {
        match self.mouse_pos() {
            (_, y) => y
        }
    }

    fn events(&mut self) -> MoveItems<LuxEvent>;
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
