use jiff::Span;

pub fn format_duration(span: &Span) -> String {
    let hours = span.get_hours().to_string();
    let minutes = span.get_minutes().to_string();
    let seconds = span.get_seconds().to_string();

    format!("{hours}:{minutes}:{seconds}")
}