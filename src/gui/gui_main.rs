use iced::{executor, Command};
use iced::{Application, alignment, Element, Length, Padding};
use iced::widget::{button, column, container, row, text, Column, Container, Row};
use jiff::civil::{date, Date};
use jiff::{Unit, Zoned};


use crate::config::{get_config, Config};
use crate::gui::gui_logic::OneDaysWork;

pub struct AppState {
    config: Config,
    state: State,
    sum_til_last_day: f32,
    should_hours: f32, 
    todays_work: OneDaysWork,
}

fn init_app_state() -> AppState {
    let config = get_config();
    let hours_week = config.hours_week;
    let start_day = config.start_date;
    let today = Zoned::now().date();

    let work_span = today.since(start_day).unwrap();
    let work_days = (work_span.get_days() + 1) as f32 ;     // + 1 because today is not elapsed, hence it os not in get_days. But we want to know how much I should have worked at the end of today 
    let should_hours = work_days * (hours_week / 5.);

    // TODO: compute the should hours 

    AppState {
        config: config,
        state: State::Stopped,
        sum_til_last_day: 0.,
        should_hours: should_hours,
        todays_work: OneDaysWork::init(),
    } 
}

#[derive(Debug, Clone)]
pub enum Message {
    Start,
    Stop,
}

enum State {
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
            // Self {
            //     config: get_config(),
            //     state: State::Stopped,
            //     sum_til_last_day: 0.,
            //     shuold_hours: 0.,
            //     todays_work: OneDaysWork::init(),
            // },
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
            },
            Message::Stop => {
                self.todays_work.stop();
                self.state = State::Stopped;
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

        let mut sum_duration = String::from("Work total: ");
        if let Some(sum) = self.todays_work.sum_work {
            let hours = sum.get_hours().to_string();
            let minutes = sum.get_minutes().to_string();
            let seconds = sum.get_seconds().to_string();
            sum_duration = sum_duration + &(format!("{}:{}:{}", hours, minutes, seconds));
        }

        let mut sum_pauses = String::from("Breaks total: ");
        if let Some(sum) = self.todays_work.sum_pause {
            let hours = sum.get_hours().to_string();
            let minutes = sum.get_minutes().to_string();
            let seconds = sum.get_seconds().to_string();
            sum_pauses = sum_pauses + &(format!("{}:{}:{}", hours, minutes, seconds));
        }

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
                    .width(Length::Fill)
                    .align_items(iced::Alignment::End),
                    row!(
                        text(sum_duration),
                    ),
                    row!(
                        text(sum_pauses),
                    )
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
            start_label = start.time().round(Unit::Second).unwrap().to_string();
        }
        if let Some(end) = &item.end {
            stop_label = end.time().round(Unit::Second).unwrap().to_string();
        }
        if let Some(duration) = &item.duration {
            let hours = duration.get_hours().to_string();
            let minutes = duration.get_minutes().to_string();
            let seconds = duration.get_seconds().to_string();
            duration_label = format!("{}:{}:{}", hours, minutes, seconds);
        }
        if let Some(pause) = &item.pause {
            let hours = pause.get_hours().to_string();
            let minutes = pause.get_minutes().to_string();
            let seconds = pause.get_seconds().to_string();
            pause_label = format!("{}:{}:{}", hours, minutes, seconds);
        }

        start_col = start_col.push(row!(text(&start_label)));
        stop_col = stop_col.push(row!(text(&stop_label)));
        duration_col = duration_col.push(row!(text(&duration_label)));
        pause_col = pause_col.push(row!(text(&pause_label)));
        
    }

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
