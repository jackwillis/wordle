use std::io;
use std::io::Write;

use wordle::{Game, GameStatus, LetterKnowledge, Word, WordParseError, WordScore};

fn main() {
    println!("WORDLE!");
    println!("Type \"help\" for game rules.");

    let secret_word: Word = wordle::random_word();
    let game: Game = Game::new(secret_word);

    game_loop(game);
}

static HELP_MESSAGE: &str = r"Guess the secret word -- a random five-letter-long English word.

Make up to (6) guesses.

An 'X' under a letter means you guessed the right letter in the right spot.
An 'O' means the letter you guessed there is in the word, but somewhere else.
An '_' means the letter you guessed there isn't in the word.";

#[derive(Clone, Debug)]
enum Turn {
    PlayValidWord(Word),
    PlayInvalidWord(WordParseError),
    DisplayHelpMessage,
    DisplaySecretWord,
    NoOp,
}

impl Turn {
    fn parse(input: &str) -> Turn {
        let input = input.trim();

        if input.is_empty() {
            return Turn::NoOp;
        } else if input.to_lowercase() == "help" {
            return Turn::DisplayHelpMessage;
        } else if input == "?" {
            return Turn::DisplaySecretWord;
        }

        match input.parse::<Word>() {
            Ok(word) => Turn::PlayValidWord(word),
            Err(err) => Turn::PlayInvalidWord(err),
        }
    }
}

fn advance_game(turn: Turn, game: Game) -> Game {
    match turn {
        // Typical case
        Turn::PlayValidWord(word) => {
            let new_game: Game = game.with_prediction(word);

            print_player_knowledge(&new_game);

            new_game
        }

        // Cases with no state change
        Turn::PlayInvalidWord(msg) => {
            println!("Invalid word: {}", msg);
            game
        }
        Turn::DisplayHelpMessage => {
            println!("{}", HELP_MESSAGE);
            game
        }
        Turn::DisplaySecretWord => {
            println!("    {}", game.secret_word); // offset to line up with prompt
            game
        }
        Turn::NoOp => game,
    }
}

/// Read, evaluate, print, loop (recurse).
/// Max depth is Game::MAXIMUM_PLAYS == 6.
fn game_loop(game: Game) {
    match game.calculate_status() {
        GameStatus::Won => println!("You're a winner, baby!"),
        GameStatus::Lost => println!("You lost :(\nThe word was: {}", game.secret_word),
        GameStatus::Active => {
            print_prompt(&game);
            let input: String = read_line();
            let turn: Turn = Turn::parse(&input);
            let new_game: Game = advance_game(turn, game);
            game_loop(new_game);
        }
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
    let last_score: &WordScore = game.last_score().unwrap();
    print!("    {} // ", last_score); // offset to line up with prompt

    let letter_knowledge: &LetterKnowledge = &game.letter_knowledge;

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
