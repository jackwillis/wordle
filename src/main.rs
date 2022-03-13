use std::io;
use std::io::Write;

use wordle::{Game, GameStatus, PlayableWord};

fn main() -> io::Result<()> {
    println!("WORDLE!");

    let mut game = Game::new(wordle::dictionary::random_word());

    while game.status() == GameStatus::Active {
        print!("{} ", game.remaining_guesses());
        io::stdout().flush()?;

        let user_input = PlayableWord::try_from(read_line()?);

        match user_input {
            Ok(prediction) => {
                game.play(prediction);

                print!("  {} | ", game.last_outcome().unwrap());

                print!("good: ");
                for c in &game.correctly_guessed_letters {
                    print!("{}", c);
                }

                print!(" | bad: ");
                for c in &game.incorrectly_guessed_letters {
                    print!("{}", c);
                }

                print!(" | unknown: ");
                for c in &game.unknown_letters {
                    print!("{}", c);
                }

                println!();
            }
            Err(msg) => {
                println!("Input error: {}", msg);
            }
        }
    }

    match game.status() {
        GameStatus::Lost => println!("You lost :(\nThe word was: {}", game.secret_word),
        GameStatus::Won => println!("You're a winner, baby!"),
        _ => unreachable!(),
    }

    Ok(())
}

fn read_line() -> io::Result<String> {
    let mut input_buffer = String::new();
    io::stdin().read_line(&mut input_buffer)?;
    Ok(String::from(input_buffer.trim()))
}
