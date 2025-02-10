// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .max_blocking_threads(512)
        .thread_stack_size(20 * 1024 * 1024)
        .build()
        .unwrap()
        .block_on(async {
            tauri::async_runtime::set(tokio::runtime::Handle::current());
            desktop_lib::run();
        })
}
