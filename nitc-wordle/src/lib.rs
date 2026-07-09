use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;
use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;

// Combined master list of all NITC words across all lengths
const WORD_BANK: &[&str] = &[
    "ragam", "eatya", "amphi", "milma", "exams", "autos", "proxy", "eclhc", "origo", "ignis", "profs",
    "tathva", "hostel",
    "rajpath", "calicut", "canteen", "unwired", "miraqui",
    "fosscell", "academic", "gdscclub", "dramaclub"
];

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
    word_length: usize,
}

#[wasm_bindgen]
impl WordleGame {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut rng = SmallRng::from_entropy();
        let random_index = rng.gen_range(0..WORD_BANK.len());
        let secret_word = WORD_BANK[random_index].to_string();
        let word_length = secret_word.len();

        Self {
            secret_word,
            guesses: Vec::new(),
            max_guesses: 6,
            word_length,
        }
    }

    pub fn get_word_length(&self) -> usize {
        self.word_length
    }

    pub fn submit_guess(&mut self, guess: String) -> Result<JsValue, JsValue> {
        let guess = guess.to_lowercase();
        
        if guess.len() != self.word_length {
            return Err(JsValue::from_str(&format!("Guess must be exactly {} letters long.", self.word_length)));
        }
        if self.guesses.len() >= self.max_guesses {
            return Err(JsValue::from_str("No guesses remaining."));
        }

        self.guesses.push(guess.clone());
        
        let mut results = Vec::new();
        let secret_chars: Vec<char> = self.secret_word.chars().collect();
        let guess_chars: Vec<char> = guess.chars().collect();

        for i in 0..self.word_length {
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

    pub fn get_secret_word(&self) -> String {
        self.secret_word.clone()
    }
}
