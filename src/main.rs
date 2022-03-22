use std::io;
use std::io::Write;

use wordle::{
    Game,
    GameStatus::{Active, Lost, Won},
    Word,
};

fn main() {
    println!("WORDLE!");

    let secret_word = wordle::random_word();
    let game = Game::new(secret_word);

    game_loop(game);
}

// Recursive
fn game_loop(game: Game) {
    match game.status() {
        Won => println!("You're a winner, baby!"),
        Lost => println!("You lost :(\nThe word was: {}", game.secret_word),
        Active => {
            print_prompt(&game);

            match try_read_word() {
                Ok(prediction) => {
                    let updated_game = game.with_prediction(prediction);
                    print_player_knowledge(&updated_game);
                    game_loop(updated_game); // Tail recursion
                }
                Err(msg) => {
                    println!("Invalid word: {}", msg);
                    game_loop(game); // Tail recursion
                }
            }
        }
    }
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

fn try_read_word() -> Result<Word, &'static str> {
    Word::try_from(read_line())
}

fn read_line() -> String {
    let mut input_buffer = String::new();
    io::stdin()
        .read_line(&mut input_buffer)
        .expect("Failed to read from stdin.");
    String::from(input_buffer.trim())
}
