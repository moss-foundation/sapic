const COMMANDS: &[&str] = &[
    "get_value",
    "update_value",
    "remove_value",
    "batch_update_value",
    "batch_get_value",
    "batch_remove_value",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
