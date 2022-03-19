use std::io;
use std::io::Write;

use wordle::{Game, GameStatus, LegalWord};

fn main() {
    println!("WORDLE!");

    let game = Game::new(wordle::dictionary::random_word());
    game_loop(game);
}

// Recursive
fn game_loop(game: Game) {
    match game.status() {
        GameStatus::Active => {
            print_prompt(&game);

            let user_input = LegalWord::try_from(read_line());
            match user_input {
                Ok(prediction) => {
                    let updated_game = game.make_play(prediction);
                    print_player_knowledge(&updated_game);
                    game_loop(updated_game); // Tail recursion
                }
                Err(msg) => println!("Invalid word: {}", msg),
            }
        }
        GameStatus::Won => println!("You're a winner, baby!"),
        GameStatus::Lost => println!("You lost :(\nThe word was: {}", game.secret_word),
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

fn read_line() -> String {
    let mut input_buffer = String::new();
    io::stdin()
        .read_line(&mut input_buffer)
        .expect("Failed to read from stdin.");
    String::from(input_buffer.trim())
}
