use iced::Sandbox;
use iced::widget::{Container, Column};

use crate::lib::{Chet7, Message};

impl Sandbox for Chet7 {
    type Message = Message;

    fn new() -> Self {
        Chet7 {  }
    }

    fn title(&self) -> String {
        String::from("Chet7")
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let col = Column::new();
        Container::new(col).into()
    }

}