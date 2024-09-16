use iced::{executor, Command};
use iced::{Application, alignment, Element, Length, Padding};
use iced::widget::{button, column, container, horizontal_rule, horizontal_space, row, text, vertical_space, Column, Container, Row};
use jiff::Unit;
use serde::{Deserialize, Serialize};

use crate::config::{get_config, Config};
use crate::utils::{compute_hours_and_minutes, format_duration};
use crate::gui::gui_logic::OneDaysWork;
use crate::gui::serialize::{update_work_data, export};

#[derive(Serialize, Deserialize, Clone)]
pub struct AppState {
    pub config: Config,
    pub state: State,
    pub todays_work: OneDaysWork,
}

fn init_app_state() -> AppState {
    let config = get_config();
    let todays_work = OneDaysWork::init(&config);

    let mut state = State::Stopped;
    if let Some(todays_work) = todays_work.work_duration.last() {
        if  todays_work.end.is_none() && todays_work.start.is_some()  {
            state = State::Started
        }
    }

    AppState {
        config,
        state,
        todays_work,
    } 
}

#[derive(Debug, Clone)]
pub enum Message {
    Start,
    Stop,
    Export
}

#[derive(Serialize, Deserialize, Clone)]
pub enum State {
    Started,
    Stopped
} 

impl Application for AppState {
    type Executor = executor::Default;
    type Flags = ();
    type Theme = iced::Theme;
	type Message = Message;
	
	fn new(_flags: ()) -> (AppState, Command<Self::Message>) {
		(
            init_app_state(), 
            Command::none()
        )
	}
	
	fn title(&self) -> String {
		String::from("My Time Tracker")
	}
    
    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
	}

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::Start => {
                self.todays_work.start();
                self.state = State::Started;
                update_work_data(self.todays_work.clone());

            },
            Message::Stop => {
                self.todays_work.stop();
                self.state = State::Stopped;
                update_work_data(self.todays_work.clone());
            }
            Message::Export => {
                export();
            }
        } 

        Command::none()
    }

	fn view(&self) -> Element<Self::Message> {
        let start_btn = button("Start");
        let stop_btn= button("Stop");
        let (start_btn, stop_btn) = match &self.state {
            State::Stopped => (start_btn.on_press(Message::Start), stop_btn),
            State::Started => (start_btn, stop_btn.on_press(Message::Stop))
        };

        
        
        let main_container = Container::new(
            row!(
                column!(
                    one_days_work(&self.todays_work),
                )
                .padding(Padding::from(10))
                .height(Length::Fill)
                .width(Length::FillPortion(4)),
                column!(
                    row!(
                        start_btn,
                        stop_btn,
                    )
                    .spacing(15)
                    .padding(Padding::from(10))
                    .width(Length::Fill),
                    table_totals(&self.todays_work),
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

}

fn one_days_work(one_days_work: &OneDaysWork) -> Element<'static, Message> {
    
    let mut date_label = "".to_owned();
    
    if let Some(date) = &one_days_work.date {
        date_label = date.date().to_string();
    }
    
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

        start_col = start_col.push(row!(text(&start_label)));
        stop_col = stop_col.push(row!(text(&stop_label)));
        duration_col = duration_col.push(row!(text(&duration_label)));
        pause_col = pause_col.push(row!(text(&pause_label)));
        
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

    let one_days_work_wideget = container(
        column!(
            row!(
                text("Date: "),
                text(date_label),
            )
            .padding(Padding{top: 0., right: 0., bottom:7., left:0.}),
            table,
        )
    );

    one_days_work_wideget.into()
}


fn table_totals(one_days_work: &OneDaysWork) -> Element<'static, Message> {

    let sum_til_last_day = one_days_work.sum_total;
    let should_hours = one_days_work.should_hours;

    let delta = sum_til_last_day - should_hours;
    let (hours_delta, minutes_delta) = compute_hours_and_minutes(delta);
    let delta_label = format!("{hours_delta}:{:0>2}", minutes_delta);
    
    let work_all_times: Row<Message> = row!(
        text("Contingent: "),
        text(delta_label)
    );
    
    let mut table: Column<Message> = Column::new();
        table = table.push(work_all_times);
    
    table.into()
}

fn compute_sum_one_days_work(one_days_work: &OneDaysWork) -> String {
    let mut sum_duration = String::from("");
    if let Some(sum) = one_days_work.sum_work {
        let hours = sum.get_hours().to_string();
        let minutes = sum.get_minutes().to_string();
        sum_duration = format!("{}:{}", hours, minutes);
    }
    sum_duration
}

fn compute_sum_one_days_breaks(one_days_work: &OneDaysWork) -> String {
    let mut sum_pauses = String::from("");
    if let Some(sum) = one_days_work.sum_pause {
        let hours = sum.get_hours().to_string();
        let minutes = sum.get_minutes().to_string();
        sum_pauses = format!("{}:{}", hours, minutes);
    }
    sum_pauses
}