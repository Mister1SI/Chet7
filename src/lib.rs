
pub mod lib {
    mod gui;
    mod netcode;

    pub struct Chet7 {
        address: String,
        username: String,
    }

    #[derive(Debug, Clone)]
    pub enum Message {
        AddressUpdate(String),
        UsernameUpdate(String),
        Connect,
    }

}
