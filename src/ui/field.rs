use crossterm::event::{ KeyCode, KeyEvent };
use ratatui::{ style::{ Color, Style, Stylize }, widgets::{ Block, BorderType, Borders }, Frame };
use tui_textarea::TextArea;

use super::{ layout::Layout, ui::LayoutsRef, widget::Widget };

pub struct Field<'a> {
    textarea: TextArea<'a>,
    title: String,
}

impl Widget for Field<'_> {
    fn draw(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
        frame.render_widget(self.textarea.widget(), area);
    }

    fn update(
        &mut self,
        focused: bool,
        key: KeyEvent,
        _layout: &mut Layout,
        _layouts: LayoutsRef
    ) -> bool {
        if focused {
            self.textarea.set_block(
                Block::new()
                    .border_type(BorderType::Rounded)
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Blue))
                    .title(self.title.clone())
            );

            match key.code {
                KeyCode::Backspace => {
                    self.textarea.input(key);
                }
                KeyCode::Char(_c) => {
                    self.textarea.input(key);
                }
                _ => {}
            }
        } else {
            self.textarea.set_block(
                Block::new()
                    .border_type(BorderType::Rounded)
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Gray))
                    .borders(Borders::ALL)
                    .title(self.title.clone())
            );
        }
        false
    }

    fn is_focusable(&self) -> bool {
        true
    }

    // fn clone(&self) -> Box<dyn Widget> {
    //     Box::new(Field {
    //         textarea: self.textarea.clone(),
    //         title: self.title.clone(),
    //     })
    // }
}

impl<'a> Field<'a> {
    pub fn new(title: String, placeholder: String, is_password: bool) -> Self {
        let mut field = Field {
            textarea: TextArea::default(),
            title,
        };

        field.textarea.set_placeholder_text(placeholder.clone());
        field.textarea.set_block(
            ratatui::widgets::Block::default().title(field.title.clone()).borders(Borders::ALL)
        );
        if is_password {
            field.textarea.set_mask_char('*');
        }

        field.textarea.set_cursor_line_style(Style::default().not_underlined());

        field
    }
}
