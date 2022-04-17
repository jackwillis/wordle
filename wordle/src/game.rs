use std::collections::BTreeSet;

use crate::word::{Word, WordScore};

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

    /// The player should gain knowledge from the letters in their prediction.
    /// Each letter could be revealed as a "good" or "bad" letter --
    /// "good" meaning the letter is in the secret word.
    pub fn update(&self, secret_word: &Word, prediction: &Word) -> Self {
        let mut knowledge = self.clone();

        for letter in prediction.letters() {
            if secret_word.letters().any(|x| x == letter) {
                knowledge.good.insert(letter);
            } else {
                knowledge.bad.insert(letter);
            }
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

#[derive(Clone, Copy, PartialEq)]
pub enum GameStatus {
    Active,
    Lost,
    Won,
}

#[derive(Clone)]
pub struct Game {
    pub secret_word: Word,
    pub predictions: Vec<Word>,
    pub scores: Vec<WordScore>,
    pub letter_knowledge: LetterKnowledge,
}

impl Game {
    const MAXIMUM_GUESSES: i32 = 6;

    pub fn new(secret_word: Word) -> Game {
        Game {
            secret_word,
            predictions: Vec::new(),
            scores: Vec::new(),
            letter_knowledge: LetterKnowledge::default(),
        }
    }

    pub fn with_prediction(&self, prediction: Word) -> Self {
        let mut updated_game = self.clone();

        let updated_knowledge = self.letter_knowledge.update(&self.secret_word, &prediction);
        updated_game.letter_knowledge = updated_knowledge;

        let score = self.secret_word.guess(&prediction);
        updated_game.scores.push(score);
        updated_game.predictions.push(prediction);

        updated_game
    }

    pub fn remaining_guesses(&self) -> usize {
        Game::MAXIMUM_GUESSES as usize - self.scores.len()
    }

    pub fn last_score(&self) -> Option<&WordScore> {
        self.scores.last()
    }

    pub fn calculate_status(&self) -> GameStatus {
        match self.last_score() {
            Some(score) => {
                if score.is_winner() {
                    GameStatus::Won
                } else if self.remaining_guesses() == 0 {
                    GameStatus::Lost
                } else {
                    GameStatus::Active
                }
            }
            // no moves have been played yet
            None => GameStatus::Active,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{random_word, Game, GameStatus};

    #[test]
    fn test_new_game_is_active() {
        let game = Game::new(random_word());

        assert!(game.calculate_status() == GameStatus::Active);
    }
}
