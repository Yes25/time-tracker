use jiff::Span;

pub fn format_duration(span: &Span) -> String {
    let hours = span.get_hours().to_string();
    let minutes = span.get_minutes().to_string();
    format!("{hours}:{minutes}")
}

// returns hours and minutes from a fractional hour value, like 2.7h
pub fn compute_hours_and_minutes(input_hours: f32) -> (i32, i32) {
    let hours = input_hours.trunc() as i32;
    let fraction = input_hours.fract();
    let minutes = (fraction * 60.0) as i32;

    (hours, minutes)
}