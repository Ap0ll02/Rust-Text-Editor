// Crate Use Statements
use iced::widget::{button, column, text, Container};
use iced::{Alignment, Element, Sandbox, Settings};

// // Running The Application. Giving The Window A Name "Scribe"
pub fn main() -> iced::Result {
    // iced::run("Text Editor", iced::Settings::default())
    TextState::run(Settings::default())
}

// A Text Editor Program: But First Testing The ICED Library With A Simple Text Layout
#[derive(Default)]
struct TextState {
    value: String,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    DName,
}

impl Sandbox for TextState {
    type Message = Message;
    
    fn new() -> Self {
        Self { value: "Hi".to_string() }
    }

    fn title(&self) -> String {
        String::from("Code Editor 1.0")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::DName => {
                self.value = "Welcome To My Editor, Jack.".to_string();
            }
        }
    }

    fn view(&self) -> Element<Message> {

        let ui = column![
            text(self.value.clone()).size(50),
            button("Login").on_press(Message::DName),
        ].padding(40)
        .align_items(Alignment::Center);
        let container = Container::new(ui).center_x().center_y().width(iced::Length::Fill).height(iced::Length::Fill).into();
        container
    }
    
    fn theme(&self) -> iced::Theme {
        iced::Theme::default()
    }
    
    fn style(&self) -> iced::theme::Application {
        iced::theme::Application::default()
    }
    
    fn scale_factor(&self) -> f64 {
        1.0
    }
    
    fn run(settings: Settings<()>) -> Result<(), iced::Error>
    where
        Self: 'static + Sized,
    {
        <Self as iced::Application>::run(settings)
    }
}

// Test to ensure the message/interaction is changing the state.
#[test]
fn it_prints() {
    let mut label = TextState::default();
    label.update(Message::DName);
    assert_eq!(label.value, "Welcome To My Editor, Jack.".to_string())
}