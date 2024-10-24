#[cfg(test)]
mod tests {
    use jiff::Span;
    use crate::config::get_config;

    #[test]
    fn test_spans() {
        let config = get_config();

        let hours = (config.hours_week / 5.).trunc() as i64;
        println!("{}", hours);
        let mins = ((config.hours_week / 5.).fract() * 60.) as i64;
        println!("{}", mins);
        let work_hours: Span = Span::new().hours(hours).minutes(mins);
        dbg!(work_hours);

        let another_span = Span::new().hours(2).minutes(24);

        let added = work_hours.checked_add(another_span).unwrap();
        dbg!(added);
        assert_eq!(added.get_hours(), 10);
        assert_eq!(added.get_minutes(), 5);
    }
}