use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Passage {
    pub act: u16,
    pub location: String,
    pub id: String,
    pub messages: Vec<Message>,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    pub id: String,
    pub text: String,
}