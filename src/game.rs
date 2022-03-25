use std::collections::BTreeSet;

use crate::word::{Word, WordScore};

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

    pub fn update(&self, secret_word: &Word, prediction: &Word) -> Self {
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
    pub secret_word: Word,
    pub scores: Vec<WordScore>,
    pub letter_knowledge: LetterKnowledge,
}

impl Game {
    const MAXIMUM_GUESSES: i32 = 6;

    pub fn new(secret_word: Word) -> Game {
        Game {
            secret_word,
            scores: Vec::new(),
            letter_knowledge: LetterKnowledge::default(),
        }
    }

    pub fn add_prediction(&self, prediction: Word) -> Self {
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
