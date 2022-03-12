#![allow(dead_code)]

use crate::words::PlayableWord;
use rand::seq::SliceRandom;
use rand::thread_rng;

const DICTIONARY: &[&str] = &[
  "CIGAR", "REBUT", "SISSY", "HUMPH", "AWAKE", "BLUSH", "FOCAL", "EVADE", "NAVAL", "SERVE",
  "HEATH", "DWARF", "MODEL", "KARMA", "STINK", "GRADE", "QUIET", "BENCH", "ABATE", "FEIGN",
];

pub fn random_word() -> PlayableWord {
  let mut rng = thread_rng();
  let word_str = DICTIONARY.choose(&mut rng).unwrap();
  PlayableWord::from(*word_str)
}
