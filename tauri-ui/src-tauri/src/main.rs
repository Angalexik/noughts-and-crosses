#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod commands;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      commands::sanity_check,
      commands::place,
      commands::board,
      commands::reset,
      commands::can_play,
      commands::over,
      commands::place_best_move,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
