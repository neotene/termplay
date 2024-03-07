use std::{ cell::RefCell, rc::Rc };

use crossterm::event::{ KeyCode, KeyEvent, KeyEventKind };
use ratatui::{ style::{ Color, Style }, widgets::{ Block, BorderType, Borders, Paragraph }, Frame };

use super::{ layout::Layout, ui::UI, widget::Widget };

pub struct Button<'a> {
    paragraph: Paragraph<'a>,
    title: String,
    action: fn(),
}

pub fn get_default_paragraph<'a>(title: String) -> Paragraph<'a> {
    Paragraph::new(title).block(
        Block::new()
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Gray))
    )
}

impl<'a> Widget for Button<'a> {
    fn draw(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
        frame.render_widget(self.paragraph.clone(), area);
    }

    fn update(&mut self, focused: bool, key: KeyEvent, layouts: RefCell<Vec<Layout>>) -> bool {
        if focused {
            self.paragraph = get_default_paragraph(self.title.clone()).block(
                Block::new()
                    .border_type(BorderType::Rounded)
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Blue))
            );

            match key.code {
                KeyCode::Enter => {
                    match key.kind {
                        KeyEventKind::Press => {
                            (self.action)();
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        } else {
            self.paragraph = get_default_paragraph(self.title.clone()).block(
                Block::new()
                    .border_type(BorderType::Rounded)
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Gray))
            );
        }
        false
    }

    fn is_focusable(&self) -> bool {
        true
    }
}

impl<'a> Button<'a> {
    pub fn new(title: String, action: fn()) -> Self {
        Button {
            paragraph: get_default_paragraph(title.clone()),
            title: title.clone(),
            action: action,
        }
    }
}
