use std::fmt;
use std::str::Chars;

extern crate derive_more;

/// The player's score for one letter of a guess.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LetterScore {
    /// The correct letter was guessed in the correct spot.
    PlacedCorrectly,
    /// The letter was part of the word but not at that position.
    PresentElsewhere,
    /// The letter was not in the word.
    NotPresent,
}

impl fmt::Display for LetterScore {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::PlacedCorrectly => write!(f, "X"),
            Self::PresentElsewhere => write!(f, "O"),
            Self::NotPresent => write!(f, "_"),
        }
    }
}

/// Represents the player's score for guessing one word.
///
/// Wraps a [Vec] of five [LetterScore]s.
///
/// ```rust
/// use wordle::word::{LetterScore, WordScore};
///
/// let o = LetterScore::PresentElsewhere;
/// let x = LetterScore::PlacedCorrectly;
/// let u = LetterScore::NotPresent;
///
/// let mediocre_score = WordScore(vec![o, u, x, x, u]);
///
/// println!("{}", mediocre_score); //=> "O_XX_"
/// assert!(!mediocre_score.is_winner());
///
/// let winning_score = WordScore(vec![x, x, x, x, x]);
///
/// println!("{}", winning_score); //=> "XXXXX"
/// assert!(winning_score.is_winner());
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct WordScore(pub Vec<LetterScore>);

impl WordScore {
    /// Returns true if all letters have been guessed correctly.
    pub fn is_winner(&self) -> bool {
        self.0.iter().all(|x| x == &LetterScore::PlacedCorrectly)
    }
}

impl fmt::Display for WordScore {
    /// Concatenates all the [LetterScore]s.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for x in self.0.iter() {
            write!(f, "{}", x)?
        }
        Ok(())
    }
}

/// A legal word according to the rules of wordle. A newtype adding constraints on [String].
///
/// This type guarantees that:
/// * The word is exactly five letters long.
/// * The word contains only letters from the English alphabet.
/// * The letters are stored as uppercase.
///
/// ```rust
/// use wordle::Word;
///
/// let adieu: Word = Word::try_from("Adieu").unwrap();
///
/// // Words are normalized to uppercase
/// assert_eq!(String::from(adieu), "ADIEU");
///
/// // Invalid words are not allowed
/// let invalid_word: Result<Word, &str> = Word::try_from("onomatopeia");
/// assert!(invalid_word.is_err());
/// ```
#[derive(Clone, Debug, PartialEq, derive_more::Display, derive_more::Into)]
pub struct Word(String);

impl Word {
    /// Returns an iterator over the letters of the word.
    pub fn letters(&self) -> Chars {
        self.0.chars()
    }

    /// Compares two letters at a given position resulting in a [LetterScore].
    fn compare_letter(&self, position: usize, letter: char) -> LetterScore {
        if self.letters().nth(position) == Some(letter) {
            LetterScore::PlacedCorrectly
        } else if self.letters().any(|c| c == letter) {
            LetterScore::PresentElsewhere
        } else {
            LetterScore::NotPresent
        }
    }

    /// Compares a prediction against the secret word, resulting in a [WordScore].
    pub fn guess(&self, prediction: &Self) -> WordScore {
        // Guess each letter then collect the result
        let letter_scores = prediction
            .letters()
            .enumerate()
            .map(|(position, letter)| self.compare_letter(position, letter));

        WordScore(letter_scores.collect())
    }
}

impl TryFrom<String> for Word {
    type Error = &'static str;

    /// Validates and creates a [Word] at runtime.
    /// Normalizes to uppercase, so words have only one representation.
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() != 5 {
            Err("Word must be five letters long.")
        } else if value.chars().any(|c| !c.is_ascii_alphabetic()) {
            Err("Word must contain only letters from the English alphabet.")
        } else {
            let uppercased = value.chars().flat_map(|c| c.to_uppercase()).collect();
            Ok(Word(uppercased))
        }
    }
}

impl TryFrom<&str> for Word {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Word::try_from(value.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::LetterScore;

    const X: LetterScore = LetterScore::PlacedCorrectly;
    const O: LetterScore = LetterScore::PresentElsewhere;
    const U: LetterScore = LetterScore::NotPresent;

    #[test]
    fn formats_letter_score() {
        assert_eq!(format!("{}", X), "X");
        assert_eq!(format!("{}", O), "O");
        assert_eq!(format!("{}", U), "_");
    }

    use super::WordScore;

    #[test]
    fn checks_winning_word_score() {
        assert!(WordScore(vec![X, X, X, X, X]).is_winner());
    }

    #[test]
    fn checks_losing_word_score() {
        assert!(!WordScore(vec![X, X, U, O, U]).is_winner());
        assert!(!WordScore(vec![O, O, O, O, O]).is_winner());
        assert!(!WordScore(vec![U, U, U, U, U]).is_winner());
        assert!(!WordScore(vec![X, X, X, X, O]).is_winner());
        assert!(!WordScore(vec![U, X, X, X, X]).is_winner());
    }

    #[test]
    fn formats_word_score() {
        assert_eq!(format!("{}", WordScore(vec![X, O, U, O, O])), "XO_OO");
        assert_eq!(format!("{}", WordScore(vec![U, U, X, X, X])), "__XXX");
        assert_eq!(format!("{}", WordScore(vec![O, X, O, O, U])), "OXOO_");
    }

    use super::Word;

    #[test]
    fn creates_valid_word() {
        assert_eq!(String::from(Word::try_from("DRAKE").unwrap()), "DRAKE");
    }

    #[test]
    fn capitalizes_valid_word() {
        assert_eq!(String::from(Word::try_from("Gumbo").unwrap()), "GUMBO");
    }

    #[test]
    fn rejects_short_word_from_string() {
        assert!(Word::try_from("HEN").is_err());
    }

    #[test]
    fn rejects_long_word_from_string() {
        assert!(Word::try_from("PRAIRIE").is_err());
    }

    #[test]
    fn rejects_non_basic_latin_word_from_string() {
        assert!(Word::try_from("OBÉIR").is_err());
    }

    #[test]
    fn rejects_blank_word() {
        assert!(Word::try_from("").is_err());
    }

    #[test]
    fn formats_word() {
        assert_eq!(format!("{}", Word::try_from("BRACK").unwrap()), "BRACK");
    }

    #[test]
    fn accepts_correct_guess() {
        let word = Word::try_from("JANUS").unwrap();
        assert_eq!(word.guess(&word), WordScore(vec![X, X, X, X, X]));
    }

    #[test]
    fn rejects_incorrect_guess() {
        let word = Word::try_from("SPICE").unwrap();
        let wrong_guess = Word::try_from("SPACE").unwrap();
        assert_eq!(word.guess(&wrong_guess), WordScore(vec![X, X, U, X, X]));
    }

    #[test]
    fn evaluates_and_formats_guess() {
        let word = Word::try_from("CRANE").unwrap();
        let wrong_guess = Word::try_from("BROWN").unwrap();
        let guess = word.guess(&wrong_guess);
        assert_eq!(format!("{}", guess), "_X__O");
    }
}
