use std::io;
use std::io::Write;

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
    /// In the normal case, user input is a valid word,
    /// and this function will return a [Command::MakeLegalPlay(Word)].
    fn parse(input: &str) -> Command {
        let input = input.trim();
        if input.is_empty() {
            Command::NoOp
        } else if input.to_lowercase() == "help" {
            Command::DisplayHelpMessage
        } else {
            // try to parse a word
            match Word::try_from(input) {
                Ok(prediction) => Command::MakeLegalPlay(prediction), // input was a legal word
                Err(msg) => Command::SyntaxError(msg),
            }
        }
    }
}

/// read user input - parse into [Command]
/// evaluate new game state - with [Game] and [Word]
/// print player knowledge - represented in [WordScore] and [LetterKnowledge]
/// loop - terminates when `game.status()` is no longer [GameStatus::Active].
fn game_loop(game: Game) {
    match game.status() {
        // Base case for recursion
        GameStatus::Won => println!("You're a winner, baby!"),
        GameStatus::Lost => println!("You lost :(\nThe word was: {}", game.secret_word),

        // Entry point and main loop
        GameStatus::Active => {
            print_prompt(&game);

            let input = read_line();

            // at the end here, tail recursion
            match Command::parse(&input) {
                // normal case - user played valid word
                Command::MakeLegalPlay(prediction) => {
                    // evaluate prediction - create new game state on heap
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
    String::from(input_buffer)
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
