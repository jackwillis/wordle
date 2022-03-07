use std::fmt;
use std::str::Chars;

/// Represents the outcome of one tile in a guess.
#[derive(Debug, PartialEq)]
pub enum TileGuessOutcome {
  /// The letter played is in the word and in the correct position.
  PlacedCorrectly,
  /// The letter played is in the word but in the wrong position.
  PresentElsewhere,
  /// The letter played is not in the word.
  NotPresent,
}

impl fmt::Display for TileGuessOutcome {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::PlacedCorrectly => write!(f, "X"),
      Self::PresentElsewhere => write!(f, "O"),
      Self::NotPresent => write!(f, "_"),
    }
  }
}

/// Represents the outcome of a guess.
pub struct WordGuessOutcome(Vec<TileGuessOutcome>);

impl WordGuessOutcome {
  #[allow(dead_code)]
  pub fn is_correct(&self) -> bool {
    self
      .0
      .iter()
      .all(|x| x == &TileGuessOutcome::PlacedCorrectly)
  }
}

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
#[derive(Debug)]
pub struct PlayableWord(String);

impl fmt::Display for PlayableWord {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

/// Creates a [`PlayableWord`] safely, returning a [`Result`].
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

impl PlayableWord {
  /// Creates a [`PlayableWord`] from a known good input.
  /// Will panic if the input does not validate.
  /// Wraps [`PlayableWord::try_from<String>()`].
  pub fn word(word_str: &str) -> PlayableWord {
    PlayableWord::try_from(String::from(word_str)).unwrap()
  }

  /// Returns an iterator over the letters of the word.
  fn chars(&self) -> Chars {
    self.0.chars()
  }

  /// Returns the *n*th letter in the word.
  fn nth(&self, n: usize) -> Option<char> {
    self.chars().nth(n)
  }

  /// Returns the outcome of a guess on a [`PlayableWord`].
  pub fn guess(&self, other: &Self) -> WordGuessOutcome {
    let guess_tile = |(i, tile)| {
      if self.nth(i).unwrap() == tile {
        TileGuessOutcome::PlacedCorrectly
      } else if self.chars().any(|x| x == tile) {
        TileGuessOutcome::PresentElsewhere
      } else {
        TileGuessOutcome::NotPresent
      }
    };
    WordGuessOutcome(other.chars().enumerate().map(guess_tile).collect())
  }
}
