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
}

