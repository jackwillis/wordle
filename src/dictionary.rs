use crate::LegalWord;
use rand::seq::SliceRandom;

const DICTIONARY: &[&str] = &[
    "CIGAR", "REBUT", "SISSY", "HUMPH", "AWAKE", "BLUSH", "FOCAL", "EVADE", "NAVAL", "SERVE",
    "HEATH", "DWARF", "MODEL", "KARMA", "STINK", "GRADE", "QUIET", "BENCH", "ABATE", "FEIGN",
];

/// Chooses a random word from a dictionary, and converts into [LegalWord].
pub fn random_word() -> LegalWord {
    let mut rng = rand::thread_rng();
    let word_str = DICTIONARY.choose(&mut rng).unwrap();
    (*word_str).into()
}
