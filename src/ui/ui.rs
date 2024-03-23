use crossterm::event::{ self, KeyCode, KeyEvent, KeyEventKind };
use ratatui::Frame;

use crate::utils::find_next_focusable_widget_holder;

use super::layout::Layout;

type Layouts = Vec<Layout>;
pub type LayoutsRef<'a> = &'a mut Layouts;

pub struct UI {
    layouts: Layouts,
    current_layout_idx: u16,
    tick_duration: std::time::Duration,
    last_tick: std::time::Instant,
}

impl UI {
    pub fn new(layouts: Vec<Layout>) -> Self {
        let idx: i16;
        if layouts.len() == 0 {
            idx = 0;
        } else {
            idx = (layouts.len() as i16) - 1;
        }
        UI {
            layouts: layouts,
            current_layout_idx: idx as u16,
            tick_duration: std::time::Duration::from_millis(100),
            last_tick: std::time::Instant::now(),
        }
    }

    pub fn draw(&self, frame: &mut Frame) {
        self.layouts.iter().for_each(|layer| {
            layer.draw(frame);
        });
    }

    pub fn update(&mut self) -> bool {
        self.current_layout_idx = (self.layouts.len() as u16) - 1;

        let mut key = KeyEvent::from(KeyCode::Null);
        if let Ok(true) = event::poll(std::time::Duration::from_millis(16)) {
            if let Ok(event) = event::read() {
                match event {
                    event::Event::Key(key_event) => {
                        key = key_event.clone();
                        if
                            key_event.kind == KeyEventKind::Press &&
                            key_event.modifiers == event::KeyModifiers::CONTROL &&
                            key_event.code == KeyCode::Char('c')
                        {
                            return false;
                        }
                    }
                    _ => (),
                }
            }
        }

        let now = std::time::Instant::now();
        // if now.duration_since(self.last_tick) < self.tick_duration {
        //     return true;
        // }
        self.last_tick = now;

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
        true
    }
}
