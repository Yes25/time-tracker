use std::collections::HashMap;
use iced::Task;
use iced::{alignment, Element, Length, Padding};
use iced::widget::{button, checkbox, column, container, horizontal_rule, horizontal_space, pick_list, row, text, vertical_space, Button, Column, Container, Row, Text};
use iced_aw::{date_picker, date_picker::Date};
use jiff::{Span, SpanRound, Unit, Zoned};
use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::utils::{compute_hours_and_minutes, compute_should_hours, format_duration, jiff_date_from_picker};
use crate::gui::gui_logic::{OneDaysWork};
use crate::gui::serialize::{export, init_calendar, Calendar};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Location {
    Homeoffice,
    Office,
}

impl Location {
    const ALL: [Location; 2] = [
        Location::Homeoffice,
        Location::Office,
    ];
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Location::Homeoffice => "home-office",
                Location::Office => "office",
            }
        )
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct App {
    pub config: Config,
    pub state: State,
    pub show_picker: bool,
    pub date: jiff::civil::Date,
    pub calendar: HashMap<String, OneDaysWork>,
}

fn init_app_state() -> App {
    let config = Config::get_config();
    let calendar = init_calendar();

    let mut state = State::Stopped;
    if let Some(todays_work) = calendar.get(&Zoned::now().date().to_string()).unwrap().work_duration.last() {
        if  todays_work.end.is_none() && todays_work.start.is_some()  {
            state = State::Started
        }
    }

    App {
        config,
        state,
        date: Zoned::now().date(),
        show_picker: false,
        calendar,
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Start,
    Stop,
    Export,
    ChooseDate,
    SubmitDate(Date),
    CancelDate,
    LocationSelected(Location),
    VacationToggled(bool),
}

#[derive(Serialize, Deserialize, Clone)]
pub enum State {
    Started,
    Stopped,
    NotToday,
} 

impl App {
	
	pub(crate) fn new() -> (Self, Task<Message>) {
		(
            init_app_state(),
            Task::none()
        )
	}
    
    pub(crate) fn theme(&self) -> iced::Theme {
        iced::Theme::TokyoNightStorm
	}

    pub(crate) fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Start => {
                self.calendar.get_mut(&Zoned::now().date().to_string()).unwrap().start();
                self.state = State::Started;
                Calendar::update(&self.calendar);
            }
            Message::Stop => {
                self.calendar.get_mut(&Zoned::now().date().to_string()).unwrap().stop();
                self.state = State::Stopped;
                Calendar::update(&self.calendar);
            }
            Message::Export => {
                export(&self.config);
            }
            Message::ChooseDate => {
                self.show_picker = true;
            }
            Message::SubmitDate(date) => {
                let date = jiff_date_from_picker(date);
                if let None = self.calendar.get(&date.to_string()) {
                    self.calendar.insert(date.to_string(), OneDaysWork {
                        date,
                        location: Some(Location::Homeoffice),
                        work_duration: vec![],
                        sum_work: None,
                        sum_pause: None,
                        vacation: false,
                    });
                    }
                self.date = date;
                self.show_picker = false;
                if date != Zoned::now().date() {
                    self.state = State::NotToday;
                } else {
                    self.state = State::Stopped;
                    if let Some(todays_work) = self.calendar.get(&Zoned::now().date().to_string()).unwrap().work_duration.last() {
                        if  todays_work.end.is_none() && todays_work.start.is_some()  {
                            self.state = State::Started
                        }
                    }
                }
            }
            Message::CancelDate => {
                self.show_picker = false;
            }
            Message::LocationSelected(location) => {
                self.calendar.get_mut(&self.date.to_string()).unwrap().location = Some(location);
                Calendar::update(&self.calendar);
            }
            Message::VacationToggled(is_vacation) => {
                self.calendar.get_mut(&self.date.to_string()).unwrap().vacation = is_vacation;

                let work_hours= self.config.get_workday_span();

                if is_vacation {
                    let mut work = Span::new();
                    if let Some(sum_work) = self.calendar.get(&self.date.to_string()).unwrap().sum_work {
                        work = sum_work;
                    }
                    let new_sum = work.checked_add(work_hours).unwrap();
                    self.calendar.get_mut(&self.date.to_string()).unwrap().sum_work = Some(new_sum);
                } else {
                    if let Some(sum_work) = self.calendar.get(&self.date.to_string()).unwrap().sum_work {
                        let new_sum = sum_work.checked_sub(work_hours).unwrap();
                        self.calendar.get_mut(&self.date.to_string()).unwrap().sum_work = Some(new_sum);
                    }
                }
                Calendar::update(&self.calendar);
            }
        }
        Task::none()
    }

	pub(crate) fn view(&self) -> Element<Message> {

        let pick_list = row!(pick_list(
            &Location::ALL[..],
            self.calendar.get(&self.date.to_string()).unwrap().location,
            Message::LocationSelected,
        ))
            .width(Length::Fill)
            .padding(Padding{top:5., right:0., bottom:5., left:10.});

        let vacation_checkbox = row!(checkbox("Vacation", self.calendar.get(&self.date.to_string()).unwrap().vacation)
            .on_toggle(Message::VacationToggled))
            .padding(Padding{top:5., right:0., bottom:25., left:10.});

        let main_container = Container::new(
            row!(
                column!(
                    date_section(self),
                    one_days_work(&self.calendar.get(&self.date.to_string()).unwrap()),
                )
                .padding(Padding::from(10))
                .height(Length::Fill)
                .width(Length::FillPortion(4)),
                column!(
                    start_stop_btn(&self.state),
                    pick_list,
                    vacation_checkbox,
                    table_totals(self),
                    vertical_space(),
                    row!(
                        horizontal_space(),
                        button("export").on_press(Message::Export),
                    )
                    .width(Length::Fill)
                    .spacing(15)
                    .padding(Padding::from(10))
                )
                .height(Length::Fill)
                .width(Length::FillPortion(2)),
            )
        );
        
        main_container.height(Length::Fill)
        .width(Length::Fill)
        .align_x(alignment::Horizontal::Center)
        .align_y(alignment::Vertical::Center)
        .into()
    }

    fn total_worked_hours(&self) -> f32 {
        let mut sum: f32 = 0.;
        for work_day in self.calendar.values() {
            if let Some(work_hours) = work_day.sum_work {
                let hours = work_hours.get_hours() as f32;
                let minutes = work_hours.get_minutes() as f32 / 60.;
                sum += hours + minutes;
            }
        }
        sum
    }
}


fn start_stop_btn(state: &State) -> Element<Message> {
    let start_btn = button("Start");
    let stop_btn= button("Stop");
    let (start_btn, stop_btn) = match state {
        State::Stopped => (start_btn.on_press(Message::Start), stop_btn),
        State::Started => (start_btn, stop_btn.on_press(Message::Stop)),
        State::NotToday => (start_btn, stop_btn),
    };

    row!(
        start_btn,
        stop_btn,
    )
    .spacing(23)
    .padding(Padding::from(10))
    .width(Length::Fill)
    .into()
}


fn date_section(app: &App) -> Element<Message> {
    let mut picker_date = Date::today();

    let date_label = app.date.to_string();
    picker_date.year = app.date.year() as i32;
    picker_date.month = app.date.month() as u32;
    picker_date.day = app.date.day() as u32;


    let date_btn = Button::new(Text::new("Date"))
        .on_press(Message::ChooseDate)
        .padding(Padding{top: 5., right: 5., bottom:5., left:5.});

    let date_picker = date_picker(
        app.show_picker,
        picker_date,
        date_btn,
        Message::CancelDate,
        Message::SubmitDate,
    ).font_size(12);

    row!(
        date_picker,
        container(
            text(date_label)
        ).padding(Padding{top: 3., right: 5., bottom:0., left:15.})
    ).padding(Padding{top: 0., right: 0., bottom:15., left:0.})
    .into()
}


fn one_days_work(one_days_work: &OneDaysWork) -> Element<Message> {
    let padding = Padding{top: 2., left: 5., bottom: 2., right: 0.};
    let col_width = 75;

    let mut start_col: Column<Message> = column!( row!( text("Start") ) ).padding(padding).width(col_width);
    let mut stop_col: Column<Message> = column!( row!( text("Stop") ) ).padding(padding).width(col_width);
    let mut duration_col: Column<Message> = column!( row!( text("Duration") ) ).padding(padding).width(col_width);
    let mut pause_col: Column<Message> = column!( row!( text("Break") ) ).padding(padding).width(col_width);

    for item in &one_days_work.work_duration {
        let mut start_label = "".to_owned();
        let mut stop_label = "".to_owned();
        let mut duration_label = "".to_owned();
        let mut pause_label = "".to_owned();

        if let Some(start) = &item.start {
            start_label = start.time().round(Unit::Minute).unwrap().to_string()[0..5].to_owned();
        }
        if let Some(end) = &item.end {
            stop_label = end.time().round(Unit::Minute).unwrap().to_string()[0..5].to_owned();
        }
        if let Some(duration) = &item.duration {
            duration_label = format_duration(duration);
        }
        if let Some(pause) = &item.pause {
            pause_label = format_duration(pause);
        }

        start_col = start_col.push(row!(text(start_label)));
        stop_col = stop_col.push(row!(text(stop_label)));
        duration_col = duration_col.push(row!(text(duration_label)));
        pause_col = pause_col.push(row!(text(pause_label)));
        
    }

    duration_col = duration_col.push(row!(horizontal_rule(4)));
    pause_col = pause_col.push(row!(horizontal_rule(4)));

    duration_col = duration_col.push(row!(text(compute_sum_one_days_work(one_days_work))));
    pause_col = pause_col.push(row!(text(compute_sum_one_days_breaks(one_days_work))));

    let mut table: Row<Message> = Row::new();
        table = table.push(start_col);
        table = table.push(stop_col);
        table = table.push(duration_col);
        table = table.push(pause_col);


    let one_days_work_widget = container(
            table
    );

    one_days_work_widget.into()
}


fn table_totals(app: &App) -> Element<'static, Message> {
    let start_date = app.config.start_date;
    let today = Zoned::now().date();
    let should_hours = compute_should_hours(start_date, today, &app.config);
    let sum_til_last_day = app.total_worked_hours();

    let delta = sum_til_last_day - should_hours;
    let (hours_delta, minutes_delta) = compute_hours_and_minutes(delta);
    let mut sign = "+";
    if delta < 0. {
        sign = "-";
    }
    let delta_label = format!("{sign} {}:{:0>2}", hours_delta.abs(), minutes_delta.abs());
    
    let work_all_times: Row<Message> = row!(
        text("Contingent: "),
        text(delta_label)
    )
        .padding(Padding{top: 5., right: 0., bottom:5., left:10.});
    
    let mut table: Column<Message> = Column::new();
        table = table.push(work_all_times);
    
    table.into()
}


fn compute_sum_one_days_work(one_days_work: &OneDaysWork) -> String {
    let mut sum_duration = String::from("");
    if let Some(sum) = one_days_work.sum_work {
        let sum = sum.round(SpanRound::new().largest(Unit::Hour)).unwrap();
        let hours = sum.get_hours().to_string();
        let minutes = sum.get_minutes().to_string();
        sum_duration = format!("{}:{}", hours, minutes);
    }
    sum_duration
}


fn compute_sum_one_days_breaks(one_days_work: &OneDaysWork) -> String {
    let mut sum_pauses = String::from("");
    if let Some(sum) = one_days_work.sum_pause {
        let sum = sum.round(SpanRound::new().largest(Unit::Hour)).unwrap();
        let hours = sum.get_hours().to_string();
        let minutes = sum.get_minutes().to_string();
        sum_pauses = format!("{}:{}", hours, minutes);
    }
    sum_pauses
}