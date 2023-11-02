use iced::executor;
use iced::widget::{column, container, text_input};
use iced::{Application, Command, Element, TextInput, Theme};

fn main() -> iced::Result {
  Carrier::run(Settings::default())
}

struct Carrier {
  input: String
}


#[derive(Debug, Clone)]
enum Message {
  TextInputChanged(TextInput::Action),
}

impl Application for Carrier {
  type Message = Message;
  type Theme = Theme;
  type Executor = executor::Default;
  type Flags = ();

  fn new(_flags: Self::Flags) -> Self, Command<Message>) {
    Self.default()
  }

  fn title(&self) -> String {
    String::from("Carrier")
  }

  fn update(&mut self, message: Message) -> Command<Message> {
    match message {
      Message::TextInputChanged(action) => {
        self.content.edit(action)
      }
    }
  }

  fn view(&self) -> iced::Element<Message> {
    let request_url = TextInput::new("request url", "")
      .on_input(Message::TextInputChanged)
      .padding(10);

    container(column![request_url])
      .padding(10)
      .into()
  }

}
