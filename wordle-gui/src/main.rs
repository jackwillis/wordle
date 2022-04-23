// Hide console window in release builds on Windows, this blocks stdout.
// See <https://github.com/emilk/eframe_template/commit/86fe7b7b87e3a3868ce2648a3f2a63b6a044133f>.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use iced::{text_input, window, Align, Color, Column, Element, Sandbox, Settings, Text, TextInput};

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

#[derive(Default, Debug, Clone)]
struct App {
    text_input_value: String,
    text_input_is_valid_word: bool,
    text_input_state: text_input::State,
    flash_message: Option<String>,
    words: Vec<wordle::Word>,
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
            text_input_state: text_input::State::focused(), // focus text input when app just opened
            ..Default::default()
        }
    }

    fn title(&self) -> String {
        "Wordle".into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::TextInputChanged(value) => {
                self.text_input_is_valid_word = value.parse::<wordle::Word>().is_ok();
                self.text_input_value = value;
            }
            Message::TextInputSubmitted => match self.text_input_value.parse::<wordle::Word>() {
                Ok(word) => {
                    self.words.push(word);
                    self.text_input_value.clear();
                    self.flash_message = None;
                }
                Err(err) => {
                    self.flash_message = Some(err.to_string());
                }
            },
        }
    }

    fn view(&mut self) -> Element<Message> {
        // Layout
        let mut column = Column::new().padding(20).align_items(Align::Center);

        // Title
        let title = Text::new("Wordle").size(50);
        column = column.push(title);

        // Text input
        let text_input: Element<Message> = TextInput::new(
            &mut self.text_input_state,
            "Submit your guess...",
            &self.text_input_value,
            Message::TextInputChanged,
        )
        .on_submit(Message::TextInputSubmitted)
        .padding(10)
        .into();
        column = column.push(text_input);

        // Flash
        let nbsp = String::from("\u{00A0}");
        let orangered = Color::from_rgb8(255, 69, 0);
        let flash_message = self.flash_message.to_owned().unwrap_or(nbsp);
        let flash_text: Element<Message> =
            Text::new(flash_message).size(20).color(orangered).into();
        column = column.push(flash_text);

        // Guessed words
        for word in &self.words {
            let word_label = Text::new(word.to_owned()).size(20);
            column = column.push(word_label);
        }

        column.into()
    }
}
