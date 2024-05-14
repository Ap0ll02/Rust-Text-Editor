use iced::widget::{text, container, text_editor};
use iced::{Theme, Element, Sandbox, Settings};

pub fn main() -> iced::Result {
    Editor::run(Settings::default())
}

struct Editor {
    content: text_editor::Content,
}

#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
}

impl Sandbox for Editor {
    type Message = Message;

    fn new() -> Self {
        Self {
            content: text_editor::Content::new(),
        }
    }

    fn title(&self) -> String {
        String::from("Muse Editor 1.0")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Edit(action) => {
                self.content.perform(action);
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let input = text_editor(&self.content).on_action(Message::Edit);
        container(input).padding(15).into()

    }

    fn theme(&self) -> Theme {
        Theme::Dracula
    }
}