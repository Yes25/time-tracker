use std::borrow::Borrow;

use iced::{executor, Command};
use iced::{Application, alignment, Element, Length, Padding};
use iced::widget::{button, column, container, row, text, Container, Column};
use jiff::Unit;


use crate::gui::gui_logic::OneDaysWork;

pub struct AppState {
    state: State,
    todays_work: OneDaysWork,
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
            Self {
                state: State::Stopped,
                todays_work: OneDaysWork::init(),
            }, 
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
    
    let mut col: Column<Message> = Column::new();

    for item in &one_days_work.work_duration {
        let mut start_label = "".to_owned();
        let mut stop_label = "".to_owned();
        let mut duration_label = "".to_owned();
        if let Some(start) = &item.start {
            start_label = start.time().round(Unit::Second).unwrap().to_string();
        }
        if let Some(end) = &item.end {
            stop_label = end.time().round(Unit::Second).unwrap().to_string();
        }
        if let Some(duration) = &item.duration {
            duration_label = duration.get_minutes().to_string();
        }

        col = col.push(
            row!(
                text("start: ".to_owned() + &start_label),
                text("stop: ".to_owned() + &stop_label),
                text("duration: ".to_owned() + &duration_label),
            )
            .spacing(20)
        );
    }

    let one_days_work_wideget = container(
        column!(
            row!(
                text("date: "),
                text(date_label),
            ),
            col,
        )
    );

    one_days_work_wideget.into()
}
