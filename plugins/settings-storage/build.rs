const COMMANDS: &[&str] = &["get_value", "update_value", "remove_value"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
