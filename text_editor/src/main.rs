// A Text Editor Program: But First Testing The ICED Library With A Simple Text Layout
struct TextState {
    value: String,
}

enum Message {
    DName,
}

impl TextState {
    fn update(&mut self, message: Message) {
        match message {
            Message::DName => {
                self.value = "Welcome To My Editor, Jack.".to_string();
            }
        }
    }
}

// // Running The Application. Giving The Window A Name "Scribe"
fn main() {
    // iced::run("Text Editor", iced::Settings::default())
    println!("Hello Saturn");
}

// Test to ensure the message/interaction is changing the state.
#[test]
fn it_prints() {
    let mut label = TextState {value: "Good Morning".to_string()};
    label.update(Message::DName);
    assert_eq!(label.value, "Welcome To My Editor, Jack.".to_string())
}