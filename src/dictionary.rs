use rand::seq::SliceRandom;
use std::str::FromStr;

use crate::Word;

// Static dictionary. Every item is guaranteed to be a valid [LegalWord].
const DICTIONARY: &[&str] = &[
    "CIGAR", "REBUT", "SISSY", "HUMPH", "AWAKE", "BLUSH", "FOCAL", "EVADE", "NAVAL", "SERVE",
    "HEATH", "DWARF", "MODEL", "KARMA", "STINK", "GRADE", "QUIET", "BENCH", "ABATE", "FEIGN",
];

/// Chooses a random [Word] from a static dictionary.
pub fn random_word() -> Word {
    let word = *DICTIONARY.choose(&mut rand::thread_rng()).unwrap();
    Word::from_str(word).unwrap()
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::DICTIONARY;
    use crate::Word;

    #[test]
    fn test_every_word_in_dictionary_is_valid() {
        for word in DICTIONARY {
            assert!(Word::from_str(word).is_ok());
        }
    }
}
