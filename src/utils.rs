use iced_aw::date_picker::Date;
use jiff::{Span, Zoned};
use jiff::civil::{date, Weekday};
use crate::config::Config;

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

pub fn compute_should_hours(config: &Config) -> f32 {
    // TODO: only able to start on a monday -> other wise the the weeks would not be correct
    let hours_week = config.hours_week;
    let start_day = config.start_date;

    let today = Zoned::now().date();
    let work_span = today.since(start_day).unwrap();
    let work_days = work_span.get_days() as f32;
    let full_weeks = (work_days / 7.0).trunc();
    let days_this_week = work_days % 7. + 1.;

    hours_week * full_weeks + days_this_week * (hours_week / 5.)
}

pub fn jiff_date_from_picker(picker_date: Date) -> jiff::civil::Date {
    date(picker_date.year as i16, picker_date.month as i8, picker_date.day as i8)
}


pub fn get_num_workdays(from: jiff::civil::Date, to: jiff::civil::Date) -> f32 {
    let work_span = from.until(to).unwrap();
    let work_days = work_span.get_days() as f32;
    
    if work_days < 5. {
        match from.weekday() {
            Weekday::Saturday => work_days,
            Weekday::Sunday => work_days - 1.,
            _ => work_days + 1.
        }
    } else {
        let first_week_workdays = num_workdays_first_week(from.weekday());
        let this_week_workdays = num_workdays_this_week_workdays(to.weekday());
        let full_weeks = ((work_days / 7.).trunc() - 1.).max(0.);         // -2 because without first week and this week

        first_week_workdays + full_weeks * 5. + this_week_workdays
    }
}

fn num_workdays_first_week(workday: Weekday) -> f32 {
    match workday {
        Weekday::Monday => 5.,
        Weekday::Tuesday => 4.,
        Weekday::Wednesday => 3.,
        Weekday::Thursday => 2.,
        Weekday::Friday => 1.,
        _ => 0.
    }
}

fn num_workdays_this_week_workdays(workday: Weekday) -> f32 {
    match workday {
        Weekday::Monday => 1.,
        Weekday::Tuesday => 2.,
        Weekday::Wednesday => 3.,
        Weekday::Thursday => 4.,
        Weekday::Friday => 5.,
        _ => 5.
    }
}