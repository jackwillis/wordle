use std::io;
use std::io::Write;

use wordle::{Game, GameStatus, Word};

fn main() {
    println!("WORDLE!");
    println!("Type \"help\" for game rules.");

    let secret_word = wordle::random_word();
    let game = Game::new(secret_word);

    game_loop(game);
}

static HELP_MESSAGE: &str = r"Guess the secret word -- a random five-letter-long English word.

Make up to (6) guesses.

An 'X' under a letter means you guessed the right letter in the right spot.
An 'O' means the letter you guessed there is in the word, but somewhere else.
An '_' means the letter you guessed there isn't in the word.";

type WordParseError = &'static str;

/// Represents user input from command line.
enum GameMove {
    PlayValidWord(Word),
    PlayInvalidWord(WordParseError),
    DisplayHelpMessage,
    DisplaySecretWord,
    NoOp,
}

impl GameMove {
    /// Parses user input from command line.
    fn parse(input: &str) -> GameMove {
        let input = input.trim();

        if input.is_empty() {
            return GameMove::NoOp;
        } else if input.to_lowercase() == "help" {
            return GameMove::DisplayHelpMessage;
        } else if input == "?" {
            return GameMove::DisplaySecretWord;
        }

        match input.parse::<Word>() {
            Ok(word) => GameMove::PlayValidWord(word),
            Err(err) => GameMove::PlayInvalidWord(err),
        }
    }
}

/// Read, evaluate, print, loop (recurse).
/// Max depth is [Game::MAXIMUM_PLAYS] == 6.
fn game_loop(game: Game) {
    match game.calculate_status() {
        GameStatus::Won => println!("You're a winner, baby!"),
        GameStatus::Lost => println!("You lost :(\nThe word was: {}", game.secret_word),
        GameStatus::Active => {
            print_prompt(&game);
            let command = read_line();
            let game_move = GameMove::parse(&command);
            let new_game = advance_game(game_move, game);
            game_loop(new_game);
        }
    }
}

fn advance_game(game_move: GameMove, game: Game) -> Game {
    match game_move {
        // Typical case
        GameMove::PlayValidWord(word) => {
            let new_game = game.with_prediction(word);

            print_player_knowledge(&new_game);

            new_game
        }

        // Cases with no state change
        GameMove::PlayInvalidWord(msg) => {
            println!("Invalid word: {}", msg);
            game
        }
        GameMove::DisplayHelpMessage => {
            println!("{}", HELP_MESSAGE);
            game
        }
        GameMove::DisplaySecretWord => {
            println!("    {}", game.secret_word); // offset to line up with prompt
            game
        }
        GameMove::NoOp => game,
    }
}

// Views

/// Reads a line from the console into an owned [String].
fn read_line() -> String {
    let mut input_buffer = String::new();
    io::stdin()
        .read_line(&mut input_buffer)
        .expect("Failed to read from stdin.");
    input_buffer
}

/// Prints a command line prompt of the number of remaining guesses.
fn print_prompt(game: &Game) {
    print!("({}) ", game.remaining_guesses());
    io::stdout().flush().expect("Failed to flush stdout.");
}

/// Prints the score for the last play, and the player's knowledge of "good" and "bad" letters.
fn print_player_knowledge(game: &Game) {
    let last_score = game.last_score().unwrap();
    print!("    {} // ", last_score); // offset to line up with prompt

    let letter_knowledge = &game.letter_knowledge;

    print!("good: ");
    for c in &letter_knowledge.good {
        print!("{}", c);
    }

    print!(" / bad: ");
    for c in &letter_knowledge.bad {
        print!("{}", c);
    }

    print!(" / unknown: ");
    for c in &letter_knowledge.unknown {
        print!("{}", c);
    }

    println!();
}
