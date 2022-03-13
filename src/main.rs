use std::io;
use std::io::Write;

enum GameOutcome {
    Lost,
    Won,
}

static NUM_ALLOWED_GUESSES: i32 = 6;

fn main() -> io::Result<()> {
    let secret_word = wordle::dictionary::random_word();

    let mut game_outcome = GameOutcome::Lost;
    let mut remaining_guesses = NUM_ALLOWED_GUESSES;

    println!("WORDLE!");

    while remaining_guesses != 0 {
        print!("{} ", remaining_guesses);
        io::stdout().flush()?;

        let play = wordle::PlayableWord::try_from(read_line()?);

        match play {
            Ok(predicted_word) => {
                let guess = secret_word.guess(&predicted_word);
                println!("  {}", guess);

                if guess.is_correct() {
                    game_outcome = GameOutcome::Won;
                    break;
                }

                remaining_guesses -= 1;
            }
            Err(msg) => {
                println!("Input error: {}", msg);
            }
        }
    }

    match game_outcome {
        GameOutcome::Lost => println!("You lost. :("),
        GameOutcome::Won => println!("You're a winner, baby!"),
    }

    Ok(())
}

fn read_line() -> io::Result<String> {
    let mut input_buffer = String::new();
    io::stdin().read_line(&mut input_buffer)?;
    Ok(String::from(input_buffer.trim()))
}
