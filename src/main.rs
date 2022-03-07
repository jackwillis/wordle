use wordle::words::PlayableWord;

fn main() {
    let todays_word = PlayableWord::from("DRINK");
    let first_guess = PlayableWord::from("ADIEU");

    println!(
        "Today's word is:\n{}. Guess:\n{}. Result:\n{}",
        todays_word,
        first_guess,
        todays_word.guess(&first_guess)
    );
}
