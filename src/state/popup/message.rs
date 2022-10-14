#[derive(Clone)]
pub struct MessageState {
    pub value: String,
}

impl From<String> for MessageState {
    fn from(value: String) -> Self {
        Self { value }
    }
}
