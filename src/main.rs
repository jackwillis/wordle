use std::io;
use std::io::Write;

use wordle::{Game, GameStatus, LegalWord};

fn main() -> io::Result<()> {
    println!("WORDLE!");

    // this object is the state for the whole program
    let mut game = Game::new(wordle::dictionary::random_word());

    while game.status() == GameStatus::Active {
        print!("{} ", game.remaining_guesses());
        io::stdout().flush()?;

        let user_input = LegalWord::try_from(read_line()?);

        match user_input {
            Ok(prediction) => {
                // this is where the game state is updated.
                // the score of prediction is added to `game.scores`
                // the player knowledge of good and bad letters is updated
                game = game.update(prediction);

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
