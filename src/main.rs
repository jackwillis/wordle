use std::collections::BTreeSet;
use std::io;
use std::io::Write;

use wordle::WordGuessOutcome;

#[derive(PartialEq)]
enum GameStatus {
    Active,
    Lost,
    Won,
}

#[derive(Clone, Debug)]
struct Game {
    secret_word: wordle::PlayableWord,
    guess_outcomes: Vec<WordGuessOutcome>,
    pub guessed_letters: BTreeSet<char>,
}

impl Game {
    const MAXIMUM_GUESSES: i32 = 6;

    pub fn new(secret_word: wordle::PlayableWord) -> Game {
        Game {
            secret_word,
            guess_outcomes: Vec::new(),
            guessed_letters: BTreeSet::new(),
        }
    }

    pub fn remaining_guesses(&self) -> usize {
        Game::MAXIMUM_GUESSES as usize - self.guess_outcomes.len()
    }

    pub fn play(&mut self, prediction: wordle::PlayableWord) {
        let guess_outcome = self.secret_word.guess(&prediction);
        self.guess_outcomes.push(guess_outcome.clone());

        prediction.tiles().for_each(|letter| {
            self.guessed_letters.insert(letter);
        });
    }

    pub fn last_outcome(&self) -> Option<&WordGuessOutcome> {
        self.guess_outcomes.last()
    }

    pub fn status(&self) -> GameStatus {
        if self.remaining_guesses() == 0 {
            if self.last_outcome().unwrap().is_correct() {
                GameStatus::Won
            } else {
                GameStatus::Lost
            }
        } else {
            GameStatus::Active
        }
    }
}

fn main() -> io::Result<()> {
    println!("WORDLE!");

    let secret_word = wordle::dictionary::random_word();
    let mut game = Game::new(secret_word);

    while game.status() == GameStatus::Active {
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
