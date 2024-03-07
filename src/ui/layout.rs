use std::{ cell::RefCell, rc::Rc };

use crossterm::event::KeyEvent;
use ratatui::{ layout::{ Constraint, Rect }, widgets::Borders, Frame };

use crate::utils::enlarge_rect;

use super::{ line::Line, ui::UI };

pub struct Layout {
    pub lines: Vec<Line>,
    constraints: Vec<Constraint>,
    area: ratatui::layout::Rect,
    pub line_focused_idx: u16,
}

impl<'a> Layout {
    pub fn new(
        lines: Vec<Line>,
        constraints: Vec<Constraint>,
        area: ratatui::layout::Rect
    ) -> Self {
        Layout {
            lines,
            constraints,
            area,
            line_focused_idx: 0,
        }
    }

    pub fn draw(&self, frame: &mut Frame) {
        frame.render_widget(
            ratatui::widgets::Block
                ::default()
                .border_type(ratatui::widgets::BorderType::Rounded)
                .borders(Borders::ALL),
            enlarge_rect(self.area, 1)
        );

        let areas = ratatui::layout::Layout
            ::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints(self.constraints.clone())
            .split(self.area);

        self.lines
            .iter()
            .zip(areas.iter())
            .for_each(|(line, area)| {
                let columns_areas = ratatui::layout::Layout
                    ::default()
                    .direction(ratatui::layout::Direction::Horizontal)
                    .constraints(line.constraints.clone())
                    .split(*area);
                line.widget_holders
                    .iter()
                    .zip(columns_areas.iter())
                    .for_each(|(widget_holder, area)| {
                        let new_area = Rect {
                            x: area.x,
                            y: area.y,
                            width: area.width,
                            height: area.height,
                        };
                        widget_holder.widget.draw(frame, new_area);
                    });
            });
    }

    pub fn update(&mut self, key: KeyEvent, layouts: RefCell<Vec<Layout>>) {
        let mut is_new_layout = false;
        for line in self.lines.iter_mut(). {
            for widget_holder in line.widget_holders.iter_mut() {
                self.pop_widget(i, j);
                widget_holder.widget.update(widget_holder.is_focused, key, layouts);
            }
        }
        is_new_layout
    }

    pub fn pop_widget(&mut self, line_index: usize, widget_index: usize) -> Option<WidgetType> {
        if let Some(line) = self.lines.get_mut(line_index) {
            if let Some(widget_holder) = line.widget_holders.remove(widget_index) {
                return Some(widget_holder.widget);
            }
        }
        None
    }
}
