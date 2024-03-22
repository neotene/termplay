use crossterm::event::{ KeyCode, KeyEvent, KeyEventKind };
use ratatui::Frame;

use crate::utils::find_next_focusable_widget_holder;

use super::layout::Layout;

type Layouts = Vec<Layout>;
pub type LayoutsRef<'a> = &'a mut Layouts;

pub struct UI {
    layouts: Layouts,
    current_layout_idx: u16,
}

impl UI {
    pub fn new(layouts: Vec<Layout>) -> Self {
        let idx: i16;
        if layouts.len() == 0 {
            idx = 0;
        } else {
            idx = (layouts.len() as i16) - 1;
        }
        let var = UI {
            layouts: layouts,
            current_layout_idx: idx as u16,
        };
        var
    }

    pub fn draw(&self, frame: &mut Frame) {
        self.layouts.iter().for_each(|layer| {
            layer.draw(frame);
        });
    }

    pub fn update(&mut self, key: KeyEvent) {
        let mut val: i16 = 0;
        match key.code {
            KeyCode::Tab => {
                val += 1;
            }
            KeyCode::BackTab => {
                val -= 1;
            }
            _ => {}
        }

        let mut current_layout = self.layouts.remove(self.current_layout_idx as usize);

        if val != 0 && key.kind == KeyEventKind::Press {
            let idxs = find_next_focusable_widget_holder(
                &current_layout,
                current_layout.line_focused_idx,
                current_layout.lines
                    [current_layout.line_focused_idx as usize].widget_holder_focused_idx,
                val < 0
            );
            let prev_widget_holder_idx = current_layout.lines
                [current_layout.line_focused_idx as usize].widget_holder_focused_idx;

            current_layout.lines[current_layout.line_focused_idx as usize].widget_holders[
                prev_widget_holder_idx as usize
            ].is_focused = false;

            current_layout.line_focused_idx = idxs.0;
            current_layout.lines[idxs.0 as usize].widget_holder_focused_idx = idxs.1;

            current_layout.lines[idxs.0 as usize].widget_holders[idxs.1 as usize].is_focused = true;
        }

        current_layout.update(key, &mut self.layouts);
        self.layouts.insert(self.current_layout_idx as usize, current_layout);
    }
}
