use solver::Game;

#[test]
fn test_heuristics() {
    let mut game = Game::new(3, 3, 3);

    game.place((0, 0));
    game.place((1, 0));
    game.place((0, 1));
    game.place((2, 0));

    assert_eq!(game.best_move(), game.pos_to_move((0, 2)));
}

#[test]
fn test_algo8() {
    let mut game = Game::new(4, 4, 3);

    game.best_move(); // Create transposition table
    game.place((1, 1));
    game.place((1, 0));

    game.board.print(true);

    assert_eq!(game.best_move(), game.pos_to_move((2, 1)));
}

#[test]
fn test_algo9() {
    let mut game = Game::new(4, 4, 3);

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
    let mut game = Game::new(3, 3, 3);

    game.place((0, 0));
    game.place((0, 2));
    assert_ne!(game.best_move(), game.pos_to_move((1, 1)));
}

#[test]
#[ignore]
fn profile_3_3_3() {
    let mut game = Game::new(3, 3, 3);

    game.best_move();
}

#[test]
#[ignore]
fn profile_4_4_3() {
    let mut game = Game::new(4, 4, 3);

    game.best_move();
}

#[test]
#[ignore]
fn profile_4_4_4() {
    let mut game = Game::new(4, 4, 4);

    game.best_move();
}

#[test]
#[ignore]
fn profile_5_5_3() {
    let mut game = Game::new(5, 5, 3);

    game.best_move();
}

#[test]
fn test_algo2() {
    let mut game = Game::new(3, 3, 3);

    // assert_eq!(board.internal_best_move(), (0, 0));
    game.place((0, 0)); // X
    game.place((1, 1)); // O
    game.place((0, 1)); // X
    assert_eq!(game.best_move(), game.pos_to_move((0, 2)));
}

#[test]
fn test_algo1() {
    let mut game = Game::new(3, 3, 3);

    game.place((0, 0));
    assert_eq!(game.best_move(), game.pos_to_move((1, 1)));
}

#[test]
fn test_algo3() {
    let mut game = Game::new(3, 3, 3);

    game.place((0, 0)); // X
    game.board.print(true);
    game.place((0, 1)); // O
    assert_ne!(game.best_move(), game.pos_to_move((0, 2)));
}

#[test]
fn test_algo4() {
    let mut game = Game::new(3, 3, 3);

    game.place((0, 0)); // X
    game.place((0, 1)); // O
    game.place((1, 1)); // X
    game.place((2, 2)); // O
    let best_move = game.best_move();
    assert!(best_move == game.pos_to_move((1, 0)) || best_move == game.pos_to_move((2, 0)));
}

#[test]
fn test_algo5() {
    let mut game = Game::new(3, 3, 3);

    game.place((0, 0)); // X
    game.place((1, 1)); // O
    game.place((0, 1)); // X
    game.place((0, 2)); // O
    game.board.print(true);

    assert_eq!(game.best_move(), game.pos_to_move((2, 0)));
}

#[test]
fn test_algo6() {
    let mut game = Game::new(3, 3, 3);

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
    let mut game = Game::new(3, 3, 3);

    game.place((1, 1));
    game.place((0, 0));
    game.place((0, 1));

    game.board.print(true);

    assert_eq!(game.best_move(), game.pos_to_move((2, 1)));
}
