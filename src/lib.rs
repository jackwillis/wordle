pub mod dictionary;

use std::collections::BTreeSet;
use std::fmt;
use std::str::Chars;

/// Represents the outcome of guessing one tile.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LetterComparison {
    /// The letter played *is* in the word, and was guessed in the *correct* position.
    PlacedCorrectly,
    /// The letter played *is* in the word, but was guessed in the *wrong* position.
    PresentElsewhere,
    /// The letter played is *not* in the word.
    NotPresent,
}

impl fmt::Display for LetterComparison {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::PlacedCorrectly => write!(f, "X"),
            Self::PresentElsewhere => write!(f, "O"),
            Self::NotPresent => write!(f, "_"),
        }
    }
}

/// Represents the outcome of guessing a word.
#[derive(Clone, Debug, PartialEq)]
pub struct WordComparison(pub Vec<LetterComparison>);

impl WordComparison {
    /// Have all tiles been guessed correctly?
    pub fn is_correct(&self) -> bool {
        let WordComparison(v) = self; // deconstruct sole unnamed struct item into `v`
        v.iter().all(|x| x == &LetterComparison::PlacedCorrectly)
    }
}

/// Format a word guess outcome by just printing the outcome all of its tiles.
impl fmt::Display for WordComparison {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for x in self.0.iter() {
            write!(f, "{}", x)?
        }
        Ok(())
    }
}

/// Represents a word that can be played in Wordle.
/// Must be exactly five letters long and contain only uppercase basic Latin letters.
#[derive(Clone, Debug, PartialEq)]
pub struct PlayableWord(String);

impl PlayableWord {
    /// Returns an iterator over the letters of the word.
    pub fn letters(&self) -> Chars {
        self.0.chars()
    }

    /// Returns the outcome of guessing a letter at a given position
    fn compare_letter(&self, position: usize, letter: char) -> LetterComparison {
        if self.letters().nth(position) == Some(letter) {
            LetterComparison::PlacedCorrectly
        } else if self.letters().any(|c| c == letter) {
            LetterComparison::PresentElsewhere
        } else {
            LetterComparison::NotPresent
        }
    }

    /// Returns the outcome of a guess on a [`PlayableWord`].
    pub fn compare_word(&self, prediction: &Self) -> WordComparison {
        // Guess each letter then collect the result
        let letter_comparisons = prediction
            .letters()
            .enumerate()
            .map(|(position, letter)| self.compare_letter(position, letter));

        WordComparison(letter_comparisons.collect())
    }
}

/// Format a playable word by plainly printing the word.
impl fmt::Display for PlayableWord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Creates a playable word from user input which may be invalid.
/// Normalizes to uppercase, so playable words have only one representation.
impl TryFrom<String> for PlayableWord {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() != 5 {
            Err("Word must be five letters long.")
        } else if value.chars().any(|c| !c.is_ascii_alphabetic()) {
            Err("Word must contain only letters from the English alphabet.")
        } else {
            let uppercased = value.chars().flat_map(|c| c.to_uppercase()).collect();
            Ok(PlayableWord(uppercased))
        }
    }
}

/// Creates a playable word from a known good input.
/// Will panic if the input does not validate.
impl From<&str> for PlayableWord {
    fn from(value: &str) -> Self {
        PlayableWord::try_from(String::from(value)).unwrap()
    }
}

#[derive(PartialEq)]
pub enum GameStatus {
    Active,
    Lost,
    Won,
}

#[derive(Clone, Debug)]
pub struct LetterKnowledge {
    pub correct: BTreeSet<char>,
    pub incorrect: BTreeSet<char>,
    pub unknown: BTreeSet<char>,
}

impl LetterKnowledge {
    const ALPHABET: [char; 26] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];

    pub fn update(&self, secret_word: &PlayableWord, prediction: &PlayableWord) -> Self {
        let mut updated_knowledge = self.clone();

        for letter in prediction.letters() {
            if secret_word.letters().any(|x| x == letter) {
                updated_knowledge.correct.insert(letter);
            } else {
                updated_knowledge.incorrect.insert(letter);
            }

            updated_knowledge.unknown.remove(&letter);
        }

        updated_knowledge
    }
}

impl Default for LetterKnowledge {
    fn default() -> Self {
        Self {
            correct: BTreeSet::new(),
            incorrect: BTreeSet::new(),
            unknown: BTreeSet::from(LetterKnowledge::ALPHABET),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Game {
    pub secret_word: PlayableWord,
    pub word_comparisons: Vec<WordComparison>,
    pub letter_knowledge: LetterKnowledge,
}

impl Game {
    const MAXIMUM_GUESSES: i32 = 6;

    pub fn new(secret_word: PlayableWord) -> Game {
        Game {
            secret_word,
            word_comparisons: Vec::new(),
            letter_knowledge: LetterKnowledge::default(),
        }
    }

    pub fn update(&self, prediction: PlayableWord) -> Self {
        let mut updated_game = self.clone();

        let guess_outcome = self.secret_word.compare_word(&prediction);
        updated_game.word_comparisons.push(guess_outcome);

        let updated_knowledge = self.letter_knowledge.update(&self.secret_word, &prediction);
        updated_game.letter_knowledge = updated_knowledge;

        updated_game
    }

    pub fn remaining_guesses(&self) -> usize {
        Game::MAXIMUM_GUESSES as usize - self.word_comparisons.len()
    }

    pub fn last_comparison(&self) -> Option<&WordComparison> {
        self.word_comparisons.last()
    }

    pub fn status(&self) -> GameStatus {
        if self.last_comparison().is_some() && self.last_comparison().unwrap().is_correct() {
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
    use super::LetterComparison;

    #[test]
    fn formats_tile_guess_outcome() {
        assert_eq!(format!("{}", LetterComparison::PlacedCorrectly), "X");
        assert_eq!(format!("{}", LetterComparison::PresentElsewhere), "O");
        assert_eq!(format!("{}", LetterComparison::NotPresent), "_");
    }

    use super::WordComparison;

    fn correct_word_guess_outcome() -> WordComparison {
        WordComparison(vec![
            LetterComparison::PlacedCorrectly,
            LetterComparison::PlacedCorrectly,
            LetterComparison::PlacedCorrectly,
            LetterComparison::PlacedCorrectly,
            LetterComparison::PlacedCorrectly,
        ])
    }

    fn incorrect_word_guess_outcome() -> WordComparison {
        WordComparison(vec![
            LetterComparison::PlacedCorrectly,
            LetterComparison::PlacedCorrectly,
            LetterComparison::NotPresent,
            LetterComparison::PresentElsewhere,
            LetterComparison::NotPresent,
        ])
    }

    #[test]
    fn checks_correct_word_guess_outcome() {
        assert!(correct_word_guess_outcome().is_correct());
    }

    #[test]
    fn checks_incorrect_word_guess_outcome() {
        assert!(!incorrect_word_guess_outcome().is_correct());
    }

    #[test]
    fn formats_word_guess_outcome() {
        assert_eq!(format!("{}", incorrect_word_guess_outcome()), "XX_O_");
    }

    use super::PlayableWord;

    #[test]
    fn formats_word() {
        assert_eq!(format!("{}", PlayableWord::from("BRACK")), "BRACK");
    }

    #[test]
    fn accepts_valid_word_from_string() {
        assert_eq!(
            PlayableWord::try_from(String::from("DRAKE")).unwrap(),
            PlayableWord(String::from("DRAKE"))
        );
    }

    #[test]
    fn capitalizes_valid_word_from_string() {
        assert_eq!(
            PlayableWord::try_from(String::from("Gumbo")).unwrap(),
            PlayableWord(String::from("GUMBO"))
        );
    }

    #[test]
    fn rejects_short_word_from_string() {
        assert!(PlayableWord::try_from(String::from("HEN")).is_err());
    }

    #[test]
    fn rejects_long_word_from_string() {
        assert!(PlayableWord::try_from(String::from("PRAIRIE")).is_err());
    }

    #[test]
    fn rejects_non_basic_latin_word_from_string() {
        assert!(PlayableWord::try_from(String::from("OBÉIR")).is_err());
    }

    #[test]
    fn rejects_blank_word_from_string() {
        assert!(PlayableWord::try_from(String::new()).is_err());
    }

    #[test]
    fn accepts_correct_guess() {
        let word = PlayableWord::from("JANUS");
        assert_eq!(word.compare_word(&word), correct_word_guess_outcome());
    }

    #[test]
    fn rejects_incorrect_guess() {
        let word = PlayableWord::from("SPICE");
        let wrong_guess = PlayableWord::from("SPACE");
        assert_ne!(
            word.compare_word(&wrong_guess),
            correct_word_guess_outcome()
        );
    }

    #[test]
    fn evaluates_and_formats_guess() {
        let word = PlayableWord::from("CRANE");
        let wrong_guess = PlayableWord::from("BROWN");
        let guess = word.compare_word(&wrong_guess);
        assert_eq!(format!("{}", guess), "_X__O");
    }
}
