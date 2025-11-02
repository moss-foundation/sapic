const COMMANDS: &[&str] = &["get_item", "put_item", "remove_item"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
