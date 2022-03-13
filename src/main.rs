use std::io;
use std::io::Write;

use wordle::GameStatus;

fn main() -> io::Result<()> {
    println!("WORDLE!");

    let secret_word = wordle::dictionary::random_word();
    let mut game = wordle::Game::new(secret_word);

    while game.status() == wordle::GameStatus::Active {
        print!("{} ", game.remaining_guesses());
        io::stdout().flush()?;

        let user_input = wordle::PlayableWord::try_from(read_line()?);

        match user_input {
            Ok(prediction) => {
                game.play(prediction);

                print!("  {} | ", game.last_outcome().unwrap());

                for c in &game.guessed_letters {
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
        GameStatus::Lost => println!("You lost. :("),
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
