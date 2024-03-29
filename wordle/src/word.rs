use std::fmt;
use std::str::Chars;
use std::str::FromStr;

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
/// ```rust
/// # use wordle::word::{LetterScore, WordScore};
/// let o = LetterScore::PresentElsewhere;
/// let x = LetterScore::PlacedCorrectly;
/// let u = LetterScore::NotPresent;
///
/// let not_a_winner = WordScore(vec![o, u, x, x, u]);
///
/// assert_eq!(format!("{}", not_a_winner), "O_XX_");
/// assert!(!not_a_winner.is_winner());
///
/// let winning_score = WordScore(vec![x, x, x, x, x]);
///
/// assert_eq!(format!("{}", winning_score), "XXXXX");
/// assert!(winning_score.is_winner());
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct WordScore(pub Vec<LetterScore>);

impl WordScore {
    /// Returns true if all letters have been guessed correctly.
    pub fn is_winner(&self) -> bool {
        self.0.iter().all(|&x| x == LetterScore::PlacedCorrectly)
    }
}

impl fmt::Display for WordScore {
    /// Concatenates all the [LetterScore]s.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for &x in &self.0 {
            write!(f, "{}", x)?;
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
/// use wordle::{Word, WordParseError};
///
/// let adieu: Word = "Adieu".parse::<Word>().unwrap();
///
/// // Words are normalized to uppercase
/// assert_eq!(adieu.to_string(), "ADIEU");
///
/// // Invalid words are not allowed
/// let invalid_word: Result<Word, WordParseError> = "onomatopeia".parse::<Word>();
/// assert!(invalid_word.is_err());
/// ```
#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug)]
pub enum WordParseError {
    InvalidLength,
    InvalidCharacters,
}

impl fmt::Display for WordParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLength => write!(f, "Word must be five letters long."),
            Self::InvalidCharacters => write!(
                f,
                "Word must contain only letters from the English alphabet."
            ),
        }
    }
}

/// implements [str::parse::<Word>]
impl FromStr for Word {
    type Err = WordParseError;

    /// Validates and creates a [Word] at runtime.
    /// Normalizes to uppercase, so words have only one representation.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().count() != 5 {
            Err(WordParseError::InvalidLength)
        } else if s.chars().any(|c| !c.is_ascii_alphabetic()) {
            Err(WordParseError::InvalidCharacters)
        } else {
            Ok(Word(s.to_uppercase()))
        }
    }
}

// implements `println!("{}", word)` and `let s: String = word.to_string()`
impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// implements `String::from(word)` and `let s: String = word.into()`
impl From<Word> for String {
    fn from(word: Word) -> Self {
        word.0
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
        let str = "DRAKE";
        let word = str.parse::<Word>().unwrap();

        assert_eq!(String::from(word), str);
    }

    #[test]
    fn capitalizes_valid_word() {
        let str = "Gumbo";
        let word = str.parse::<Word>().unwrap();

        assert_eq!(String::from(word), str.to_uppercase());
    }

    #[test]
    fn rejects_short_word_from_string() {
        assert!("HEN".parse::<Word>().is_err());
    }

    #[test]
    fn rejects_long_word_from_string() {
        assert!("PRAIRIE".parse::<Word>().is_err());
    }

    #[test]
    fn rejects_non_basic_latin_word_from_string() {
        assert!("OBÉIR".parse::<Word>().is_err());
    }

    #[test]
    fn rejects_blank_word() {
        assert!("".parse::<Word>().is_err());
    }

    #[test]
    fn formats_word() {
        let str = "BRACK";
        let word = str.parse::<Word>().unwrap();
        assert_eq!(format!("{}", word), str);
    }

    #[test]
    fn accepts_correct_guess() {
        let str = "JANUS";
        let word = str.parse::<Word>().unwrap();
        assert_eq!(word.guess(&word), WordScore(vec![X, X, X, X, X]));
    }

    #[test]
    fn rejects_incorrect_guess() {
        let word = "SPICE".parse::<Word>().unwrap();
        let wrong_guess = "SPACE".parse::<Word>().unwrap();
        assert_eq!(word.guess(&wrong_guess), WordScore(vec![X, X, U, X, X]));
    }

    #[test]
    fn evaluates_and_formats_guess() {
        let word = "CRANE".parse::<Word>().unwrap();
        let wrong_guess = "BROWN".parse::<Word>().unwrap();
        let guess = word.guess(&wrong_guess);
        assert_eq!(format!("{}", guess), "_X__O");
    }
}
