# @@DISPLAY_NAME@@

*Words of Iron* — A Wordle parody where there is no secret word.

[@@SITE_URL@@](@@SITE_URL@@)

You type a five-letter guess. The game finds the dictionary word that best matches your guess, colouring tiles green (exact position), yellow (wrong position), or grey (not present). The illusion is seamless — it looks and feels exactly like Wordle, but the answer is always optimised to agree with you.

## How it works

### Vector space matching

Every word in the dictionary is encoded as a **130-dimensional one-hot vector** (5 positions × 26 letters). A `1.0` at index `pos * 26 + letter_idx` marks that a letter appears at that position; all other entries are `0.0`.

### Nearest-match algorithm

When you submit a guess:

1. The guess is encoded into the same 130-d vector space.
2. The dot product of the guess vector against every dictionary vector gives the **green count** (position-perfect letter matches).
3. The dictionary word with the highest dot product is the primary candidate.
4. **Tie-breaker:** if multiple words have the same green count, the one with more **yellows** (letters present but in a different position) wins.

The selected word is locked in as the "answer" for the rest of the game, exactly as if it had been chosen beforehand.

### Scoring

Standard Wordle rules apply:
- **Green** — letter is correct and in the right position.
- **Yellow** — letter is in the word but in a different position.
- **Grey** — letter is not in the word.

Duplicate letters are handled correctly: each instance in the answer can only be matched once.

## Tech stack

- **Language:** Rust
- **Compilation target:** WebAssembly (via `wasm-bindgen`)
- **DOM manipulation:** `web-sys` — no JavaScript framework
- **Serialisation:** `serde` / `serde_json`
- **Dictionary:** ~@@DICT_SIZE@@ five-letter words, embedded at compile time via `include_str!`

## Build & run

```bash
# Prerequisites: Rust + wasm-pack
source ~/.cargo/env

# Full build (generates config, WASM, www/)
./build-site.sh

# Serve www/ with any static file server
python3 -m http.server 8080 --directory www/
```

Open http://localhost:8080 in a browser.

## Project structure

```
@@PKG_NAME@@/
├── site_data/               # Marketing content (edit these to rebrand)
│   ├── features.json
│   ├── testimonials.json
│   └── badges.json
├── static/                  # Truly static assets
│   ├── style.css
│   └── icon.svg
├── templates/               # Files with @@PLACEHOLDER@@ substitutions
│   ├── index.html
│   ├── game.html
│   ├── sw.js
│   ├── manifest.json
│   └── README.md
├── build/
│   └── assemble.py          # Single build script (substitutes + assembles www/)
├── src/
│   ├── lib.rs              # WASM entry point, JS-exposed API
│   ├── dictionary/          # Dictionary loading + nearest-match search
│   ├── game/                # Vector encoding + Wordle-style scoring
│   │   ├── vector.rs        # 130-d one-hot encoding + dot product
│   │   └── scoring.rs       # Green / Yellow / Grey tile logic
│   └── ui/                  # DOM rendering (web-sys), event handlers
├── build.rs                # Generates config.rs from Cargo.toml metadata
├── Cargo.toml
└── www/                    # Flat deployable output (generated, gitignored)
```

## Forking

To rebrand this game as your own:

1. **Edit `Cargo.toml`** — change `name`, `display_name`, and `site_url` under `[package.metadata.site]`.
2. **Edit `site_data/*.json`** — your features, testimonials, and badges.
3. **Edit `src/dictionary/words.txt`** — replace the word list (optional).
4. **Edit `templates/`** — swap the icon, tweak the CSS, or change the legal blurb.
5. **Run `./build-site.sh`** — generates everything into `www/`.
6. **Deploy `www/`** to GitHub Pages, Netlify, or any static host.

The README, PWA manifest, service worker, marketing page, and game page all pick up your metadata automatically.

## Tests

```bash
cargo test                 # 17 unit tests
cargo clippy --all-targets -- -D warnings   # lint
cargo fmt -- --check       # formatting
```

## License

MIT
