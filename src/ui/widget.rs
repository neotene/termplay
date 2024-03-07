use std::{ cell::RefCell, rc::Rc };

use crossterm::event::KeyEvent;
use ratatui::Frame;

use super::layout::Layout;

pub trait Widget {
    fn draw(&self, frame: &mut Frame, area: ratatui::layout::Rect);
    fn update(&mut self, focused: bool, key: KeyEvent, layouts: RefCell<Vec<Layout>>) -> bool;
    fn is_focusable(&self) -> bool;
}
