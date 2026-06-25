pub mod config;
pub mod dictionary;
pub mod game;

#[cfg(target_arch = "wasm32")]
pub mod ui;

use wasm_bindgen::prelude::*;

use dictionary::Dictionary;
use game::scoring::score_guess;

static DICT: once_cell::sync::Lazy<Dictionary> = once_cell::sync::Lazy::new(Dictionary::new);

#[wasm_bindgen]
pub fn submit_guess(guess: &str) -> Result<JsValue, JsValue> {
    let guess = guess.trim().to_lowercase();
    if guess.len() != 5 {
        return Err(JsValue::from_str("guess must be exactly 5 letters"));
    }
    if !guess.bytes().all(|b| b.is_ascii_lowercase()) {
        return Err(JsValue::from_str("guess must contain only letters a-z"));
    }

    let (answer, _) = DICT.nearest_match(&guess);
    let score = score_guess(&guess, &answer);

    serde_json::to_string(&score.as_slice())
        .map(|s| JsValue::from_str(&s))
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn dictionary_size() -> usize {
    DICT.len()
}
