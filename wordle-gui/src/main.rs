// Hide console window in release builds on Windows, this blocks stdout.
// See <https://github.com/emilk/eframe_template/commit/86fe7b7b87e3a3868ce2648a3f2a63b6a044133f>.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use iced::{
    text_input, window, Align, Color, Column, Element, Font, Sandbox, Settings, Text, TextInput,
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

// Fonts
const NANUM_GOTHIC_BOLD: Font = Font::External {
    name: "NanumGothicCoding-Bold",
    bytes: include_bytes!("../fonts/NanumGothicCoding/NanumGothicCoding-Bold.ttf"),
};

const NANUM_GOTHIC_REGULAR: Font = Font::External {
    name: "NanumGothicCoding-Regular",
    bytes: include_bytes!("../fonts/NanumGothicCoding/NanumGothicCoding-Regular.ttf"),
};

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

                let text_input_value = value
                    .to_uppercase()
                    .chars()
                    .filter(|c| c.is_ascii_alphabetic())
                    .collect::<String>();
                self.text_input_value = text_input_value;
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
        let mut column = Column::new()
            .padding(20)
            .align_items(Align::Center)
            .spacing(10);

        // Title
        let title_label = "Wordle";
        let title = Text::new(title_label).size(50).font(NANUM_GOTHIC_BOLD);
        column = column.push(title);

        // Text input
        let placeholder = "Enter your guess";
        let text_input: Element<Message> = TextInput::new(
            &mut self.text_input_state,
            placeholder,
            &self.text_input_value,
            Message::TextInputChanged,
        )
        .on_submit(Message::TextInputSubmitted)
        .padding(10)
        .font(NANUM_GOTHIC_REGULAR)
        .into();
        column = column.push(text_input);

        // Flash
        if let Some(flash_message) = &self.flash_message {
            let orangered = Color::from_rgb8(255, 69, 0);
            let flash_text: Element<Message> = Text::new(flash_message)
                .size(20)
                .color(orangered)
                .font(NANUM_GOTHIC_REGULAR)
                .into();
            column = column.push(flash_text);
        }

        // Guessed words
        for word in &self.words {
            let word_label = Text::new(word.to_owned())
                .size(32)
                .font(NANUM_GOTHIC_REGULAR);
            column = column.push(word_label);
        }

        column.into()
    }
}
