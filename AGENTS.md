# Verba Ferri

A parody of Wordle where there is no secret word, but the UI is exactly the same as Wordle. A web-based game written in Rust compiled to WebAssembly.

A user is presented with an empty grid. They type in five letters and click "Submit". On the backend we check a small ~2000 word dictionary to see if there are matches or partial matches.

On the backend, we figure out which word matches closest to their entry to maximize green squares, then check if remainders could be yellow, and finally which cells will be grey.

## Build / Lint / Test Commands

```bash
# IMPORTANT: Always source cargo env first (local rust install is nonstandard)
source ~/.cargo/env

# Full site build (WASM + assets → www/)
./build-site.sh

# Quick dev server from www/
python3 -m http.server 8080 --directory www/

# Run all Rust tests
cargo test

# Run a single test by name
cargo test test_name_here

# Run tests for a specific package
cargo test -p my_crate

# Run WASM tests in headless browser
wasm-pack test --headless --firefox
# or
wasm-pack test --headless --chrome

# Run WASM tests with Node.js
wasm-pack test --node

# Lint (clippy)
cargo clippy --all-targets -- -D warnings

# Format check
cargo fmt -- --check

# Format (auto-fix)
cargo fmt

# Check compilation without producing artifacts
cargo check

# Audit dependencies for security vulnerabilities
cargo audit
```

## Project Structure

```
verba_ferri/
├── src/            # Rust source code (lib + binary)
│   ├── lib.rs      # Library root (WASM entry point)
│   ├── main.rs     # Binary entry point (if applicable)
│   ├── dictionary/ # Dictionary loading and word list
│   ├── game/       # Game logic (matching, scoring)
│   └── ui/         # UI rendering (DOM manipulation via web-sys)
├── tests/          # Integration tests
├── static/         # Static assets + generated word list
├── build-site.sh   # Full site build script (outputs to www/)
├── www/            # Deployable site root (generated, gitignored)
├── Cargo.toml      # Rust dependencies and metadata
├── index.html      # Marketing / splash page
├── game.html       # Game page
└── README.md
```

## Code Style Guidelines

### Imports

Group imports in blocks separated by blank lines, in this order:
1. `std` imports
2. External crate imports (e.g. `wasm_bindgen`, `web_sys`)
3. `crate` imports (internal modules)

```rust
use std::collections::HashSet;

use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

use crate::dictionary::Dictionary;
use crate::game::MatchResult;
```

### Formatting

- Use `cargo fmt` (rustfmt defaults). No tab width overrides — standard 4-space indentation.
- Max line length: 100 characters.
- Trailing commas on all multi-line struct/tuple/function-call expressions.
- Blank lines between function/struct/enum/impl items (one blank line).

### Naming Conventions

- **Types** (structs, enums, type aliases, traits): `PascalCase` — `MatchResult`, `LetterState`, `Dictionary`
- **Functions, methods, variables**: `snake_case` — `check_guess`, `word_list`, `best_match`
- **Constants**: `SCREAMING_SNAKE_CASE` — `MAX_GUESSES`, `WORD_LENGTH`
- **Module names**: `snake_case`, short and singular — `game`, `dictionary`, `ui`
- **Error types**: suffix with `Error` — `DictionaryError`, `ParseError`

### Types and Patterns

- Use `String`/`&str` conventions: `&str` for function parameters (borrowed), `String` for owned data.
- Prefer `Vec<T>` over slices in struct fields (owned).
- Use `HashSet<&str>` (with lifetimes) or `HashSet<String>` for the word dictionary.
- Use `#[wasm_bindgen]` on functions that JS needs to call. Keep WASM-exported functions thin — delegate logic to non-exported functions.
- Use `JsValue` for return types of public WASM functions, converting internally with `JsValue::from_str()` or serde.

### Error Handling

- Use `Result<T, String>` for WASM-exported functions (WASM-bindgen compatible).
- For internal logic, use `anyhow::Result` (or custom error enums) inside the crate.
- Prefer `thiserror` for defining structured error types in internal modules.
- Use `.context("descriptive message")` from `anyhow` to annotate errors.
- Avoid `unwrap()` and `expect()` in production code — propagate errors instead.
- `unwrap()` is acceptable only in test code or when the invariant is provably impossible and documented.

### WASM-specific Patterns

- Access the DOM via `web_sys` (not raw `js_sys` string manipulation).
- Attach event listeners using `Closure::wrap(Box::new(move || ...))` — store the closure to prevent GC.
- Use `#[wasm_bindgen(start)]` for an initialization function that runs on page load.
- Use `Rc<RefCell<>>` or `Cell` for interior mutability — WASM is single-threaded, so `Mutex` is unnecessary.
- For persistent game state: define a `struct GameState` wrapped in `Rc<RefCell<GameState>>` and pass it to closures.
- Minimize `unsafe` code. If `unsafe` is needed, add a `// SAFETY:` comment explaining why.

### Dictionary / Word Lists

- Store the word list as a static `&[&str]` or lazy-static `HashSet<&'static str>`.
- The dictionary has ~2000 five-letter words.
- Case-insensitive matching: normalize input to lowercase before lookup.
- Word matching algorithm:
  1. Find the dictionary word that maximizes green-square count for the user's guess.
  2. For remaining positions, check if the letter appears elsewhere in the candidate (yellow).
  3. Remaining cells are grey.

### CSS / UI

- Use CSS via a `<style>` tag in `index.html` or a separate `.css` file in `static/`.
- Match Wordle's visual design: green (`#6aaa64`), yellow (`#c9b458`), grey (`#787c7e`), dark tile background (`#121213`).
- The grid: 6 rows × 5 columns of square tiles.
- Keep UI logic (DOM manipulation) separate from game logic.

### Testing

- Unit tests: place `#[cfg(test)] mod tests { ... }` at the bottom of each module file.
- Integration tests: place in `tests/` directory, one file per concern.
- Use `wasm_bindgen_test::wasm_bindgen_test` for tests that need browser APIs.
- Name tests descriptively: `#[test] fn test_green_match_exact_word()`.
- Test edge cases: all grey, all green, partial matches, duplicate letters, short/long input.
