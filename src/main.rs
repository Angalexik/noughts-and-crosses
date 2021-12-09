use itertools::Itertools;
use rustyline::Editor;
use solver::Move;
use std::process::exit;
use std::time::Instant;
use std::env;

fn check_game_end(game: &solver::Game) {
    if game.board.over() {
        game.board.print(false);
        if game.board.has_won(solver::Player::X) {
            println!("X won")
        } else if game.board.has_won(solver::Player::O) {
            println!("O won")
        } else {
            println!("Draw 🙁")
        }

        exit(0);
    }
}

fn cpuplay(game: &mut solver::Game) {
    println!("Thinking time...");
    let now = Instant::now();
    let best_move = game.best_move();
    println!("Thinking took {}ms", now.elapsed().as_millis());
    game.placebit(best_move);
    check_game_end(&game);
    game.board.print(false);
}

fn main() {
    let game_kind = env::args().nth(1).unwrap();
    let args: Vec<u32> = env::args()
        .skip(2)
        .map(|a| a.parse::<u32>()
             .expect("First three arguments need to numbers"))
        .collect();
    // println!("{:#?}", args);
    let mut rl = Editor::<()>::new();
    // let mut game = solver::Game::new_xo(args[0], args[1], args[2]);
    let mut game = match game_kind.as_str() {
        "xo" => solver::Game::new_xo(args[0], args[1], args[2]),
        "c4" => solver::Game::new_connect_four(args[0], args[1], args[2]),
        _ => panic!()
    };

    if let Ok(answer) = rl.readline("X or O? ") {
        if answer.to_lowercase() == "o" {
            cpuplay(&mut game)
        }
    } else {
        exit(1);
    }

    loop {
        let readline = rl.readline(">> ");

        match readline {
            Ok(line) => {
                let mov: Move;
                if game_kind.as_str() == "xo" {
                    let pos: (u32, u32) = (line.splitn(2, '-') // converts "m-n" to (m, n) using rust magic
                        .map(|num| num.parse::<u32>().unwrap())
                        .collect_tuple())
                        .unwrap();
                    mov = game.pos_to_move(pos);
                } else {
                    mov = line.parse().unwrap();
                }
                if game.can_play(mov) {
                    game.placebit(mov);
                    check_game_end(&game);
                    cpuplay(&mut game);
                } else {
                    println!("you can't play that")
                }
            },
            Err(_) => {
                println!("oh no");
                break;
            },
        }
    }
}
