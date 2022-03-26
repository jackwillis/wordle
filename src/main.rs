use std::io;
use std::io::Write;
use std::str::FromStr;

use wordle::{Game, GameStatus, Word};

fn main() {
    println!("WORDLE!");
    println!("Type \"help\" for more information.");

    let secret_word = wordle::random_word();
    let game = Game::new(secret_word);

    game_loop(game);
}

static HELP_MESSAGE: &str = r"The goal of the game is to guess the secret word.
The secret word is a random five-letter-long English word.
You may make up to six guesses. The number of guesses is displayed to the left of the prompt.
You may guess any five-character sequence.
After you guess, the outcome of your guess will be shown below.
    * X represents a correct letter guess in the correct spot.
    * O means the letter is part of the word but not at that position.
    * _ means the letter is not in the word.
    * Known good, bad, and unknown letters are also shown.";

/// Represents the result of parsing user input
enum Command {
    MakeLegalPlay(Word),
    DisplayHelpMessage,
    SyntaxError(&'static str),
    NoOp,
}

impl Command {
    /// Parses user input from command line.
    fn parse(input: &str) -> Command {
        let input = input.trim();

        if input.is_empty() {
            return Command::NoOp;
        } else if input.to_lowercase() == "help" {
            return Command::DisplayHelpMessage;
        }

        match Word::from_str(input) {
            Ok(prediction) => Command::MakeLegalPlay(prediction),
            Err(msg) => Command::SyntaxError(msg),
        }
    }
}

/// Read user input - parse into [Command].
/// Evaluate new game state - with [Game] and [Word].
/// Print player's knowledge - represented in [WordScore] and [LetterKnowledge].
/// Loop - terminates when `game.calculate_status()` is no longer [GameStatus::Active].
fn game_loop(game: Game) {
    match game.calculate_status() {
        // Base cases for recursion.
        GameStatus::Won => println!("You're a winner, baby!"),
        GameStatus::Lost => println!("You lost :(\nThe word was: {}", game.secret_word),

        // Entry point and normal case.
        // Ends with tail recursion.
        GameStatus::Active => {
            print_prompt(&game);

            let input = read_line();

            match Command::parse(&input) {
                Command::MakeLegalPlay(prediction) => {
                    let new_game = game.add_prediction(prediction);

                    print_player_knowledge(&new_game);
                    game_loop(new_game);
                }
                Command::DisplayHelpMessage => {
                    println!("{}", HELP_MESSAGE);
                    game_loop(game);
                }
                Command::SyntaxError(msg) => {
                    println!("Invalid word: {}", msg);
                    game_loop(game);
                }
                Command::NoOp => game_loop(game),
            }
        }
    }
}

fn read_line() -> String {
    let mut input_buffer = String::new();
    io::stdin()
        .read_line(&mut input_buffer)
        .expect("Failed to read from stdin.");
    input_buffer
}

fn print_prompt(game: &Game) {
    print!("{} ", game.remaining_guesses());
    io::stdout().flush().expect("Failed to flush stdout.");
}

fn print_player_knowledge(game: &Game) {
    let last_score = game.last_score().unwrap();
    print!("  {} | ", last_score);

    let letter_knowledge = &game.letter_knowledge;

    print!("good: ");
    for c in &letter_knowledge.good {
        print!("{}", c);
    }

    print!(" | bad: ");
    for c in &letter_knowledge.bad {
        print!("{}", c);
    }

    print!(" | unknown: ");
    for c in &letter_knowledge.unknown {
        print!("{}", c);
    }

    println!();
}
