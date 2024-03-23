use crossterm::event::{ KeyCode, KeyEvent, KeyEventKind };
use ratatui::{
    layout::Alignment,
    style::{ Color, Style },
    widgets::{ Block, BorderType, Borders, Paragraph },
    Frame,
};

use super::{ layout::Layout, ui::LayoutsRef, widget::Widget };

pub type Callback = Box<dyn Fn(&mut Layout, LayoutsRef) -> Result<bool, std::io::Error>>;
pub struct Button<'a> {
    paragraph: Paragraph<'a>,
    title: String,
    action: Callback,
}

pub fn get_default_paragraph<'a>(title: String) -> Paragraph<'a> {
    Paragraph::new(title)
        .block(
            Block::new()
                .border_type(BorderType::Rounded)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Gray))
        )
        .alignment(Alignment::Center)
}

impl<'a> Widget for Button<'a> {
    fn draw(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
        frame.render_widget(self.paragraph.clone(), area);
    }

    fn update(
        &mut self,
        focused: bool,
        key: KeyEvent,
        layout: &mut Layout,
        layouts: LayoutsRef
    ) -> bool {
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
                            let _ = (self.action)(layout, layouts);
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

    // fn clone(&self) -> Box<dyn Widget> {
    //     Box::new(Button {
    //         paragraph: self.paragraph.clone(),
    //         title: self.title.clone(),
    //         action: self.action,
    //     })
    // }
}

impl<'a> Button<'a> {
    pub fn new(title: String, action: Callback) -> Self {
        Button {
            paragraph: get_default_paragraph(title.clone()).alignment(Alignment::Center),
            title: title.clone(),
            action: action,
        }
    }
}
