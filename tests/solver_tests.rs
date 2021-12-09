use arrayvec::ArrayVec;
use solver::{Moves, Game, Move};

macro_rules! arrayvec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = ArrayVec::<Move, 10>::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

#[test]
fn test_heuristics() {
    let mut game = Game::new_xo(3, 3, 3);

    game.place((0, 0));
    game.place((1, 0));
    game.place((0, 1));
    game.place((2, 0));

    assert_eq!(game.best_move(), game.pos_to_move((0, 2)));
}

#[test]
fn test_algo8() {
    let mut game = Game::new_xo(4, 4, 3);

    game.best_move(); // Create transposition table
    game.place((1, 1));
    game.place((1, 0));

    game.board.print(true);

    assert_eq!(game.best_move(), game.pos_to_move((2, 1)));
}

#[test]
fn test_algo9() {
    let mut game = Game::new_xo(4, 4, 3);

    game.best_move(); // Create transposition table
    game.place((0, 1));
    game.place((0, 2));
    game.place((2, 3));
    game.place((1, 2));

    game.board.print(true);

    assert_eq!(game.best_move(), game.pos_to_move((2, 2)));
}

#[test]
// Passes when transposition table is disabled
fn test_algo10() {
    let mut game = Game::new_xo(3, 3, 3);

    game.place((0, 0));
    game.place((0, 2));
    assert_ne!(game.best_move(), game.pos_to_move((1, 1)));
}

#[test]
#[ignore]
fn profile_3_3_3() {
    let mut game = Game::new_xo(3, 3, 3);

    game.best_move();
}

#[test]
#[ignore]
fn profile_4_4_3() {
    let mut game = Game::new_xo(4, 4, 3);

    game.best_move();
}

#[test]
#[ignore]
fn profile_4_4_4() {
    let mut game = Game::new_xo(4, 4, 4);

    game.best_move();
}

#[test]
#[ignore]
fn profile_5_5_3() {
    let mut game = Game::new_xo(5, 5, 3);

    game.best_move();
}

#[test]
fn test_algo2() {
    let mut game = Game::new_xo(3, 3, 3);

    // assert_eq!(board.internal_best_move(), (0, 0));
    game.place((0, 0)); // X
    game.place((1, 1)); // O
    game.place((0, 1)); // X
    assert_eq!(game.best_move(), game.pos_to_move((0, 2)));
}

#[test]
fn test_algo1() {
    let mut game = Game::new_xo(3, 3, 3);

    game.place((0, 0));
    assert_eq!(game.best_move(), game.pos_to_move((1, 1)));
}

#[test]
fn test_algo3() {
    let mut game = Game::new_xo(3, 3, 3);

    game.place((0, 0)); // X
    game.board.print(true);
    game.place((0, 1)); // O
    assert_ne!(game.best_move(), game.pos_to_move((0, 2)));
}

#[test]
fn test_algo4() {
    let mut game = Game::new_xo(3, 3, 3);

    game.place((0, 0)); // X
    game.place((0, 1)); // O
    game.place((1, 1)); // X
    game.place((2, 2)); // O
    let best_move = game.best_move();
    assert!(best_move == game.pos_to_move((1, 0)) || best_move == game.pos_to_move((2, 0)));
}

#[test]
fn test_algo5() {
    let mut game = Game::new_xo(3, 3, 3);

    game.place((0, 0)); // X
    game.place((1, 1)); // O
    game.place((0, 1)); // X
    game.place((0, 2)); // O
    game.board.print(true);

    assert_eq!(game.best_move(), game.pos_to_move((2, 0)));
}

#[test]
fn test_algo6() {
    let mut game = Game::new_xo(3, 3, 3);

    game.place((0, 0));
    game.place((1, 0));
    game.place((1, 1));
    game.place((2, 2));
    game.place((0, 1));
    game.place((0, 2));

    game.board.print(true);

    assert_eq!(game.best_move(), game.pos_to_move((2, 1)));
}

#[test]
fn test_algo7() {
    let mut game = Game::new_xo(3, 3, 3);

    game.place((1, 1));
    game.place((0, 0));
    game.place((0, 1));

    game.board.print(true);

    assert_eq!(game.best_move(), game.pos_to_move((2, 1)));
}

#[test]
fn test_not_over() {
    let mut game = Game::new_xo(3, 3, 3);

    game.place((2, 0));
    game.board.print(true);
    assert!(!game.board.over());

    game.place((1, 1));
    game.board.print(true);
    assert!(!game.board.over());

    game.place((0, 2));
    game.board.print(true);
    assert!(!game.board.over());
}

#[test]
fn test_connect_4_move_generation() {
    let game = Game::new_connect_four(3, 3, 3);

    assert_eq!(game.board.generate_moves(), Moves::C4Moves(arrayvec![0, 1, 2], 0))
}

#[test]
fn test_xo_moves() {
    let mut moves = Moves::XOMoves(0b11101110111, 0);
    assert_eq!(moves.next().unwrap(), 0b1);
    assert_eq!(moves.next().unwrap(), 0b10);
    assert_eq!(moves.next().unwrap(), 0b100);

    assert_eq!(moves.next().unwrap(), 0b10000);
    assert_eq!(moves.next().unwrap(), 0b100000);
    assert_eq!(moves.next().unwrap(), 0b1000000);
}

#[test]
fn test_c4_moves() {
    let mut moves = Moves::C4Moves(arrayvec![1,2,3,4], 0);
    assert_eq!(moves.next().unwrap(), 1);
    assert_eq!(moves.next().unwrap(), 2);
    assert_eq!(moves.next().unwrap(), 3);
    assert_eq!(moves.next().unwrap(), 4);
}

#[test]
fn test_c4_placement() {
    let mut game = Game::new_connect_four(3, 3, 3);

    game.board.placebit(0);
    assert_eq!(game.board.bitboards[0], 1);

    game.board.placebit(0);
    assert_eq!(game.board.bitboards[1], 0b10);
}

#[test]
fn test_c4_undo() {
    let mut game = Game::new_connect_four(3, 3, 3);

    game.board.placebit(2);
    assert_eq!(game.board.bitboards[0], 1 << 8);

    game.board.undo_move(2);
    assert_eq!(game.board.bitboards[0], 0);
}

#[test]
fn test_can_xo_play() {
    let mut game = Game::new_xo(3, 3, 3);

    assert!(game.can_play(1 << 2));
    game.place((0, 0));
    assert!(!game.can_play(1 << 2));
}

#[test]
fn test_can_c4_play() {
    let mut game = Game::new_connect_four(3, 3, 3);

    for _ in 0..3 {
        assert!(game.can_play(0));
        game.placebit(0);
    }

    assert!(!game.can_play(0));
}