use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Blob, BlobPropertyBag, Document, Element, HtmlElement, HtmlInputElement, Url};

use crate::dictionary::Dictionary;
use crate::game::scoring::{score_guess, LetterState};

const MAX_GUESSES: usize = 6;
const WORD_LENGTH: usize = 5;

thread_local! {
    static DICT: Dictionary = Dictionary::new();
}

#[wasm_bindgen]
pub struct GameUI {
    document: Document,
    grid: Element,
    message_el: Element,
    input_el: HtmlInputElement,
    submit_btn: HtmlElement,
    new_game_btn: HtmlElement,
    share_btn: HtmlElement,
    current_row: usize,
    answer: Option<String>,
    rows: Vec<(String, [LetterState; WORD_LENGTH])>,
}

#[wasm_bindgen]
impl GameUI {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<GameUI, JsValue> {
        let window = web_sys::window().ok_or_else(|| JsValue::from_str("no window"))?;
        let document = window
            .document()
            .ok_or_else(|| JsValue::from_str("no document"))?;
        let grid = document
            .get_element_by_id("grid")
            .ok_or_else(|| JsValue::from_str("no grid element"))?;
        let message_el = document
            .get_element_by_id("message")
            .ok_or_else(|| JsValue::from_str("no message element"))?;
        let input_el = document
            .get_element_by_id("guess-input")
            .ok_or_else(|| JsValue::from_str("no input field"))?
            .dyn_into::<HtmlInputElement>()
            .map_err(|_| JsValue::from_str("input is not HtmlInputElement"))?;
        let submit_btn = document
            .get_element_by_id("submit-btn")
            .ok_or_else(|| JsValue::from_str("no submit button"))?
            .dyn_into::<HtmlElement>()
            .map_err(|_| JsValue::from_str("submit-btn is not HtmlElement"))?;
        let new_game_btn = document
            .get_element_by_id("new-game-btn")
            .ok_or_else(|| JsValue::from_str("no new game button"))?
            .dyn_into::<HtmlElement>()
            .map_err(|_| JsValue::from_str("new-game-btn is not HtmlElement"))?;
        let share_btn = document
            .get_element_by_id("share-btn")
            .ok_or_else(|| JsValue::from_str("no share button"))?
            .dyn_into::<HtmlElement>()
            .map_err(|_| JsValue::from_str("share-btn is not HtmlElement"))?;

        Ok(GameUI {
            document,
            grid,
            message_el,
            input_el,
            submit_btn,
            new_game_btn,
            share_btn,
            current_row: 0,
            answer: None,
            rows: Vec::new(),
        })
    }

    pub fn submit_guess(&mut self, guess: &str) -> Result<JsValue, JsValue> {
        let guess = guess.trim().to_lowercase();
        if guess.len() != WORD_LENGTH {
            return Err(JsValue::from_str("guess must be 5 letters"));
        }
        if !guess.bytes().all(|b| b.is_ascii_lowercase()) {
            return Err(JsValue::from_str("guess must contain only letters"));
        }
        if self.current_row >= MAX_GUESSES {
            return Err(JsValue::from_str("no more guesses"));
        }

        let answer = self
            .answer
            .get_or_insert_with(|| DICT.with(|dict| dict.nearest_match(&guess).0))
            .clone();
        let score = score_guess(&guess, &answer);

        let row = self.render_row(&guess, &score, &answer)?;
        self.grid
            .append_child(&row)
            .map_err(|_| JsValue::from_str("failed to append row"))?;

        self.rows.push((guess.clone(), score));
        self.current_row += 1;

        let won = score.iter().all(|s| *s == LetterState::Green);
        let lost = !won && self.current_row >= MAX_GUESSES;

        if won {
            self.message_el.set_text_content(Some("You won!"));
            self.set_game_over();
        } else if lost {
            self.message_el
                .set_text_content(Some(&format!("You lost! The word was: {}", answer)));
            self.set_game_over();
        }

        let json = serde_json::to_string(&score.as_slice())
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(JsValue::from_str(&json))
    }

    pub fn new_game(&mut self) {
        while let Some(child) = self.grid.first_child() {
            self.grid.remove_child(&child).ok();
        }
        self.current_row = 0;
        self.answer = None;
        self.rows.clear();
        self.message_el.set_text_content(None);
        self.input_el.set_disabled(false);
        self.input_el.set_value("");
        self.submit_btn.remove_attribute("disabled").ok();
        self.share_btn.set_attribute("disabled", "disabled").ok();
        self.input_el.focus().ok();
    }

    pub fn share(&self) -> Result<(), JsValue> {
        let text = self.generate_grid_text();

        let window = web_sys::window().ok_or_else(|| JsValue::from_str("no window"))?;
        let navigator = window.navigator();

        let has_share = js_sys::Reflect::has(&navigator, &JsValue::from_str("share"))
            .unwrap_or(false);

        if has_share {
            let share_data = web_sys::ShareData::new();
            share_data.set_text(&text);
            let promise = navigator.share_with_data(&share_data);
            let future = JsFuture::from(promise);
            wasm_bindgen_futures::spawn_local(async move {
                future.await.ok();
            });
            return Ok(());
        }

        let svg = self.generate_svg();
        let arr = js_sys::Array::new();
        arr.push(&JsValue::from_str(&svg));
        let opts = BlobPropertyBag::new();
        opts.set_type("image/svg+xml");
        let blob = Blob::new_with_str_sequence_and_options(&arr, &opts)?;
        let url = Url::create_object_url_with_blob(&blob)?;

        let link = self.document.create_element("a")?;
        link.set_attribute("href", &url)?;
        link.set_attribute("download", "verba-ferri.svg")?;
        link.set_attribute("style", "display: none")?;
        let body = self
            .document
            .body()
            .ok_or_else(|| JsValue::from_str("no body"))?;
        body.append_child(&link)?;
        let html_link = link.dyn_into::<HtmlElement>()?;
        html_link.click();
        body.remove_child(&html_link)?;

        Ok(())
    }

    fn generate_grid_text(&self) -> String {
        let mut lines = String::from("Verba Ferri\n\n");
        for (_, score) in &self.rows {
            for state in score {
                match state {
                    LetterState::Green => lines.push('\u{1F7E9}'),
                    LetterState::Yellow => lines.push('\u{1F7E8}'),
                    LetterState::Grey => lines.push('\u{2B1B}'),
                }
            }
            lines.push('\n');
        }
        lines
    }

    fn set_game_over(&self) {
        self.input_el.set_disabled(true);
        self.submit_btn.set_attribute("disabled", "disabled").ok();
        self.share_btn.remove_attribute("disabled").ok();
    }

    fn render_row(
        &self,
        guess: &str,
        score: &[LetterState; WORD_LENGTH],
        _answer: &str,
    ) -> Result<Element, JsValue> {
        let row = self.document.create_element("div")?;
        row.set_class_name("row");

        for (i, ch) in guess.chars().enumerate() {
            let tile = self.document.create_element("div")?;
            tile.set_class_name("tile");
            tile.set_text_content(Some(&ch.to_string().to_uppercase()));

            let color = match score[i] {
                LetterState::Green => "#6aaa64",
                LetterState::Yellow => "#c9b458",
                LetterState::Grey => "#787c7e",
            };
            tile.set_attribute(
                "style",
                &format!("background-color: {}; color: white;", color),
            )?;

            row.append_child(&tile)?;
        }

        Ok(row)
    }

    fn generate_svg(&self) -> String {
        let ts = 40i32;
        let gap = 4i32;
        let pad = 8i32;
        let cols = 5i32;
        let rows = 6i32;
        let w = pad * 2 + cols * ts + (cols - 1) * gap;
        let h = pad * 2 + rows * ts + (rows - 1) * gap;

        let mut svg = format!(
            r##"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}">"##,
            w, h, w, h
        );
        svg.push_str(&format!(
            r##"<rect width="{}" height="{}" fill="#121213"/>"##,
            w, h
        ));

        for ri in 0..rows {
            for ci in 0..cols {
                let x = pad + ci * (ts + gap);
                let y = pad + ri * (ts + gap);
                let index = ri as usize;

                if index < self.rows.len() {
                    let (_, ref score) = self.rows[index];
                    let color = match score[ci as usize] {
                        LetterState::Green => "#6aaa64",
                        LetterState::Yellow => "#c9b458",
                        LetterState::Grey => "#787c7e",
                    };
                    svg.push_str(&format!(
                        r##"<rect x="{}" y="{}" width="{}" height="{}" rx="5" fill="{}"/>"##,
                        x, y, ts, ts, color,
                    ));
                } else {
                    svg.push_str(&format!(
                        r##"<rect x="{}" y="{}" width="{}" height="{}" rx="5" fill="none" stroke="#3a3a3c" stroke-width="2"/>"##,
                        x, y, ts, ts
                    ));
                }
            }
        }

        svg.push_str("</svg>");
        svg
    }
}

#[wasm_bindgen(start)]
pub fn init_ui() -> Result<(), JsValue> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let game_ui = Rc::new(std::cell::RefCell::new(GameUI::new()?));

    let gu = game_ui.clone();
    let submit_btn = gu.borrow().submit_btn.clone();
    let submit_closure = Closure::wrap(Box::new(move || {
        let guess = gu.borrow().input_el.value();
        if guess.len() == WORD_LENGTH {
            let result = gu.borrow_mut().submit_guess(&guess);
            match result {
                Ok(_) => gu.borrow().input_el.set_value(""),
                Err(e) => {
                    web_sys::console::log_1(&e);
                }
            }
        }
    }) as Box<dyn FnMut()>);
    submit_btn
        .add_event_listener_with_callback("click", submit_closure.as_ref().unchecked_ref())?;
    submit_closure.forget();

    let gu_for_input = game_ui.clone();
    let input_for_closure = game_ui.borrow().input_el.clone();
    let input_closure = Closure::wrap(Box::new(move || {
        let guess = input_for_closure.value();
        if guess.len() == WORD_LENGTH {
            let result = gu_for_input.borrow_mut().submit_guess(&guess);
            match result {
                Ok(_) => gu_for_input.borrow().input_el.set_value(""),
                Err(e) => {
                    web_sys::console::log_1(&e);
                }
            }
        }
    }) as Box<dyn FnMut()>);
    game_ui
        .borrow()
        .input_el
        .add_event_listener_with_callback("keydown", input_closure.as_ref().unchecked_ref())?;
    input_closure.forget();

    let gu = game_ui.clone();
    let new_game_btn = gu.borrow().new_game_btn.clone();
    let new_game_closure = Closure::wrap(Box::new(move || {
        gu.borrow_mut().new_game();
    }) as Box<dyn FnMut()>);
    new_game_btn
        .add_event_listener_with_callback("click", new_game_closure.as_ref().unchecked_ref())?;
    new_game_closure.forget();

    let gu = game_ui.clone();
    let share_btn = gu.borrow().share_btn.clone();
    let share_closure = Closure::wrap(Box::new(move || {
        if let Err(e) = gu.borrow().share() {
            web_sys::console::log_1(&e);
        }
    }) as Box<dyn FnMut()>);
    share_btn.add_event_listener_with_callback("click", share_closure.as_ref().unchecked_ref())?;
    share_closure.forget();

    Ok(())
}
