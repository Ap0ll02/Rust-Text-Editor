use iced::widget::{column, container, horizontal_space, row, text, text_editor};
use iced::{executor, Application, Command, Element, Length, Settings, Theme};
use std::io;
use std::path::Path;
use std::sync::Arc;

pub fn main() -> iced::Result {
    Editor::run(Settings::default())
}

struct Editor {
    content: text_editor::Content,
}

#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
    FileOpened(Result<Arc<String>, io::ErrorKind>),
}

impl Application for Editor {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            Self {
                content: text_editor::Content::new(),
            },
            Command::perform(
                load_file(format!("{}/src/main.rs", env!("CARGO_MANIFEST_DIR"),)),
                Message::FileOpened,
            ),
        )
    }

    fn title(&self) -> String {
        String::from("Muse Editor 1.0")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Edit(action) => {
                self.content.perform(action);
            }
            Message::FileOpened(result) => {
                if let Ok(content) = result {
                    self.content = text_editor::Content::with_text(&content);
                }
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let input = text_editor(&self.content).on_action(Message::Edit);
        let position = {
            let (line, column) = self.content.cursor_position();
            text(format!("{}:{}", line + 1, column + 1))
        };
        let status_bar = row![horizontal_space().width(Length::Fill), position];
        let ui = column![input, status_bar].spacing(10);
        container(ui).padding(15).into()
    }

    fn theme(&self) -> Theme {
        Theme::Dracula
    }
}
async fn load_file(path: impl AsRef<Path>) -> Result<Arc<String>, io::ErrorKind> {
    tokio::fs::read_to_string(path)
        .await
        .map(Arc::new)
        .map_err(|error| error.kind())
}
