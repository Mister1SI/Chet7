
pub mod lib {
    mod gui;

    pub struct Chet7 {
        address: String,
    }

    #[derive(Debug, Clone)]
    pub enum Message {
        AddressUpdate(String),
    }

}
