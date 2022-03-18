use crate::LegalWord;

use rand::seq::SliceRandom;

// Static dictionary. Every item is guaranteed to be a valid [LegalWord].
const DICTIONARY: &[&str] = &[
    "CIGAR", "REBUT", "SISSY", "HUMPH", "AWAKE", "BLUSH", "FOCAL", "EVADE", "NAVAL", "SERVE",
    "HEATH", "DWARF", "MODEL", "KARMA", "STINK", "GRADE", "QUIET", "BENCH", "ABATE", "FEIGN",
];

/// Chooses a random [LegalWord] from a static dictionary.
pub fn random_word() -> LegalWord {
    let word = *DICTIONARY.choose(&mut rand::thread_rng()).unwrap();
    LegalWord::try_from(word.to_owned()).unwrap()
}
