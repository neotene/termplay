use crossterm::event::KeyEvent;
use ratatui::{ layout::{ Constraint, Rect }, widgets::Borders, Frame };

use crate::utils::enlarge_rect;

use super::{ line::Line, ui::LayoutsRef };

// #[derive(Clone)]
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

    pub fn update(&mut self, key: KeyEvent, layouts: LayoutsRef) {
        if self.lines.len() == 0 {
            return;
        }
        for i in 0..self.lines.len() {
            if self.lines[i].widget_holders.len() == 0 {
                continue;
            }
            for j in 0..self.lines[i].widget_holders.len() {
                let mut save = self.lines[i].widget_holders.remove(j);
                save.widget.update(save.is_focused, key, self, layouts);
                self.lines[i].widget_holders.insert(j, save);
            }
        }
    }
}
