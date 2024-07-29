use jiff::Zoned;

pub struct WorkTimes {
    pub label: Option<String>,
    pub start: Option<Zoned>,
    pub end: Option<Zoned>,
    pub duration: Option<f32>
}

impl WorkTimes {
    pub fn init() -> WorkTimes {
        WorkTimes {
            label: None,
            start: None,
            end: None,
            duration: None,
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

pub struct OneDaysWork {
    pub date: Option<Zoned>,
    pub work_duration: Vec<WorkTimes>,
    pub pause: Vec<i32>
}

impl OneDaysWork {

    pub fn init() -> OneDaysWork {
        OneDaysWork {
            date: None,
            work_duration: vec![],
            pause: vec![]
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
                    self.work_duration.push(work_times);
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
            }
        }
    }
}
