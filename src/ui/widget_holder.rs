use super::widget::Widget;

pub struct WidgetHolder {
    // constraint: ratatui::layout::Constraint,
    pub widget: Box<dyn Widget>,
    pub is_focused: bool,
}

impl<'a> WidgetHolder {
    pub fn new(widget: Box<dyn Widget>) -> Self {
        WidgetHolder {
            widget,
            is_focused: false,
        }
    }
}
