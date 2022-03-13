pub mod dictionary;

use std::collections::BTreeSet;
use std::fmt;
use std::str::Chars;

/// Represents the outcome of guessing one tile.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TileGuessOutcome {
    /// The letter played *is* in the word, and was guessed in the *correct* position.
    PlacedCorrectly,
    /// The letter played *is* in the word, but was guessed in the *wrong* position.
    PresentElsewhere,
    /// The letter played is *not* in the word.
    NotPresent,
}

/// Format a tile guess outcome: X (green), O (yellow), or _ (gray)
impl fmt::Display for TileGuessOutcome {
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
pub struct WordGuessOutcome(pub Vec<TileGuessOutcome>);

impl WordGuessOutcome {
    /// Have all tiles been guessed correctly?
    pub fn is_correct(&self) -> bool {
        self.0
            .iter()
            .all(|x| x == &TileGuessOutcome::PlacedCorrectly)
    }
}

/// Format a word guess outcome by just printing the outcome all of its tiles.
impl fmt::Display for WordGuessOutcome {
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

/// Format a playable word by plainly printing the word.
impl fmt::Display for PlayableWord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Creates a playable word from user input which may be invalid.
/// Normalizes to uppercase.
impl TryFrom<String> for PlayableWord {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() != 5 {
            Err("Word must be five letters long.")
        } else if value.chars().any(|c| !c.is_ascii_alphabetic()) {
            Err("Word contains non--Latin letter values.")
        } else {
            // Store word as uppercase, so playable words have only one representation
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

impl PlayableWord {
    /// Returns an iterator over the letters of the word.
    pub fn tiles(&self) -> Chars {
        self.0.chars()
    }

    /// Returns the outcome of guessing a letter-tile at a given position
    fn guess_tile(&self, position: usize, tile: char) -> TileGuessOutcome {
        if self.tiles().nth(position) == Some(tile) {
            TileGuessOutcome::PlacedCorrectly
        } else if self.tiles().any(|x| x == tile) {
            TileGuessOutcome::PresentElsewhere
        } else {
            TileGuessOutcome::NotPresent
        }
    }

    /// Returns the outcome of a guess on a [`PlayableWord`].
    pub fn guess(&self, prediction: &Self) -> WordGuessOutcome {
        // Guess each tile then collect the result
        let tile_outcomes = prediction
            .tiles()
            .enumerate()
            .map(|(position, tile)| self.guess_tile(position, tile));

        WordGuessOutcome(tile_outcomes.collect())
    }
}

#[derive(PartialEq)]
pub enum GameStatus {
    Active,
    Lost,
    Won,
}

#[derive(Clone, Debug)]
pub struct Game {
    secret_word: PlayableWord,
    guess_outcomes: Vec<WordGuessOutcome>,
    pub guessed_letters: BTreeSet<char>,
}

impl Game {
    const MAXIMUM_GUESSES: i32 = 6;

    pub fn new(secret_word: PlayableWord) -> Game {
        Game {
            secret_word,
            guess_outcomes: Vec::new(),
            guessed_letters: BTreeSet::new(),
        }
    }

    pub fn remaining_guesses(&self) -> usize {
        Game::MAXIMUM_GUESSES as usize - self.guess_outcomes.len()
    }

    pub fn play(&mut self, prediction: PlayableWord) {
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

#[cfg(test)]
mod tests {
    use super::TileGuessOutcome;

    #[test]
    fn formats_tile_guess_outcome() {
        assert_eq!(format!("{}", TileGuessOutcome::PlacedCorrectly), "X");
        assert_eq!(format!("{}", TileGuessOutcome::PresentElsewhere), "O");
        assert_eq!(format!("{}", TileGuessOutcome::NotPresent), "_");
    }

    use super::WordGuessOutcome;

    fn correct_word_guess_outcome() -> WordGuessOutcome {
        WordGuessOutcome(vec![
            TileGuessOutcome::PlacedCorrectly,
            TileGuessOutcome::PlacedCorrectly,
            TileGuessOutcome::PlacedCorrectly,
            TileGuessOutcome::PlacedCorrectly,
            TileGuessOutcome::PlacedCorrectly,
        ])
    }

    fn incorrect_word_guess_outcome() -> WordGuessOutcome {
        WordGuessOutcome(vec![
            TileGuessOutcome::PlacedCorrectly,
            TileGuessOutcome::PlacedCorrectly,
            TileGuessOutcome::NotPresent,
            TileGuessOutcome::PresentElsewhere,
            TileGuessOutcome::NotPresent,
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
        assert_eq!(word.guess(&word), correct_word_guess_outcome());
    }

    #[test]
    fn rejects_incorrect_guess() {
        let word = PlayableWord::from("SPICE");
        let wrong_guess = PlayableWord::from("SPACE");
        assert_ne!(word.guess(&wrong_guess), correct_word_guess_outcome());
    }

    #[test]
    fn evaluates_and_formats_guess() {
        let word = PlayableWord::from("CRANE");
        let wrong_guess = PlayableWord::from("BROWN");
        let guess = word.guess(&wrong_guess);
        assert_eq!(format!("{}", guess), "_X__O");
    }
}
