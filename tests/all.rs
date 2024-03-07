#[cfg(test)]
mod tests {
    use ratatui::layout::{ Constraint, Rect };
    use termplay::{
        ui::{ self, label::Label, layout::Layout, line::Line, widget_holder::WidgetHolder },
        utils::{ find_next_focusable_widget_holder, find_next_widget_holder },
    };

    // use super::*;
    #[test]
    fn test_next_widget_holder_one_widget() {
        let widget = Label::default();
        let lay = Layout::new(
            vec![
                Line::new(
                    vec![WidgetHolder::new(Box::new(widget))],
                    vec![Constraint::Percentage(100)]
                )
            ],
            vec![Constraint::Min(3)],
            Rect::new(0, 0, 10, 10)
        );

        let idxs = find_next_widget_holder(&lay, 0, 0, true);
        assert_eq!(idxs, (0, 0));
    }

    #[test]
    fn test_next_widget_holder_two_widgets() {
        let widget = ui::label::Label::default();
        let widget2 = ui::field::Field::new(
            String::from("Email"),
            String::from("Please enter your email address"),
            false
        );
        let lay = Layout::new(
            vec![
                Line::new(
                    vec![WidgetHolder::new(Box::new(widget)), WidgetHolder::new(Box::new(widget2))],
                    vec![Constraint::Percentage(50), Constraint::Percentage(50)]
                )
            ],
            vec![Constraint::Min(3)],
            Rect::new(0, 0, 10, 10)
        );

        let idxs = find_next_widget_holder(&lay, 0, 0, true);
        assert_eq!(idxs, (0, 1));
    }

    #[test]
    fn test_next_widget_holder_four_widgets() {
        let widget1 = ui::label::Label::default();
        let widget2 = ui::field::Field::new(
            String::from("Email"),
            String::from("Please enter your email address"),
            false
        );
        let widget3 = ui::field::Field::new(
            String::from("Password"),
            String::from("Please enter your password"),
            true
        );
        let widget4 = ui::field::Field::new(
            String::from("Password"),
            String::from("Please enter your password"),
            true
        );
        let lay = Layout::new(
            vec![
                Line::new(
                    vec![
                        WidgetHolder::new(Box::new(widget1)),
                        WidgetHolder::new(Box::new(widget2))
                    ],
                    vec![Constraint::Percentage(50), Constraint::Percentage(50)]
                ),
                Line::new(
                    vec![
                        WidgetHolder::new(Box::new(widget3)),
                        WidgetHolder::new(Box::new(widget4))
                    ],
                    vec![Constraint::Percentage(50), Constraint::Percentage(50)]
                )
            ],
            vec![Constraint::Min(3)],
            Rect::new(0, 0, 10, 10)
        );

        let mut idxs = find_next_widget_holder(&lay, 0, 0, false);
        assert_eq!(idxs, (0, 1));
        idxs = find_next_widget_holder(&lay, 0, 0, true);
        assert_eq!(idxs, (1, 1));
        idxs = find_next_widget_holder(&lay, 1, 1, true);
        assert_eq!(idxs, (1, 0));
        idxs = find_next_widget_holder(&lay, 1, 0, true);
        assert_eq!(idxs, (0, 1));
        idxs = find_next_widget_holder(&lay, 0, 1, true);
        assert_eq!(idxs, (0, 0));
    }

    #[test]
    fn test_next_widget_holder_four_focusable() {
        let widget1 = ui::field::Field::new(
            String::from("Email"),
            String::from("Please enter your email address"),
            false
        );
        let widget2 = ui::field::Field::new(
            String::from("Password"),
            String::from("Please enter your password"),
            true
        );
        let widget3 = ui::field::Field::new(
            String::from("Password"),
            String::from("Please enter your password"),
            true
        );
        let widget4 = ui::field::Field::new(
            String::from("Password"),
            String::from("Please enter your password"),
            true
        );

        let lay = Layout::new(
            vec![
                Line::new(
                    vec![WidgetHolder::new(Box::new(widget1))],
                    vec![Constraint::Percentage(100)]
                ),
                Line::new(
                    vec![WidgetHolder::new(Box::new(widget2))],
                    vec![Constraint::Percentage(100)]
                ),
                Line::new(
                    vec![
                        WidgetHolder::new(Box::new(widget3)),
                        WidgetHolder::new(Box::new(widget4))
                    ],
                    vec![Constraint::Percentage(50), Constraint::Percentage(50)]
                )
            ],
            vec![Constraint::Min(3), Constraint::Min(3), Constraint::Min(3)],
            Rect::new(0, 0, 10, 10)
        );

        let idxs = find_next_widget_holder(&lay, 2, 0, true);
        assert_eq!(idxs, (1, 0));
    }

    #[test]
    fn test_next_focusable_widget_holder() {
        let widget1 = ui::label::Label::default();
        let widget2 = ui::field::Field::new(
            String::from("Email"),
            String::from("Please enter your email address"),
            false
        );
        let widget3 = ui::label::Label::default();
        let widget4 = ui::field::Field::new(
            String::from("Password"),
            String::from("Please enter your password"),
            true
        );
        let lay = Layout::new(
            vec![
                Line::new(
                    vec![
                        WidgetHolder::new(Box::new(widget1)),
                        WidgetHolder::new(Box::new(widget2))
                    ],
                    vec![Constraint::Percentage(50), Constraint::Percentage(50)]
                ),
                Line::new(
                    vec![
                        WidgetHolder::new(Box::new(widget3)),
                        WidgetHolder::new(Box::new(widget4))
                    ],
                    vec![Constraint::Percentage(50), Constraint::Percentage(50)]
                )
            ],
            vec![Constraint::Min(3)],
            Rect::new(0, 0, 10, 10)
        );

        let mut idxs = find_next_focusable_widget_holder(&lay, 0, 0, false);
        assert_eq!(idxs, (0, 1));
        idxs = find_next_focusable_widget_holder(&lay, 0, 1, false);
        assert_eq!(idxs, (1, 1));
        idxs = find_next_focusable_widget_holder(&lay, 1, 1, false);
        assert_eq!(idxs, (0, 1));
        idxs = find_next_focusable_widget_holder(&lay, 0, 1, true);
        assert_eq!(idxs, (1, 1));
        idxs = find_next_focusable_widget_holder(&lay, 1, 1, true);
        assert_eq!(idxs, (0, 1));
    }

    #[test]
    fn test_next_focusable_widget_holder_four_focusable() {
        let widget1 = ui::field::Field::new(
            String::from("Email"),
            String::from("Please enter your email address"),
            false
        );
        let widget2 = ui::field::Field::new(
            String::from("Password"),
            String::from("Please enter your password"),
            true
        );
        let widget3 = ui::field::Field::new(
            String::from("Password"),
            String::from("Please enter your password"),
            true
        );
        let widget4 = ui::field::Field::new(
            String::from("Password"),
            String::from("Please enter your password"),
            true
        );

        let lay = Layout::new(
            vec![
                Line::new(
                    vec![WidgetHolder::new(Box::new(widget1))],
                    vec![Constraint::Percentage(100)]
                ),
                Line::new(
                    vec![WidgetHolder::new(Box::new(widget2))],
                    vec![Constraint::Percentage(100)]
                ),
                Line::new(
                    vec![
                        WidgetHolder::new(Box::new(widget3)),
                        WidgetHolder::new(Box::new(widget4))
                    ],
                    vec![Constraint::Percentage(50), Constraint::Percentage(50)]
                )
            ],
            vec![Constraint::Min(3), Constraint::Min(3), Constraint::Min(3)],
            Rect::new(0, 0, 10, 10)
        );

        let idxs = find_next_focusable_widget_holder(&lay, 2, 0, true);
        assert_eq!(idxs, (1, 0));
    }
}
