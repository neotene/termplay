use crossterm::event::KeyEvent;
use ratatui::Frame;

use super::{ layout::Layout, ui::LayoutsRef };

pub trait Widget {
    fn draw(&self, frame: &mut Frame, area: ratatui::layout::Rect);
    fn update(
        &mut self,
        focused: bool,
        key: KeyEvent,
        layout: &mut Layout,
        layouts: LayoutsRef
    ) -> bool;
    fn is_focusable(&self) -> bool;
    // fn clone(&self) -> Box<dyn Widget>;
}
