use std::fmt;
use std::str::Chars;

// use rand::seq::SliceRandom;
// use rand::thread_rng;

#[derive(Debug)]
struct PlayableWord(String);

impl fmt::Display for PlayableWord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Convert String to PlayableWord after validation+normalization
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

#[derive(Debug)]
enum TilePlacement {
    PlacedCorrectly,
    PresentElsewhere,
    NotPresent,
}

impl fmt::Display for TilePlacement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TilePlacement::PlacedCorrectly => write!(f, "X"),
            TilePlacement::PresentElsewhere => write!(f, "O"),
            TilePlacement::NotPresent => write!(f, "_"),
        }
    }
}

struct WordGuess(Vec<TilePlacement>);

impl fmt::Display for WordGuess {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for x in self.0.iter() {
            write!(f, "{}", x)?
        }
        Ok(())
    }
}

impl PlayableWord {
    fn chars(&self) -> Chars {
        self.0.chars()
    }

    fn guess(&self, other: &Self) -> WordGuess {
        let guess_tile = |(i, tile)| {
            if self.chars().nth(i).unwrap() == tile {
                TilePlacement::PlacedCorrectly
            } else if self.chars().any(|x| x == tile) {
                TilePlacement::PresentElsewhere
            } else {
                TilePlacement::NotPresent
            }
        };
        WordGuess(other.chars().enumerate().map(guess_tile).collect())
    }
}

#[allow(dead_code)]
const DICTIONARY: &'static [&'static str] = &[
    "CIGAR", "REBUT", "SISSY", "HUMPH", "AWAKE", "BLUSH", "FOCAL", "EVADE", "NAVAL", "SERVE",
    "HEATH", "DWARF", "MODEL", "KARMA", "STINK", "GRADE", "QUIET", "BENCH", "ABATE", "FEIGN",
];

// fn random_word() -> PlayableWord {
//     let mut rng = thread_rng();
//     let word_str = DICTIONARY.choose(&mut rng).unwrap();
//     PlayableWord::try_from(String::from(*word_str)).unwrap()
// }

fn main() {
    let todays_word = PlayableWord::try_from(String::from("DRINK")).unwrap();
    let first_guess = PlayableWord::try_from(String::from("ADIEU")).unwrap();

    println!(
        "Today's word is:\n{}. Guess:\n{}. Result:\n{}",
        todays_word,
        first_guess,
        todays_word.guess(&first_guess)
    );
}
