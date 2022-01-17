use solver::Game;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! { 
  static ref GAME: Mutex<Game> = Mutex::new(Game::new_xo(3, 3, 3)); // I only know what half of these things do
}

// #[tauri::command]
// pub fn place(row: u32, col: u32) {
//   println!("yes!");
//   GAME.lock().unwrap().place((row, col));
// }

// #[tauri::command]
// pub fn board() -> String {
//   GAME.lock().unwrap().place((0, 0));
//   GAME.lock().unwrap().render()
// }

#[tauri::command]
pub fn sanity_check() -> &'static str {
  println!("something");
  return "please";
}