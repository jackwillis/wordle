use std::io;
use std::io::Write;

use regex::Regex;

use wordle::{Game, GameStatus, Word};

fn main() {
    println!("WORDLE!");

    let secret_word = wordle::random_word();
    let game = Game::new(secret_word);

    game_loop(game);
}

enum Command {
    Predict(Word),
    HelpMessage,
    SyntaxError(&'static str),
}

// Recursive
fn game_loop(game: Game) {
    match game.status() {
        GameStatus::Won => println!("You're a winner, baby!"),
        GameStatus::Lost => println!("You lost :(\nThe word was: {}", game.secret_word),
        GameStatus::Active => {
            print_prompt(&game);

            match get_command() {
                Command::Predict(prediction) => {
                    let updated_game = game.with_prediction(prediction);
                    print_player_knowledge(&updated_game);
                    game_loop(updated_game); // Tail recursion
                }
                Command::HelpMessage => {
                    println!("Help message");
                    game_loop(game); // Tail recursion
                }
                Command::SyntaxError(msg) => {
                    println!("Invalid word: {}", msg);
                    game_loop(game); // Tail recursion
                }
            }
        }
    }
}

fn get_command() -> Command {
    // matches "h", "help", or "?", case-insensitive.
    let help_command = Regex::new(r"(?i)(h(elp)?|\?)").unwrap();

    let user_input = read_line();

    if user_input == "help" {
        Command::HelpMessage
    } else {
        match Word::try_from(user_input) {
            Ok(prediction) => Command::Predict(prediction),
            Err(msg) => Command::SyntaxError(msg),
        }
    }
}

fn read_line() -> String {
    let mut input_buffer = String::new();
    io::stdin()
        .read_line(&mut input_buffer)
        .expect("Failed to read from stdin.");
    String::from(input_buffer.trim())
}

fn print_prompt(game: &Game) {
    print!("{} ", game.remaining_guesses());
    io::stdout().flush().expect("Failed to flush stdout.");
}

fn print_player_knowledge(game: &Game) {
    print!("  {} | ", game.last_score().unwrap());

    print!("good: ");
    for c in &game.letter_knowledge.good {
        print!("{}", c);
    }

    print!(" | bad: ");
    for c in &game.letter_knowledge.bad {
        print!("{}", c);
    }

    print!(" | unknown: ");
    for c in &game.letter_knowledge.unknown {
        print!("{}", c);
    }

    println!();
}
