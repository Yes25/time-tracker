#[cfg(test)]
mod tests {
    use jiff::Span;
    use jiff::civil::Date;
    use crate::config::Config;
    use crate::utils::{get_num_workdays};

    #[test]
    fn test_spans() {
        let config = Config::get_config();

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

    #[test]
    fn test_get_work_days() {
        let today = Date::new(2024, 11, 01).unwrap();

        let start_day =  Date::new(2024, 10, 11).unwrap();
        assert_eq!(get_num_workdays(start_day, today), 16.);

        let start_day_2 =  Date::new(2024, 10, 24).unwrap();
        assert_eq!(get_num_workdays(start_day_2, today), 7.);

        let from =  Date::new(2024, 11, 1).unwrap();
        let to =  Date::new(2024, 11, 10).unwrap();
        assert_eq!(get_num_workdays(from, to), 6.);

        let from =  Date::new(2024, 11, 4).unwrap();
        let to =  Date::new(2024, 11, 6).unwrap();
        assert_eq!(get_num_workdays(from, to), 3.);
    }
}