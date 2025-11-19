const COMMANDS: &[&str] = &[
    "get_item",
    "put_item",
    "remove_item",
    "batch_get_item",
    "batch_put_item",
    "batch_remove_item",
    "batch_get_item_by_prefix",
    "batch_remove_item_by_prefix",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
