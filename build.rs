use std::fs;
use std::path::Path;

fn read_metadata() -> (String, String) {
    let content = fs::read_to_string("Cargo.toml").expect("Failed to read Cargo.toml");
    let mut display_name = String::from("Verba Ferri");
    let mut site_url = String::from("https://ferrous-lux.github.io/verba_ferri/");
    let mut in_site = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("[package.metadata.site]") {
            in_site = true;
            continue;
        }
        if in_site {
            if trimmed.starts_with('[') {
                break;
            }
            if let Some(val) = trimmed.strip_prefix("display_name = ") {
                display_name = val.trim_matches('"').to_string();
            }
            if let Some(val) = trimmed.strip_prefix("site_url = ") {
                site_url = val.trim_matches('"').to_string();
            }
        }
    }

    (display_name, site_url)
}

fn generate_config(display_name: &str, site_url: &str) {
    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR not set");
    let config_path = Path::new(&out_dir).join("config.rs");
    fs::write(
        &config_path,
        format!(
            "pub const DISPLAY_NAME: &str = \"{name}\";\npub const SITE_URL: &str = \"{url}\";\n",
            name = display_name,
            url = site_url,
        ),
    )
    .expect("Failed to write config.rs");
}

fn main() {
    if !Path::new("Cargo.toml").exists() {
        return;
    }

    println!("cargo:rerun-if-changed=Cargo.toml");

    let (display_name, site_url) = read_metadata();
    generate_config(&display_name, &site_url);
}
