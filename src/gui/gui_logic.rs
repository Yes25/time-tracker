use std::env;
use std::path::PathBuf;

use jiff::{Span, Zoned};
use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::gui::serialize::read_work_data;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WorkTimes {
    pub label: Option<String>,
    pub start: Option<Zoned>,
    pub end: Option<Zoned>,
    pub duration: Option<Span>,
    pub pause: Option<Span>
}

impl WorkTimes {
    pub fn init() -> WorkTimes {
        WorkTimes {
            label: None,
            start: None,
            end: None,
            duration: None,
            pause: None,
        }
    }

    pub fn set_start(&mut self) {
        self.start = Some(Zoned::now());
    }

    pub fn set_end(&mut self) {
        self.end = Some(Zoned::now());
    }

    pub fn set_label(&mut self, label: &str) {
        self.label = Some(label.to_owned());
    }

    pub fn build_new_work_times()  -> WorkTimes {
        let mut work_times = WorkTimes::init();
        work_times.set_start();
        work_times.set_label("Work");

        work_times
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OneDaysWork {
    pub sum_total: f32,
    pub should_hours: f32,
    pub date: Option<Zoned>,
    pub work_duration: Vec<WorkTimes>,
    pub sum_work: Option<Span>,
    pub sum_pause: Option<Span>,
}

impl OneDaysWork {

    pub fn init(config: &Config) -> OneDaysWork {
        
        let mut path: PathBuf = env::current_exe().unwrap();
        path.set_file_name(".work_data");
        path.set_extension("json");

        let today = Zoned::now().date();
        let mut sum_total = 0.;

        if let Some(work_days) =  read_work_data(&path) {
            if let Some(last_work_day) = work_days.states.last() {
                sum_total = last_work_day.sum_total;
                if let Some(date) = &last_work_day.date {
                    if date.date() == today {
                        return last_work_day.to_owned();
                    } 
                }
            };
            
        }
        
        let hours_week = config.hours_week;
        let start_day = config.start_date;

        let work_span = today.since(start_day).unwrap();
        let work_days = (work_span.get_days() + 1) as f32 ;     // + 1 because today is not elapsed, hence it os not in get_days. But we want to know how much I should have worked at the end of today 
        let should_hours = work_days * (hours_week / 5.);

        OneDaysWork {
            sum_total,
            should_hours: should_hours,
            date: None,
            work_duration: vec![],
            sum_work: None,
            sum_pause: None, 
        }
    }   
    

    pub fn set_date(&mut self) {
        self.date = Some(Zoned::now())
    }

    pub fn start(&mut self) {
        if self.work_duration.len() == 0 {
            self.set_date();
            let work_times = WorkTimes::build_new_work_times();
            self.work_duration.push(work_times);
        } else {
            match self.work_duration.last().unwrap().end {
                Some(_) => {
                    let work_times = WorkTimes::build_new_work_times();
                    let start = work_times.start.clone().unwrap();
                    
                    let end = self.work_duration.last().unwrap().end.clone().unwrap();
                    let duration_pause = end.until(&start).unwrap();
                    self.work_duration.last_mut().unwrap().pause = Some(duration_pause);
                    
                    self.work_duration.push(work_times);
                    self.sum_pauses();
                }
                None => {
                    println!("WARN ::: end wasn't set jet")    
                }
            }
        }
    }

    pub fn stop(&mut self) {
        match self.work_duration.last().unwrap().end {
            Some(_) => {
                println!("WARN ::: end was already set")
            }
            None => {
                self.work_duration.last_mut().unwrap().set_end();
                let start = self.work_duration.last().unwrap().start.clone().unwrap();
                let stop = self.work_duration.last().unwrap().end.clone().unwrap();
                let duration = start.until(&stop).unwrap();
                self.work_duration.last_mut().unwrap().duration = Some(duration);
                self.sum_durations();
                let hours = duration.get_hours() as f32;
                let minutes = duration.get_minutes() as f32 / 60.;
                self.sum_total = self.sum_total + hours + minutes
            }
        }
    }

    pub fn sum_durations(&mut self) {
        let work_times_vec = &self.work_duration;
        let mut sum = Span::new();

        for item in work_times_vec {
            if let Some(duration) = item.duration {
                sum = sum.checked_add(duration).unwrap();
            }
        }
        self.sum_work = Some(sum);
    }

    pub fn sum_pauses(&mut self) {
        let work_times_vec = &self.work_duration;
        let mut sum = Span::new();

        for item in work_times_vec {
            if let Some(duration) = item.pause {
                sum = sum.checked_add(duration).unwrap();
            }
        }
        self.sum_pause = Some(sum);
    }
}
