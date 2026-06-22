use std::fs;
use std::path::Path;

fn main() {
    let words_path = Path::new("src/dictionary/words.txt");
    if !words_path.exists() {
        return;
    }

    println!("cargo:rerun-if-changed=src/dictionary/words.txt");

    let content = fs::read_to_string(words_path).expect("Failed to read words.txt");
    let words: Vec<&str> = content
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .filter(|s| s.len() == 5)
        .collect();

    let html = generate_html(&words);
    fs::write(Path::new("static/word-list.html"), html)
        .expect("Failed to write static/word-list.html");
}

fn generate_html(words: &[&str]) -> String {
    let items: String = words
        .iter()
        .map(|w| {
            format!(
                "            <div class=\"word\">{}</div>\n",
                w.to_uppercase()
            )
        })
        .collect();

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Verba Ferri - Word List</title>
    <style>
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        body {{
            background-color: #121213;
            color: #ffffff;
            font-family: 'Clear Sans', 'Helvetica Neue', Arial, sans-serif;
            padding: 1rem;
        }}
        .back {{
            color: #787c7e;
            text-decoration: none;
            font-size: 0.85rem;
            display: block;
            margin-bottom: 0.5rem;
        }}
        .back:hover {{
            color: #c9b458;
        }}
        h1 {{
            font-size: 1.5rem;
            font-weight: 700;
            text-transform: uppercase;
            letter-spacing: 0.05em;
            margin-bottom: 1rem;
            text-align: center;
            color: #ffffff;
        }}
        .count {{
            text-align: center;
            color: #787c7e;
            margin-bottom: 1.5rem;
            font-size: 0.85rem;
        }}
        .container {{
            column-count: 5;
            column-gap: 2em;
            max-width: 900px;
            margin: 0 auto;
        }}
        .word {{
            break-inside: avoid;
            padding: 3px 0;
            font-family: monospace;
            font-size: 0.9rem;
            text-transform: uppercase;
            letter-spacing: 0.05em;
        }}
        @media (max-width: 600px) {{
            .container {{ column-count: 3; }}
        }}
    </style>
</head>
<body>
    <a href="../index.html" class="back">← Verba Ferri</a>
    <h1>Verba Ferri</h1>
    <div class="count">{words} words</div>
    <div class="container">
{items}
    </div>
</body>
</html>
"#,
        words = words.len(),
        items = items,
    )
}
