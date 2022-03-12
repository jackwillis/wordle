use std::io;
use std::io::Write;

use wordle::PlayableWord;

fn main() -> io::Result<()> {
    let todays_word = PlayableWord::from("DRINK");
    let mut remaining_guesses = 6;

    let stdin = io::stdin();

    println!("WORDLE!");

    while remaining_guesses != 0 {
        print!("{} ", remaining_guesses);
        io::stdout().flush()?;

        let mut buffer = String::new();
        stdin.read_line(&mut buffer)?;

        match PlayableWord::try_from(buffer.trim()) {
            Ok(predicted_word) => {
                let guess = todays_word.guess(&predicted_word);
                println!("  {}", guess);

                remaining_guesses -= 1;
            }
            Err(msg) => {
                println!("Input error: {}", msg);
            }
        }
    }

    Ok(())
}
