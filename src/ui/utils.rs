use num_traits::{ FromPrimitive, ToPrimitive };
use ratatui::style::Color;

pub fn cycle<T>(min: T, max: T, current: T, direction: i8) -> T
    where T: FromPrimitive + ToPrimitive + Eq + PartialEq + Clone + Copy
{
    let current = current.to_i8().unwrap();
    if current == min.to_i8().unwrap() && direction == -1 {
        return max;
    }
    if current == max.to_i8().unwrap() && direction == 1 {
        return min;
    }
    T::from_i8(current + direction).unwrap()
}

pub fn calculate_border_color<T>(
    active_section: Option<T>,
    last_hovered_section: T,
    section: T
) -> Color
    where T: FromPrimitive + ToPrimitive + Eq + PartialEq + Clone + Copy
{
    match (active_section, last_hovered_section) {
        (Some(active_section), _) if active_section.eq(&section) => Color::Yellow,
        (_, last_hovered_section) if last_hovered_section.eq(&section) => Color::Blue,
        _ => Color::Reset,
    }
}
