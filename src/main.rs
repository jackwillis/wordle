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

/// Represents the result of parsing user input
enum Command {
    /// Display the help message
    HelpMessage,
    /// Make a legal play
    LegalPlay(Word),
    /// Command could not be understood
    SyntaxError(&'static str),
}

impl Command {
    fn parse(input: &str) -> Command {
        if input.to_lowercase() == "help" {
            Command::HelpMessage
        } else {
            match Word::try_from(input) {
                Ok(prediction) => Command::LegalPlay(prediction), // input was a legal word
                Err(msg) => Command::SyntaxError(msg),
            }
        }
    }
}

// Recursive
fn game_loop(game: Game) {
    match game.status() {
        GameStatus::Won => println!("You're a winner, baby!"),
        GameStatus::Lost => println!("You lost :(\nThe word was: {}", game.secret_word),
        GameStatus::Active => {
            print_prompt(&game);

            // get next command, execute and recurse back into loop
            let input = read_line();
            match Command::parse(&input) {
                Command::LegalPlay(prediction) => {
                    // create new game object on heap
                    let new_game = game.with_prediction(prediction);
                    print_player_knowledge(&new_game);
                    game_loop(new_game);
                }
                Command::HelpMessage => {
                    println!("Example help message");
                    game_loop(game);
                }
                Command::SyntaxError(msg) => {
                    println!("Invalid word: {}", msg);
                    game_loop(game);
                }
            }
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
