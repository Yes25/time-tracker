use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use jiff::civil::date;
use jiff::civil::Date;

pub struct Config {
    pub hours_week: f32,
    pub start_date: Date,
}

pub fn get_config() -> Config {
    let mut path = env::current_exe().unwrap();
    path.set_file_name(".config");
    path.set_extension("txt");

    if !path.exists() {
        write_new_config_file(&path);
    }
    read_config_file(&path)
}


fn write_new_config_file(path: &PathBuf) {
    let content = "hours_week = 38.5\nstart_date = 2024-07-01".as_bytes();

    let mut file = File::create(path).unwrap();
    let _ = file.write_all(content.into());
}


fn read_config_file(path: &PathBuf) -> Config {
    let content = fs::read_to_string(path)
        .expect("Config file should exist");

    let mut config = Config {
        hours_week: 0.,
        start_date: date(2024, 07, 01),
    };

    for line in content.lines() {
        let key_val = line.split_once("=");
        if let Some((key, val)) = key_val  {
            let key = key.to_string().replace(" ", "");
            let val = val.to_string().replace(" ", "");

            if key == "hours_week" {
                config.hours_week = val.parse::<f32>().unwrap();
            }
            if key == "start_date" {
                config.start_date = val.parse::<Date>().unwrap();
            }
        }
    }
    config
}