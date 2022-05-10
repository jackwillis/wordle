// Hide console window in release builds on Windows, this blocks stdout.
// See <https://github.com/emilk/eframe_template/commit/86fe7b7b87e3a3868ce2648a3f2a63b6a044133f>.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use iced::{
    text_input, window, Alignment, Color, Column, Element, Font, Sandbox, Settings, Text, TextInput,
};

pub fn main() -> iced::Result {
    let settings = Settings {
        window: window::Settings {
            size: (480, 640),
            resizable: false,
            decorations: true,
            ..Default::default()
        },
        ..Default::default()
    };
    App::run(settings)
}

#[derive(Debug, Clone)]
struct App {
    game: wordle::Game,
    text_input_value: String,
    text_input_state: text_input::State,
    flash_message: Option<String>,
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
        let game = wordle::Game::new(wordle::random_word());
        println!("Secret word is {}", game.secret_word);
        Self {
            game,
            text_input_value: String::new(),
            text_input_state: text_input::State::focused(), // focus text input when app just opened
            flash_message: None,
        }
    }

    fn title(&self) -> String {
        "Wordle".into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::TextInputChanged(value) => {
                self.text_input_value = value
                    .to_uppercase()
                    .chars()
                    .filter(|c| c.is_ascii_alphabetic())
                    .take(5)
                    .collect::<String>();
            }
            Message::TextInputSubmitted => match self.text_input_value.parse::<wordle::Word>() {
                Ok(word) => {
                    self.game = self.game.with_prediction(word);
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
        let mut column = basic_column();

        // Title
        let title_label = "Wordle";
        let title = Text::new(title_label).size(50).font(NANUM_GOTHIC_BOLD);
        column = column.push(title);

        // Main content
        let main_content = match self.game.calculate_status() {
            wordle::GameStatus::Active => self.view_active(),
            wordle::GameStatus::Lost => self.view_lost(),
            wordle::GameStatus::Won => self.view_won(),
        };
        column = column.push(main_content);

        column.into()
    }
}

fn basic_column() -> Column<'static, Message> {
    let column = Column::new()
        .padding(20)
        .align_items(Alignment::Center)
        .spacing(10);
    column
}

impl App {
    fn view_active(&mut self) -> Element<Message> {
        let mut column = basic_column();

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
        for play in &self.game.plays {
            let wordle::Play { prediction, score } = play;

            let prediction_label = Text::new(prediction.to_string())
                .size(30)
                .font(NANUM_GOTHIC_REGULAR);
            column = column.push(prediction_label);

            let score_label = Text::new(score.to_string())
                .size(30)
                .font(NANUM_GOTHIC_REGULAR);
            column = column.push(score_label);
        }

        column.into()
    }

    fn view_won(&mut self) -> Element<Message> {
        let mut column = basic_column();

        column = column.push(Text::new("You won").font(NANUM_GOTHIC_REGULAR).size(40));

        column = column.push(
            Text::new(format!("The word was {}.", self.game.secret_word))
                .font(NANUM_GOTHIC_REGULAR)
                .size(30),
        );

        column.into()
    }

    fn view_lost(&mut self) -> Element<Message> {
        let mut column = basic_column();

        column = column.push(Text::new("You lost").font(NANUM_GOTHIC_REGULAR).size(40));

        column = column.push(
            Text::new(format!("The word was {}.", self.game.secret_word))
                .font(NANUM_GOTHIC_REGULAR)
                .size(20),
        );

        column.into()
    }
}
