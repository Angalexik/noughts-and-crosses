use solver::{ Game, BoardKind };
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! { 
  static ref GAME: Mutex<Game> = Mutex::new(Game::new_xo(3, 3, 3)); // I only know what half of these things do
}

#[tauri::command]
pub fn place(row: u32, col: u32) {
  println!("yes!");
  GAME.lock().unwrap().place((row, col));
}

#[tauri::command]
pub fn board() -> String {
  GAME.lock().unwrap().render()
}

#[tauri::command]
pub fn reset(width: u32, height: u32, row: u32, kind: BoardKind) {
  GAME.lock().unwrap().reset(width, height, row, kind);
}

#[tauri::command]
pub fn can_play(row: u32, col: u32) -> bool {
  let game = GAME.lock().unwrap();
  let can_play = game.can_play(game.pos_to_move((row, col)));
  println!("{can_play}");
  can_play
}

#[tauri::command]
pub fn sanity_check() -> &'static str {
  println!("something");
  return "please";
}