use wordle::words::PlayableWord;

fn main() {
    let todays_word = PlayableWord::word("DRINK");
    let first_guess = PlayableWord::word("ADIEU");

    println!(
        "Today's word is:\n{}. Guess:\n{}. Result:\n{}",
        todays_word,
        first_guess,
        todays_word.guess(&first_guess)
    );
}
