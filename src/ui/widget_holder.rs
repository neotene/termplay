use super::widget::Widget;

pub type WidgetRef = Box<dyn Widget>;

pub struct WidgetHolder {
    // constraint: ratatui::layout::Constraint,
    pub widget: WidgetRef,
    pub is_focused: bool,
}

impl<'a> WidgetHolder {
    pub fn new(widget: WidgetRef) -> Self {
        WidgetHolder {
            widget,
            is_focused: false,
        }
    }
}

// impl Clone for WidgetHolder {
//     fn clone(&self) -> Self {
//         WidgetHolder {
//             widget: self.widget.clone(),
//             is_focused: self.is_focused,
//         }
//     }
// }
