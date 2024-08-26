use std::env;
use std::fs;
use std::path::PathBuf;
use jiff::Unit;
use serde::{Deserialize, Serialize};

use crate::gui::gui_logic::OneDaysWork;
use crate::utils::format_duration;

#[derive(Serialize, Deserialize)]
pub struct WorkDays {
    pub states: Vec<OneDaysWork>
}


pub fn update_work_data(work_day: OneDaysWork) {
    let mut path: PathBuf = env::current_exe().unwrap();
    path.set_file_name(".work_data");
    path.set_extension("json");

    match read_work_data(&path) {
        None => {
            let work_days = WorkDays {
                states: vec!(work_day)
            };
            write_work_data(work_days, &path);
        },
        Some(mut work_days) => {
            if let Some(last_work_day) = work_days.states.last() {
                if let Some(last_work_day_date) = &last_work_day.date {
                    if let Some(today) = &work_day.date {
                        if last_work_day_date.date() == today.date() {
                            let idx_last = work_days.states.len()-1;
                            work_days.states[idx_last] = work_day;
                            write_work_data(work_days, &path)
                        } else {
                            work_days.states.push(work_day);
                            write_work_data(work_days, &path)
                        }
                    }
                }
            }
        }
    }
}


pub fn read_work_data(path: &PathBuf) -> Option<WorkDays> {
    match fs::read_to_string(&path) {
        Err(error) => {
            print!("{}", error);
            None
        }
        Ok(file_content) => {
            Some(serde_json::from_str(&file_content).expect("Could not deserialize .work_data.json"))
        }
    }   
}



pub fn write_work_data(work_days: WorkDays, path: &PathBuf) {
    let serialized = serde_json::to_string(&work_days).unwrap();
    fs::write(path, serialized).unwrap();
}


pub fn export() {
    let mut path: PathBuf = env::current_exe().unwrap();
    path.set_file_name(".work_data");
    path.set_extension("json");

    let mut write_string = String::from(";;;;;;;\n");

    if let Some(work_days) = read_work_data(&path) {
        for work_day in work_days.states {
            write_string = write_string + &serialize_to_csv(work_day);
        }
    }

    fs::write("/Users/jessekruse/Desktop/work_times.csv", write_string).unwrap();
}


fn serialize_to_csv(todays_work: OneDaysWork) -> String{
    let sum_til_last_day = todays_work.sum_til_last_day;
    let should_hours = todays_work.should_hours;

    let date = todays_work.date.as_ref().unwrap().date();
    
    let sum_work = match todays_work.sum_work.as_ref() {
        Some(sum_work) => &format_duration(&sum_work),
        None => ""
    };
    let sum_pause = match todays_work.sum_pause.as_ref() {
        Some(sum_pause) => &format_duration(&sum_pause),
        None => ""
    };
    let contingent = sum_til_last_day-should_hours;

    let mut write_string = format!("{date};;;;;;;\n");
    write_string = write_string + &format!("SUM WORK;{sum_work};;SUM BREAKS;{sum_pause};;CONTINGENT;{contingent}\n");
    write_string = write_string + ";;;;;;;\n";
    write_string = write_string + "START;END;DURATION;BREAK;;;;\n";
    
    for work_times in &todays_work.work_duration {
        let start = match work_times.start.as_ref() {
            Some(start) => &start.time().round(Unit::Second).unwrap().to_string(),
            None => ""
        };
        let end = match work_times.end.as_ref() {
            Some(end) => &end.time().round(Unit::Second).unwrap().to_string(),
            None => ""
        };
        let duration = match work_times.duration.as_ref() {
            Some(duration) => &format_duration(&duration),
            None => ""
        };
        let pause = match work_times.pause.as_ref() {
            Some(pause) => &format_duration(&pause),
            None => ""
        };
        write_string = write_string + &format!("{start};{end};{duration};{pause};;;;\n")
    }

    write_string = write_string + "\n";
    write_string
}