use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;
use jiff::{SpanRound, Unit, Zoned};
use serde::{Deserialize, Serialize};
use crate::config::{Config};
use crate::gui::gui_logic::OneDaysWork;
use crate::utils::{compute_hours_and_minutes, compute_should_hours, format_duration};

#[derive(Serialize, Deserialize)]
pub struct Calendar {
    pub work_days: Vec<OneDaysWork>
}

impl Calendar {
    fn to_hashmap(self) -> HashMap<String, OneDaysWork> {
        let mut map = HashMap::<String, OneDaysWork>::new();
        for one_days_work in self.work_days.into_iter() {
            map.insert(one_days_work.date.to_string(), one_days_work);
        }
        map
    }

    pub fn update(cal_map: &HashMap<String, OneDaysWork>) {
        let cal = Calendar {
            work_days: cal_map.values().cloned().collect(),
        };

        let mut path: PathBuf = env::current_exe().unwrap();
        path.set_file_name(".work_data");
        path.set_extension("json");
        let serialized = serde_json::to_string(&cal).unwrap();
        fs::write(path, serialized).unwrap();
    }
}

pub fn init_calendar() -> HashMap<String, OneDaysWork> {
    let mut path: PathBuf = env::current_exe().unwrap();
    path.set_file_name(".work_data");
    path.set_extension("json");

    let today = Zoned::now().date();
    match read_calendar(&path) {
        None => {
            let mut calendar = HashMap::new();
            calendar.insert(today.to_string(), OneDaysWork {
                date: today,
                work_duration: vec![],
                sum_work: None,
                sum_pause: None,
            });
            calendar
        },
        Some(mut calendar) => {
            if let Some(last_work_day) = calendar.work_days.last() {
                if last_work_day.date == today {
                    return calendar.to_hashmap()                    }
            }
            calendar.work_days.push(OneDaysWork {
                date: today,
                work_duration: vec![],
                sum_work: None,
                sum_pause: None,
            });
            calendar.to_hashmap()
        }
    }
}

pub fn read_calendar(path: &PathBuf) -> Option<Calendar> {
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

pub fn export(config: &Config) {

    let path_buf = rfd::FileDialog::new()
        .set_file_name("work_times_export.csv")
        // TODO: anderes default dir setzen
        .set_directory("/Users/jesse/Desktop")
        .save_file();

    if let Some(path_buf) = path_buf {
        let mut path: PathBuf = env::current_exe().unwrap();
        path.set_file_name(".work_data");
        path.set_extension("json");

        let mut write_string = String::from(";;;;;;\n");

        if let Some(work_days) = read_calendar(&path) {
            let mut total_worked: f32 = 0.;
            for work_day in work_days.work_days.into_iter() {
                if let Some(sum_work) = work_day.sum_work {
                    let minutes = sum_work.round(SpanRound::new().largest(Unit::Minute)).unwrap().get_minutes() as f32;
                    total_worked += minutes / 60.;
                }
                write_string = write_string + &serialize_to_csv(work_day, config, total_worked);
            }
        }
        fs::write(path_buf, write_string).unwrap();
    }

}


fn serialize_to_csv(todays_work: OneDaysWork, config: &Config, sum_til_last_day: f32) -> String{

    let should_hours = compute_should_hours(config);

    let date = todays_work.date;
    
    let sum_work = match todays_work.sum_work.as_ref() {
        Some(sum_work) => &format_duration(&sum_work),
        None => ""
    };
    let sum_pause = match todays_work.sum_pause.as_ref() {
        Some(sum_pause) => &format_duration(&sum_pause),
        None => ""
    };
    let contingent = sum_til_last_day - should_hours;
    let (hours, minutes) = compute_hours_and_minutes(contingent);

    let mut write_string = format!("{date};;;;;;\n");
    write_string = write_string + &format!(";SUM WORK;{sum_work};SUM BREAKS;{sum_pause};CONTINGENT;{hours} : {:0>2}\n", minutes.abs());
    write_string = write_string + ";;;;;;\n";
    write_string = write_string + ";START;END;;DURATION;BREAK;\n";
    
    for work_times in &todays_work.work_duration {
        let start = match work_times.start.as_ref() {
            Some(start) => &start.time().round(Unit::Minute).unwrap().to_string()[..5],
            None => ""
        };
        let end = match work_times.end.as_ref() {
            Some(end) => &end.time().round(Unit::Minute).unwrap().to_string()[..5],
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
        write_string = write_string + &format!(";{start};{end};;{duration};{pause};\n")
    }

    write_string = write_string + "\n";
    write_string
}