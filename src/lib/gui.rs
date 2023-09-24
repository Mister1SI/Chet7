use iced::Sandbox;
use iced::widget::{Container, Column, TextInput};

use crate::lib::{Chet7, Message};

impl Sandbox for Chet7 {
    type Message = Message;

    fn new() -> Self {
        Chet7 { 
            address: String::from(""),
        }
    }

    fn title(&self) -> String {
        String::from("Chet7")
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::AddressUpdate(s) => self.address = s,
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let addr_box: TextInput<'_, Message> = TextInput::new("ip:port", self.address.as_str())
                .on_input(Self::Message::AddressUpdate);

        let col = Column::new().push(addr_box);
        Container::new(col).into()
    }

}