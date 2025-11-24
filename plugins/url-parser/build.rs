const COMMANDS: &[&str] = &["parse_url"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
