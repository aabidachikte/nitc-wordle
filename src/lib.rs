use std::collections::HashMap;
use wasm_bindgen::prelude::*;

// =============================================================================
// Structs & Enums for JS Interoperability
// =============================================================================

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LetterState {
    Correct, // Green match
    Present, // Yellow match
    Absent,  // Grey match
}

#[wasm_bindgen]
pub struct GradedLetter {
    letter: char,
    state: String,
}

#[wasm_bindgen]
impl GradedLetter {
    #[wasm_bindgen(getter)]
    pub fn letter(&self) -> char {
        self.letter
    }

    #[wasm_bindgen(getter)]
    pub fn state(&self) -> String {
        self.state.clone()
    }
}

// =============================================================================
// Core Wordle Game State Machine
// =============================================================================

#[wasm_bindgen]
pub struct WordleGame {
    secret_word: String,
    guesses: Vec<String>,
    max_guesses: usize,
}

#[wasm_bindgen]
impl WordleGame {
    #[wasm_bindgen(constructor)]
    pub fn new(secret_word: String) -> Self {
        WordleGame {
            secret_word: secret_word.to_lowercase(),
            guesses: Vec::new(),
            max_guesses: 6,
        }
    }

    #[wasm_bindgen]
    pub fn get_word_length(&self) -> usize {
        self.secret_word.chars().count()
    }

    #[wasm_bindgen]
    pub fn get_secret_word(&self) -> String {
        self.secret_word.clone()
    }

    #[wasm_bindgen]
    pub fn is_won(&self) -> bool {
        if let Some(last_guess) = self.guesses.last() {
            return last_guess == &self.secret_word;
        }
        false
    }

    #[wasm_bindgen]
    pub fn is_game_over(&self) -> bool {
        self.is_won() || self.guesses.len() >= self.max_guesses
    }

    // =========================================================================
    // Updated submit_guess: Implements Two-Pass Budget Allocation
    // =========================================================================
    #[wasm_bindgen]
    pub fn submit_guess(&mut self, guess: String) -> Result<Vec<GradedLetter>, String> {
        let guess_lower = guess.to_lowercase();
        let word_len = self.get_word_length();

        if guess_lower.chars().count() != word_len {
            return Err(format!("Guess must be exactly {} letters.", word_len));
        }

        if self.is_game_over() {
            return Err("Game is already over!".to_string());
        }

        self.guesses.push(guess_lower.clone());

        let secret_chars: Vec<char> = self.secret_word.chars().collect();
        let guess_chars: Vec<char> = guess_lower.chars().collect();

        let mut states = vec![LetterState::Absent; word_len];
        let mut secret_letter_counts = HashMap::new();

        // Pass 1.1: Record total occurrences of each letter in the secret target
        for &c in &secret_chars {
            *secret_letter_counts.entry(c).or_insert(0) += 1;
        }

        // Pass 1.2: Lock in exact position matches (GREEN) first and reduce budget
        for i in 0..word_len {
            if guess_chars[i] == secret_chars[i] {
                states[i] = LetterState::Correct;
                if let Some(count) = secret_letter_counts.get_mut(&guess_chars[i]) {
                    *count -= 1;
                }
            }
        }

        // Pass 2: Process remaining letters left-to-right (YELLOW vs. GREY)
        for i in 0..word_len {
            if states[i] == LetterState::Correct {
                continue; // Skip already verified greens
            }

            let current_char = guess_chars[i];
            if let Some(count) = secret_letter_counts.get_mut(&current_char) {
                if *count > 0 {
                    states[i] = LetterState::Present; // Yellow (Letter exists elsewhere & budget is free)
                    *count -= 1;
                } else {
                    states[i] = LetterState::Absent;  // Grey (Out of duplicate letter allowance)
                }
            } else {
                states[i] = LetterState::Absent;      // Grey (Letter completely missing)
            }
        }

        // Convert evaluations into Javascript-binded structs
        let graded_letters = states
            .into_iter()
            .enumerate()
            .map(|(i, state)| {
                let state_str = match state {
                    LetterState::Correct => "Correct".to_string(),
                    LetterState::Present => "Present".to_string(),
                    LetterState::Absent => "Absent".to_string(),
                };
                GradedLetter {
                    letter: guess_chars[i],
                    state: state_str,
                }
            })
            .collect();

        Ok(graded_letters)
    }
}
