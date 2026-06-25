#!/usr/bin/env python3
"""Assemble www/ from templates, JSON data, and WASM build artifacts."""

import argparse
import json
import re
import shutil
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
def read_cargo_metadata():
    text = (ROOT / "Cargo.toml").read_text()
    def _get(pattern, text=text, flags=0):
        m = re.search(pattern, text, flags)
        if m is None:
            raise SystemExit(f"Could not find {pattern!r} in Cargo.toml")
        return m.group(1)

    pkg_name = _get(r'^name = "(.+)"', flags=re.MULTILINE)
    display_name = _get(r'display_name = "(.+)"')
    site_url = _get(r'site_url = "(.+)"')
    repo_url = _get(r'^repository = "(.+)"', flags=re.MULTILINE)
    return pkg_name, display_name, site_url, repo_url


def render_features(features):
    parts = []
    for f in features:
        parts.append(
            '<div class="feature">\n'
            f'  <h3>{f["title"]}</h3>\n'
            f'  <p>{f["description"]}</p>\n'
            '</div>'
        )
    return "\n".join(parts)


def render_testimonials(testimonials):
    parts = []
    for t in testimonials:
        parts.append(
            '<div class="testimonial">\n'
            f'  <div class="quote">"{t["quote"]}"</div>\n'
            f'  <div class="author">&mdash; {t["author"]}</div>\n'
            f'  <a href="./game.html" class="play-btn-small" '
            f'style="margin-bottom:0">\u25b8&nbsp; PLAY NOW</a>\n'
            '</div>'
        )
    return "\n\n".join(parts)


def render_badges(badges):
    inner = "\n".join(
        f'  <span class="badge">{b}</span>' for b in badges
    )
    return '<div class="badges">\n' + inner + '\n</div>'


def read_words():
    words = []
    for line in (ROOT / "src" / "dictionary" / "words.txt").read_text().splitlines():
        w = line.strip()
        if len(w) == 5:
            words.append(w.upper())
    return words


def write_word_list(words, display_name, dst):
    items = "\n".join(
        f'            <div class="word">{w}</div>' for w in words
    )
    html = f"""<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{display_name} - Word List</title>
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
    <a href="index.html" class="back">\u2190 {display_name}</a>
    <h1>{display_name}</h1>
    <div class="count">{len(words)} words</div>
    <div class="container">
{items}
    </div>
</body>
</html>
"""
    dst.write_text(html)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--output", default="www", help="Output directory (default: www)")
    args = parser.parse_args()

    out_dir = ROOT / args.output
    out_dir.mkdir(parents=True, exist_ok=True)

    pkg_name, display_name, site_url, repo_url = read_cargo_metadata()

    features = json.loads((ROOT / "site_data" / "features.json").read_text())
    testimonials = json.loads((ROOT / "site_data" / "testimonials.json").read_text())
    badges = json.loads((ROOT / "site_data" / "badges.json").read_text())
    words = read_words()

    inline_subs = {
        "@@DISPLAY_NAME@@": display_name,
        "@@SITE_URL@@": site_url,
        "@@PKG_NAME@@": pkg_name,
        "@@DICT_SIZE@@": str(len(words)),
        "@@REPO_URL@@": repo_url,
    }

    block_subs = {
        "@@FEATURES@@": render_features(features),
        "@@TESTIMONIALS@@": render_testimonials(testimonials),
        "@@BADGES@@": render_badges(badges),
    }

    def _replace_block(text, placeholder, value):
        def _replacer(m):
            prefix = m.group(1)
            return "\n".join(prefix + line for line in value.split("\n"))
        return re.sub(
            r'^([ \t]*)' + re.escape(placeholder) + r'[ \t]*$',
            _replacer,
            text,
            flags=re.MULTILINE,
        )

    def apply(text):
        for placeholder, value in block_subs.items():
            text = _replace_block(text, placeholder, value)
        for placeholder, value in inline_subs.items():
            text = text.replace(placeholder, value)
        return text

    for src_rel, dst in [
        ("templates/index.html", out_dir / "index.html"),
        ("templates/game.html", out_dir / "game.html"),
        ("templates/sw.js", out_dir / "sw.js"),
        ("templates/manifest.json", out_dir / "manifest.json"),
        ("templates/README.md", ROOT / "README.md"),
    ]:
        dst.write_text(apply((ROOT / src_rel).read_text()))

    for name in ["style.css", "icon.svg"]:
        shutil.copy2(ROOT / "static" / name, out_dir / name)

    write_word_list(words, display_name, out_dir / "word-list.html")

    pkg_src = ROOT / "pkg"
    for suffix in [".js", "_bg.wasm", ".d.ts", "_bg.wasm.d.ts"]:
        src_file = pkg_src / f"{pkg_name}{suffix}"
        if src_file.exists():
            shutil.copy2(src_file, out_dir / src_file.name)

    print(f"Assembled {args.output}/ for {display_name} ({site_url})")


if __name__ == "__main__":
    main()
