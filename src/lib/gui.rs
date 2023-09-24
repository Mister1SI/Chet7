use iced::{Sandbox, Length, Alignment, Pixels};
use iced::widget::{Container, Column, Row, TextInput, Button, Rule, Text};

use crate::lib::{netcode, Chet7, Message};


impl Sandbox for Chet7 {
    type Message = Message;

    fn new() -> Self {
        Chet7 { 
            address: String::from(""),
            username: String::from(""),
            message: String::from(""),
            stream: None,
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
            Message::Connect => {
                if let Ok(s) = netcode::connect(self.address.clone()) {
                    self.stream = Some(s);
                }
                ()
            }
            Message::MessageInputUpdate(m) => self.message = m,
            Message::SendMessage(m) => {
                if let Some(s) = &mut self.stream {
                    match netcode::send_message(s, m) {
                        Ok(_) => (),
                        Err(e) => {
                            println!("{e}");
                        }
                    }
                }
                
            }
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



        let msg_log = Text::new("Messages go here").height(Length::Fill);

        let msg_input: TextInput<'_, Message> = TextInput::new("Send a message", self.message.as_str())
                .on_input(Message::MessageInputUpdate);



        let col = Column::new().align_items(Alignment::Center).push(top_row)
                .push(conn_button).push(rule).push(msg_log).push(msg_input);
        Container::new(col).into()
    }

}