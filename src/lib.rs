use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum LetterState {
    Correct, // Green
    Present, // Yellow
    Absent,  // Gray
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct GuessResult {
    pub letter: char,
    pub state: LetterState,
}

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
        Self {
            secret_word: secret_word.to_lowercase(),
            guesses: Vec::new(),
            max_guesses: 6,
        }
    }

    pub fn submit_guess(&mut self, guess: String) -> Result<JsValue, JsValue> {
        let guess = guess.to_lowercase();
        
        if guess.len() != 5 {
            return Err(JsValue::from_str("Guess must be exactly 5 letters long."));
        }
        if self.guesses.len() >= self.max_guesses {
            return Err(JsValue::from_str("No guesses remaining."));
        }

        self.guesses.push(guess.clone());
        
        // Evaluate the guess against the secret word
        let mut results = Vec::new();
        let secret_chars: Vec<char> = self.secret_word.chars().collect();
        let guess_chars: Vec<char> = guess.chars().collect();

        for i in 0..5 {
            let ch = guess_chars[i];
            let state = if ch == secret_chars[i] {
                LetterState::Correct
            } else if secret_chars.contains(&ch) {
                LetterState::Present
            } else {
                LetterState::Absent
            };

            results.push(GuessResult { letter: ch, state });
        }

        serde_wasm_bindgen::to_value(&results)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    pub fn is_won(&self) -> bool {
        if let Some(last_guess) = self.guesses.last() {
            return last_guess == &self.secret_word;
        }
        false
    }

    pub fn is_game_over(&self) -> bool {
        self.is_won() || self.guesses.len() >= self.max_guesses
    }
}

#[wasm_bindgen]
impl WordleGame {
    pub fn get_word_length(&self) -> usize {
        self.secret_word.len()
    }

    pub fn get_secret_word(&self) -> String {
        self.secret_word.clone()
    }
}
