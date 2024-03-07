use ratatui::layout::{ Constraint, Direction, Rect };

use crate::ui::layout::Layout;

pub fn cycle(val: &mut u16, add: i16, max: u16) -> bool {
    if (*val as i16) + add == (max as i16) {
        *val = 0;
        true
    } else if add == -1 && *val == 0 {
        *val = max - 1;
        true
    } else {
        *val = ((*val as i16) + add) as u16;
        false
    }
}

pub fn centered_rect(r: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let popup_layout = ratatui::layout::Layout
        ::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    ratatui::layout::Layout
        ::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

pub fn enlarge_rect(area: Rect, padding: u16) -> Rect {
    Rect {
        x: area.x - padding,
        y: area.y - padding,
        width: area.width + padding * 2,
        height: area.height + padding * 2,
    }
}

pub fn find_next_focusable_widget_holder<'a>(
    current_layout: &Layout,
    mut line_idx: u16,
    mut widget_holder_idx: u16,
    backward: bool
) -> (u16, u16) {
    loop {
        let next_idxs = find_next_widget_holder(
            current_layout,
            line_idx,
            widget_holder_idx,
            backward
        );

        if next_idxs.0 == line_idx && next_idxs.1 == widget_holder_idx {
            return next_idxs;
        }

        if
            current_layout.lines[next_idxs.0 as usize].widget_holders[
                next_idxs.1 as usize
            ].widget.is_focusable()
        {
            return next_idxs;
        }

        line_idx = next_idxs.0;
        widget_holder_idx = next_idxs.1;
    }
}

pub fn find_next_widget_holder<'a>(
    current_layout: &Layout,
    line_idx: u16,
    widget_holder_idx: u16,
    backward: bool
) -> (u16, u16) {
    let mut line = &current_layout.lines[line_idx as usize];
    let mut line_idx_copy = line_idx;

    let mut widget_holder_idx_copy = widget_holder_idx;
    let has_cycled = cycle(
        &mut widget_holder_idx_copy,
        if backward {
            -1
        } else {
            1
        },
        line.widget_holders.len() as u16
    );

    if has_cycled {
        cycle(&mut line_idx_copy, if backward { -1 } else { 1 }, current_layout.lines.len() as u16);
        line = &current_layout.lines[line_idx_copy as usize];
        if backward {
            widget_holder_idx_copy = (line.widget_holders.len() as u16) - 1;
        }
    }

    (line_idx_copy, widget_holder_idx_copy)
}
