use crossterm::event::KeyEvent;
use ratatui::{ widgets::Paragraph, Frame };

use super::{ layout::Layout, ui::LayoutsRef, widget::Widget };

#[derive(Default)]
pub struct Label {
    text: String,
    style: ratatui::style::Style,
}

impl Widget for Label {
    fn draw(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
        frame.render_widget(Paragraph::new(self.text.clone()).style(self.style), area);
    }

    fn update(
        &mut self,
        _focused: bool,
        _key: KeyEvent,
        layout: &mut Layout,
        _layouts: LayoutsRef
    ) -> bool {
        // Do nothing
        false
    }

    fn is_focusable(&self) -> bool {
        false
    }
}

// impl Default for Label {
//     fn default() -> Self {
//         Label {
//             text: String::from(""),
//             style: Style::default(),
//         }
//     }
// }
