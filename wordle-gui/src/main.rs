use iced::{
    button, text_input, window, Align, Button, Column, Element, Sandbox, Settings, Text, TextInput,
};

pub fn main() -> iced::Result {
    let settings = Settings {
        window: window::Settings {
            size: (300, 500),
            resizable: true,
            decorations: true,
            ..Default::default()
        },
        ..Default::default()
    };
    App::run(settings)
}

#[derive(Default)]
struct App {
    text_input_value: String,
    text_input: text_input::State,
    words: Vec<String>,
}

#[derive(Debug, Clone)]
enum Message {
    TextInputChanged(String),
    TextInputSubmitted,
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Self {
            text_input: text_input::State::focused(), // focus text input when app just opened
            ..Default::default()
        }
    }

    fn title(&self) -> String {
        String::from("Wordle")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::TextInputChanged(value) => {
                self.text_input_value = value;
            }
            Message::TextInputSubmitted => {
                self.words.push(self.text_input_value.to_owned());
                self.text_input_value.clear();
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let title = Text::new("Wordle").size(50);

        let text_input = TextInput::new(
            &mut self.text_input,
            "This is the placeholder...",
            &self.text_input_value,
            Message::TextInputChanged,
        )
        .on_submit(Message::TextInputSubmitted)
        .padding(10);

        let mut column = Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(title)
            .push(text_input);

        for word in &self.words {
            let word_label = Text::new(word).size(12);
            column = column.push(word_label);
        }

        column.into()
    }
}
