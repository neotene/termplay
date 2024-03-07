use ratatui::layout::Constraint;

use super::widget_holder::WidgetHolder;

pub struct Line {
    pub widget_holders: Vec<WidgetHolder>,
    pub constraints: Vec<Constraint>,
    pub widget_holder_focused_idx: u16,
}

impl Line {
    pub fn new(widget_holders: Vec<WidgetHolder>, constraints: Vec<Constraint>) -> Self {
        Line {
            widget_holders,
            constraints,
            widget_holder_focused_idx: 0,
        }
    }
}
