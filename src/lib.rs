
pub mod lib {
    mod gui;
    mod netcode;

    pub struct Chet7 {
        address: String,
        username: String,
        message: String,
    }

    #[derive(Debug, Clone)]
    pub enum Message {
        AddressUpdate(String),
        UsernameUpdate(String),
        MessageInputUpdate(String),
        Connect,
    }

}
