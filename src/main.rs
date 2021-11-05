mod solver;
use solver::Move;
use itertools::Itertools;
use rustyline::Editor;
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
    game.place(best_move);
    check_game_end(&game);
    game.board.print(false);
}

fn main() {
    let args: Vec<u32> = env::args()
        .skip(1)
        .map(|a| a.parse::<u32>()
             .expect("First three arguments need to numbers"))
        .collect();
    // println!("{:#?}", args);
    let mut rl = Editor::<()>::new();
    let mut game = solver::Game::new(args[0], args[1], args[2]);

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
                let mov: Move = line.splitn(2, '-') // converts "m-n" to (m, n) using rust magic
                    .map(|num| num.parse::<u32>().unwrap())
                    .collect_tuple()
                    .unwrap();
                if game.can_play(mov) {
                    game.place(mov);
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
