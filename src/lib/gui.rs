use iced::{Sandbox, Length, Alignment, Pixels};
use iced::widget::{Container, Column, Row, TextInput, Button, Rule, Text};

use crate::lib::{netcode, Chet7, Message};

impl Sandbox for Chet7 {
    type Message = Message;

    fn new() -> Self {
        Chet7 { 
            address: String::from(""),
            username: String::from(""),
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
            Message::UsernameUpdate(u) => self.username = u,
            Message::Connect => (),
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {

        let addr_box: TextInput<'_, Message> = TextInput::new("ip:port", self.address.as_str())
                .on_input(Self::Message::AddressUpdate);

        let uname_box: TextInput<'_, Message> = TextInput::new("username", self.username.as_str())
                .on_input(Self::Message::UsernameUpdate);

        let conn_button: Button<'_, Message> = Button::new("Connect").on_press(Self::Message::Connect).width(Length::Fixed(80f32));
        
        let top_row = Row::new().push(addr_box).push(uname_box);



        let rule = Rule::horizontal(Pixels::from(10)); 



        let msg_log = Text::new("Messages go here");

        let col = Column::new().align_items(Alignment::Center).push(top_row).push(conn_button).push(rule).push(msg_log);
        Container::new(col).into()
    }

}