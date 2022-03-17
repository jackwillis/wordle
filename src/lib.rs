pub mod dictionary;

use std::collections::BTreeSet;
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
/// use wordle::LetterScore::{NotPresent, PlacedCorrectly, PresentElsewhere};
///
/// let o = PresentElsewhere;
/// let x = PlacedCorrectly;
/// let u = NotPresent;
///
/// let mediocre_score = wordle::WordScore(vec![o, u, x, x, u]);
///
/// println!("{}", mediocre_score); //=> "O_XX_"
/// assert!(!mediocre_score.is_winner());
///
/// let winning_score = wordle::WordScore(vec![x, x, x, x, x]);
///
/// println!("{}", winning_score); //=> "XXXXX"
/// assert!(winning_score.is_winner());
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct WordScore(
    /// The hidden data of this struct.
    pub Vec<LetterScore>,
);

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

/// A legal word according to the rules of wordle.
///
/// This type guarantees that:
/// * The word is exactly five letters long.
/// * The word contains only letters from the English alphabet.
/// * The letters are uppercase.
///
/// ```rust
/// use wordle::LegalWord;
///
/// // The safe way to instantiate this type
/// let adieu_heap: Result<LegalWord, _> = LegalWord::try_from(String::from("ADIEU"));
///
/// // Here the program will panic if the input does not meet the constraints for the type.
/// let adieu_static = LegalWord::from("ADIEU");
///
/// assert_eq!(adieu_heap.unwrap(), adieu_static);
///
/// let invalid_word = LegalWord::try_from(String::from("onomatopeia"));
/// assert!(invalid_word.is_err());
/// ```
#[derive(Clone, Debug, PartialEq, derive_more::Display)]
pub struct LegalWord(String);

impl LegalWord {
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

impl TryFrom<String> for LegalWord {
    type Error = &'static str;

    /// Validates and creates a [LegalWord] at runtime.
    /// Normalizes to uppercase, so words have only one representation.
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() != 5 {
            Err("Word must be five letters long.")
        } else if value.chars().any(|c| !c.is_ascii_alphabetic()) {
            Err("Word must contain only letters from the English alphabet.")
        } else {
            let uppercased = value.chars().flat_map(|c| c.to_uppercase()).collect();
            Ok(LegalWord(uppercased))
        }
    }
}

/// Creates a playable word from a known good input.
/// Will panic if the input does not validate.
impl From<&str> for LegalWord {
    fn from(value: &str) -> Self {
        LegalWord::try_from(String::from(value)).unwrap()
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum GameStatus {
    Active,
    Lost,
    Won,
}

/// Represents the player's knowledge of "good" and "bad" letters.
#[derive(Clone)]
pub struct LetterKnowledge {
    pub good: BTreeSet<char>,
    pub bad: BTreeSet<char>,
    pub unknown: BTreeSet<char>,
}

impl LetterKnowledge {
    const ALPHABET: [char; 26] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];

    pub fn update(&self, secret_word: &LegalWord, prediction: &LegalWord) -> Self {
        let mut knowledge = self.clone();

        // Receive information from the letters in our guess
        for letter in prediction.letters() {
            // Each letter could be revealed as a "good" or "bad" letter
            // Good means the guessed letter is in the set of letters of the secret word.
            if secret_word.letters().any(|x| x == letter) {
                knowledge.good.insert(letter);
            } else {
                knowledge.bad.insert(letter);
            }

            // Any letter that's been played, whether good or bad, is no longer unknown.
            knowledge.unknown.remove(&letter);
        }

        knowledge
    }
}

impl Default for LetterKnowledge {
    fn default() -> Self {
        Self {
            good: BTreeSet::new(),
            bad: BTreeSet::new(),
            unknown: BTreeSet::from(LetterKnowledge::ALPHABET),
        }
    }
}

#[derive(Clone)]
pub struct Game {
    pub secret_word: LegalWord,
    pub scores: Vec<WordScore>,
    pub letter_knowledge: LetterKnowledge,
}

impl Game {
    const MAXIMUM_GUESSES: i32 = 6;

    pub fn new(secret_word: LegalWord) -> Game {
        Game {
            secret_word,
            scores: Vec::new(),
            letter_knowledge: LetterKnowledge::default(),
        }
    }

    pub fn update(&self, prediction: LegalWord) -> Self {
        let mut updated_game = self.clone();

        let score = self.secret_word.guess(&prediction);
        updated_game.scores.push(score);

        let updated_knowledge = self.letter_knowledge.update(&self.secret_word, &prediction);
        updated_game.letter_knowledge = updated_knowledge;

        updated_game
    }

    pub fn remaining_guesses(&self) -> usize {
        Game::MAXIMUM_GUESSES as usize - self.scores.len()
    }

    pub fn last_score(&self) -> Option<&WordScore> {
        self.scores.last()
    }

    pub fn status(&self) -> GameStatus {
        if self.last_score().is_some() && self.last_score().unwrap().is_winner() {
            GameStatus::Won
        } else if self.remaining_guesses() == 0 {
            GameStatus::Lost
        } else {
            GameStatus::Active
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LetterScore;

    #[test]
    fn formats_tile_guess_outcome() {
        assert_eq!(format!("{}", LetterScore::PlacedCorrectly), "X");
        assert_eq!(format!("{}", LetterScore::PresentElsewhere), "O");
        assert_eq!(format!("{}", LetterScore::NotPresent), "_");
    }

    use super::WordScore;

    fn correct_word_guess_outcome() -> WordScore {
        WordScore(vec![
            LetterScore::PlacedCorrectly,
            LetterScore::PlacedCorrectly,
            LetterScore::PlacedCorrectly,
            LetterScore::PlacedCorrectly,
            LetterScore::PlacedCorrectly,
        ])
    }

    fn incorrect_word_guess_outcome() -> WordScore {
        WordScore(vec![
            LetterScore::PlacedCorrectly,
            LetterScore::PlacedCorrectly,
            LetterScore::NotPresent,
            LetterScore::PresentElsewhere,
            LetterScore::NotPresent,
        ])
    }

    #[test]
    fn checks_correct_word_guess_outcome() {
        assert!(correct_word_guess_outcome().is_winner());
    }

    #[test]
    fn checks_incorrect_word_guess_outcome() {
        assert!(!incorrect_word_guess_outcome().is_winner());
    }

    #[test]
    fn formats_word_guess_outcome() {
        assert_eq!(format!("{}", incorrect_word_guess_outcome()), "XX_O_");
    }

    use super::LegalWord;

    #[test]
    fn formats_word() {
        assert_eq!(format!("{}", LegalWord::from("BRACK")), "BRACK");
    }

    #[test]
    fn accepts_valid_word_from_string() {
        assert_eq!(
            LegalWord::try_from(String::from("DRAKE")).unwrap(),
            LegalWord(String::from("DRAKE"))
        );
    }

    #[test]
    fn capitalizes_valid_word_from_string() {
        assert_eq!(
            LegalWord::try_from(String::from("Gumbo")).unwrap(),
            LegalWord(String::from("GUMBO"))
        );
    }

    #[test]
    fn rejects_short_word_from_string() {
        assert!(LegalWord::try_from(String::from("HEN")).is_err());
    }

    #[test]
    fn rejects_long_word_from_string() {
        assert!(LegalWord::try_from(String::from("PRAIRIE")).is_err());
    }

    #[test]
    fn rejects_non_basic_latin_word_from_string() {
        assert!(LegalWord::try_from(String::from("OBÉIR")).is_err());
    }

    #[test]
    fn rejects_blank_word_from_string() {
        assert!(LegalWord::try_from(String::new()).is_err());
    }

    #[test]
    fn accepts_correct_guess() {
        let word = LegalWord::from("JANUS");
        assert_eq!(word.guess(&word), correct_word_guess_outcome());
    }

    #[test]
    fn rejects_incorrect_guess() {
        let word = LegalWord::from("SPICE");
        let wrong_guess = LegalWord::from("SPACE");
        assert_ne!(word.guess(&wrong_guess), correct_word_guess_outcome());
    }

    #[test]
    fn evaluates_and_formats_guess() {
        let word = LegalWord::from("CRANE");
        let wrong_guess = LegalWord::from("BROWN");
        let guess = word.guess(&wrong_guess);
        assert_eq!(format!("{}", guess), "_X__O");
    }
}
