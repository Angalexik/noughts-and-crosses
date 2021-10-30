use std::{cmp::{max, min}, ops::Not};
use ahash::AHashMap;

type Bitboard = u128; // Maximum board size is 10x10
pub type Move = (u32, u32);

const INFINITY: i32 = i32::MAX;
const NEGINFINITY: i32 = i32::MIN + 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Player {
    X = 0,
    O = 1,
}

impl Not for Player {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Player::O => Player::X,
            Player::X => Player::O,
        }
    }
}

pub struct Game {
    pub board: Board,
    solver: Solver,
}

impl Game {
    pub fn new(width: u32, height: u32, row: u32) -> Game {
        Game {
            board: Board {
                width,
                height,
                row,
                bitboards: [0, 0],
                player: Player::X,
            },
            solver: Solver {
                transpostions: AHashMap::new(),
            }
        }
    }

    pub fn place(&mut self, mov: Move) {
        self.board.place(mov);
    }

    pub fn best_move(&mut self) -> Move {
        self.solver.best_move(&self.board)
    }
}

#[derive(Clone, Copy)]
pub struct Board {
    width: u32,
    height: u32,
    row: u32,
    bitboards: [Bitboard; 2],
    player: Player
}

impl Board {
    pub fn get_index(&self, row: u32, column: u32) -> u32 {
        let height = self.height + 1;
        let row = row + 1;
        height - 1 - row + (column * height)
    }

    pub fn render(&self, debug: bool) -> String {
        let mut lines = String::new();
        for row in 0..self.height {
            let mut line = String::new();
            for col in 0..self.width {
                let mask = 1 << self.get_index(row, col);
                // line.push_str(&format!("{:02} ", self.get_index(row, col)))

                if self.bitboards[Player::X as usize] & mask != 0 {
                    line.push('X');
                    continue;
                }

                if self.bitboards[Player::O as usize] & mask != 0 {
                    line.push('O');
                    continue;
                }

                line.push('.');
            }
            lines.push_str(&line);
            lines.push('\n');
        }

        if debug {
            lines.push('\n');
            lines.push_str(&format!("X: {:b}\n", self.bitboards[Player::X as usize]));
            lines.push_str(&format!("O: {:b}", self.bitboards[Player::O as usize]));
        }

        lines
    }

    pub fn print(&self, debug: bool) {
        println!("{}", self.render(debug));
    }

    pub fn occupied(&self, idx: u32) -> bool {
        let mask = 1 << idx;
        (self.bitboards[0] | self.bitboards[1]) & mask != 0
        // (self.bitboards[Player::X as usize] & mask) >> idx != 0 || (self.bitboards[Player::O as usize] & mask) >> idx != 0
    }

    pub fn has_won(&self, player: Player) -> bool {
        // Modified version of https://github.com/qu1j0t3/fhourstones/blob/bf0e70ed9fe8128eeea8539f17dd41826f2cc6b6/Game.c#L108
        let bitboard = self.bitboards[player as usize];
        let delta = self.row - 2;
        let vert = bitboard & (bitboard >> 1);
        let hori = bitboard & (bitboard >> (self.height + 1));
        let diag1 = bitboard & (bitboard >> self.height);
        let diag2 = bitboard & (bitboard >> (self.height + 2));
        (vert & (vert >> delta)) |
            (hori & (hori >> delta * (self.height + 1))) |
            (diag1 & (diag1 >> delta * self.height)) |
            (diag2 & (diag2 >> delta * (self.height + 2))) != 0
    }

    pub fn over(&self) -> bool {
        self.has_won(Player::X) ||
            self.has_won(Player::O) ||
            self.draw()
    }


    pub fn remove(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        if self.bitboards[0] & (1 << idx) != 0 || self.bitboards[1] & (1 << idx) != 0 {
            for i in 0..2 {
                self.bitboards[i] &= !(1 << idx);
            }
        } else {
            panic!();
        }
    }

    // „borrowed“ from https://github.com/PascalPons/connect4/blob/part9/position.hpp#L226
    pub fn asdf(&self, player: Player) -> u32 {
        // vertical
        let mut r = self.bitboards[player as usize] << 1;
        // for i in range(2, self.row):
        //     r &= self.bitboards[player as usize] << i
        for i in 2..self.row {
            r &= self.bitboards[player as usize] << i;
        }
        // r = (self.bitboards[player as usize] << 1) & (self.bitboards[player as usize] << 2)// & (self.bitboards[player as usize] << 3)

        // horizontal
        let mut p = (self.bitboards[player as usize] << (self.height+1)) & (self.bitboards[player as usize] << (self.row-2)*(self.height+1));
        r |= p & (self.bitboards[player as usize] << (self.row-1)*(self.height+1));
        r |= p & (self.bitboards[player as usize] >> (self.height+1));
        p = (self.bitboards[player as usize] >> (self.height+1)) & (self.bitboards[player as usize] >> (self.row-2)*(self.height+1));
        r |= p & (self.bitboards[player as usize] << (self.height+1));
        r |= p & (self.bitboards[player as usize] >> (self.row-1)*(self.height+1));

        // diagonal 1
        p = (self.bitboards[player as usize] << self.height) & (self.bitboards[player as usize] << (self.row-2)*self.height);
        r |= p & (self.bitboards[player as usize] << (self.row-1)*self.height);
        r |= p & (self.bitboards[player as usize] >> self.height);
        p = (self.bitboards[player as usize] >> self.height) & (self.bitboards[player as usize] >> (self.row-2)*self.height);
        r |= p & (self.bitboards[player as usize] << self.height);
        r |= p & (self.bitboards[player as usize] >> (self.row-1)*self.height);

        // diagonal 2
        p = (self.bitboards[player as usize] << (self.height+2)) & (self.bitboards[player as usize] << (self.row-2)*(self.height+2));
        r |= p & (self.bitboards[player as usize] << (self.row-1)*(self.height+2));
        r |= p & (self.bitboards[player as usize] >> (self.height+2));
        p = (self.bitboards[player as usize] >> (self.height+2)) & (self.bitboards[player as usize] >> (self.row-2)*(self.height+2));
        r |= p & (self.bitboards[player as usize] << (self.height+2));
        r |= p & (self.bitboards[player as usize] >> (self.row-1)*(self.height+2));

        return (r & (!self.bitboards[!player as usize])).count_ones();
        // return r
    }
}

impl Board {
    pub fn new(width: u32, height: u32, row: u32) -> Board {
        Board {
            width,
            height,
            row,
            bitboards: [0, 0],
            player: Player::X
        }
    }

    pub fn place(&mut self, mov: Move) -> bool {
        let idx = self.get_index(mov.0, mov.1);
        self.placeidx(idx)
    }

    pub fn draw(&self) -> bool {
        (self.bitboards[Player::X as usize] | self.bitboards[Player::O as usize])
            .count_ones() == self.height * self.width && !self.has_won(Player::X) && !self.has_won(Player::O)
    }

    pub fn placeidx(&mut self, idx: u32) -> bool {
        let mask = 1 << idx;
        let mut won = self.has_won(Player::X) || self.has_won(Player::O);
        if !self.occupied(idx) && !won {
            self.bitboards[self.player as usize] |= mask;
            won = self.has_won(self.player);
            self.player = match self.player {
                Player::X => Player::O,
                Player::O => Player::X,
            };
        } else {
            panic!()
        }
        won
    }
}

enum ScoreType {
    Exact,
    LowerBound,
    UpperBound,
}

struct Score {
    value: i32,
    flag: ScoreType,
}

struct Solver {
    transpostions: AHashMap<([Bitboard; 2], i32), Score>
}

impl Solver {
    fn generate_moves(&self, board: &Board) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        for row in 0..board.height {
            for col in 0..board.width {
                if !board.occupied(board.get_index(row, col)) {
                    moves.push((row, col))
                }
            }
        }
        moves
    }

    fn score_move(&self, board: &mut Board, mov: &Move) -> i32 {
        if board.asdf(!board.player) > 1 {
            return NEGINFINITY;
        }
        board.place(*mov);
        let score = board.asdf(board.player);
        board.remove(mov.0, mov.1);
        score as i32
    }

    // fn order_moves(board: &mut board, moves: Vec<Move>) -> Vec<Move> {
    //     let scores: Vec<i32> = moves.into_iter().map(|mov| score_move(board, &mov)).collect();
    // }
    
    fn best_move(&mut self, board: &Board) -> Move {
        let player = match board.player {
            Player::O => -1,
            Player::X => 1,
        };
        let mut best_score = NEGINFINITY;
        let mut best_move: Option<Move> = None;
        let moves = self.generate_moves(board);
        // moves.sort_by(|a, b| score_move(self, b).cmp(&score_move(self, a)));
        // moves.sort_by_cached_key(|a| score_move(self, a));
        // moves.reverse();
        for mov in moves {
            // log!("{}-{}", row, col);
            let mut board2 = board.clone();
            board2.place(mov);
            let score = -self.negamax(board2, INFINITY, NEGINFINITY, INFINITY, player as i8);
            // println!("{:?} score: {}", mov, score);
            if score > best_score {
                best_score = score;
                best_move = Some(mov)
            }
        }
        // println!("Evaluation: {}", match best_score.cmp(&0) {
        //     Ordering::Equal => "draw".to_string(),
        //     Ordering::Greater => format!("X wins in {} moves", (best_score - INFINITY) * -1),
        //     Ordering::Less => format!("O wins in {} moves", best_score * -1),
        // });
        best_move.expect("No move was chosen")
    }

    fn negamax(&mut self, board: Board, depth: i32, mut alpha: i32, mut beta: i32, player: i8) -> i32 {
        let orig_alpha = alpha;
        if /*depth == 0 ||*/ board.over() {
            if board.draw() {
                return 0;
            }

            if board.has_won(Player::X) {
                // return (INFINITY - 1) * (player as i32)
                return -(depth) * (player as i32);
            }

            if board.has_won(Player::O) {
                // return (NEGINFINITY + 1) * (player as i32);
                return (depth) * (player as i32);
            }

            panic!();
        }


        if self.transpostions.contains_key(&(board.bitboards, depth)) {
            let position = self.transpostions.get(&(board.bitboards, depth)).unwrap();
            match position.flag {
                ScoreType::Exact => return position.value,
                ScoreType::LowerBound => alpha = max(alpha, position.value),
                ScoreType::UpperBound => beta = min(beta, position.value),
            }
        }

        // let moves: Vec<Move> = order_moves(board.clone(), generate_moves(&board), player);
        let moves: Vec<Move> = self.generate_moves(&board);
        // moves.sort_by_cached_key(|a| score_move(&mut board, a));
        // moves.reverse();
        // moves.sort_by(|a, b| score_move(&mut board, b).cmp(&score_move(&mut board, a)));
        assert!(moves.len() > 0);

        let mut value = NEGINFINITY;
        for mov in moves {
            let mut board2 = board.clone();
            board2.place(mov);
            let ngresult = -self.negamax(board2, depth - 1, -beta, -alpha, -player);
            // value = max(value, -negamax(board2, depth - 1, -beta, -alpha, -player)); // Beta and Alpha are swapped here
            value = max(value, ngresult);
            alpha = max(value, alpha);
            if alpha >= beta {
                // print!("snip!");
                break;
            }
        }

        self.transpostions.insert((board.bitboards, depth), Score {
            value,
            flag: match value {
                x if x <= orig_alpha => ScoreType::UpperBound,
                x if x >= beta => ScoreType::LowerBound,
                _ => ScoreType::Exact,
            }
        });

        // alpha
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heuristics() {
        let mut game = Game::new(3, 3, 3);

        game.place((0, 0));
        game.place((1, 0));
        game.place((0, 1));
        game.place((2, 0));

        assert_eq!(game.best_move(), (0, 2));
    }

    #[test]
    fn test_algo8() {
        let mut game = Game::new(4, 4, 3);

        game.best_move(); // Create transposition table
        game.place((1, 1));
        game.place((1, 0));

        game.board.print(true);

        assert_eq!(game.best_move(), (2, 1))
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

        assert_eq!(game.best_move(), (2, 2));
    }

    // #[test]
    // fn test_placement() {
    //     let mut board = Board::new(5, 5, 3);

    //     board.place((0, 0));
    //     board.place((0, 1));
    //     board.place((4, 0));

    //     let bitboards = board.bitboards();

    //     assert_eq!(bitboards[Player::X as usize], 0b1_0001);
    //     assert_eq!(bitboards[Player::O as usize], 0b100_0000_0000);
    // }

    // #[test]
    // fn test_occupied1() {
    //     let mut board = Board::new(3, 3, 3);

    //     board.place((2, 0));
    //     board.place((0, 0));
    //     board.place((1, 1));

    //     assert!(board.occupied(board.get_index(1, 1)));
    //     assert!(board.occupied(board.get_index(0, 0)));
    //     assert!(board.occupied(board.get_index(2, 0)));
    //     assert!(!board.occupied(board.get_index(2, 2)));
    // }

    // #[test]
    // fn test_occupied2() {
    //     let mut board = Board::new(4, 4, 3);

    //     board.place((1, 2));
    //     board.place((0, 0));
    //     board.place((3, 2));

    //     assert!(board.occupied(board.get_index(1, 2)));
    //     assert!(board.occupied(board.get_index(0, 0)));
    //     assert!(board.occupied(board.get_index(3, 2)));
    //     assert!(!board.occupied(board.get_index(2, 2)));
    // }

    // fn vertical_win4() -> bool {
    //     let mut board = Board::new(4, 4, 4);

    //     board.place((0, 0));
    //     board.place((3, 3));
    //     board.place((1, 0));
    //     board.place((2, 1));
    //     board.place((2, 0));
    //     board.place((0, 3));
    //     board.place((3, 0));

    //     board.has_won(Player::X)
    // }

    // fn diagonal_win3() -> bool {
    //     let mut board = Board::new(3, 3, 3);

    //     board.place((1, 1));
    //     board.place((0, 1));
    //     board.place((0, 0));
    //     board.place((1, 0));
    //     board.place((2, 2));

    //     board.has_won(Player::X)
    // }

    // fn vertical_win3() -> bool {
    //     let mut board = Board::new(3, 3, 3);

    //     board.place((1, 1));
    //     board.place((1, 0));
    //     board.place((0, 1));
    //     board.place((2, 0));
    //     board.place((2, 1));

    //     board.has_won(Player::X)
    // }

    // fn not_vertical_win() -> bool {
    //     let mut board = Board::new(4, 4, 4);

    //     board.place((0, 0));
    //     board.place((3, 3));
    //     board.place((1, 0));
    //     board.place((2, 1));
    //     board.place((2, 0));
    //     board.place((0, 3));
    //     board.place((3, 1));

    //     board.has_won(Player::X)
    // }

    // #[test]
    // fn test_vertical_win() {
    //     assert!(vertical_win4());
    //     assert!(vertical_win3());
    //     assert!(!not_vertical_win());
    // }

    // #[test]
    // fn horizontal_win4() {
    //     let mut board = Board::new(7, 7, 4);

    //     board.place((0, 2));
    //     board.place((5, 5));
    //     board.place((0, 3));
    //     board.place((2, 1));
    //     board.place((0, 4));
    //     board.place((0, 0));
    //     board.place((0, 5));

    //     assert!(board.has_won(Player::X))
    // }

    // #[test]
    // fn not_horizontal_win() {
    //     let mut board = Board::new(5, 5, 5);

    //     // board.place(3, 3);
    //     // board.place(2, 1);
    //     // board.place(3, 1);
    //     // board.place(2, 3);
    //     // board.place(1, 2);
    //     // board.place(0, 0);

    //     board.place((2, 2));
    //     board.place((0, 0));
    //     board.place((1, 2));
    //     board.place((0, 1));
    //     board.place((0, 2));
    //     board.place((0, 3));
    //     board.place((4, 2));
    //     board.place((0, 4));

    //     board.print(true);

    //     assert!(!board.has_won(Player::O));
    // }

    // #[test]
    // fn not_horizontal_win2() {
    //     let mut board = Board::new(5, 5, 5);

    //     board.place((4, 0));
    //     board.place((0, 0));
    //     board.place((4, 1));
    //     board.place((2, 2));
    //     board.place((3, 2));
    //     board.place((2, 0));
    //     board.place((4, 3));
    //     board.place((3, 1));
    //     board.place((4, 4));

    //     board.print(true);

    //     assert!(!board.has_won(Player::X));
    // }

    // #[test]
    // fn test_diagonal_win() {
    //     assert!(diagonal_win3());
    // }

    #[test]
    fn test_algo2() {
        let mut game = Game::new(3, 3, 3);

        // assert_eq!(board.internal_best_move(), (0, 0));
        game.place((0, 0)); // X
        game.place((1, 1)); // O
        game.place((0, 1)); // X
        assert_eq!(game.best_move(), (0, 2));
    }

    #[test]
    fn test_algo1() {
        let mut game = Game::new(3, 3, 3);

        game.place((0, 0));
        assert_eq!(game.best_move(), (1, 1));
    }

    // #[test]
    // fn test_board_over() {
    //     let mut board = Board::new(3, 3, 3);
    //     board.place((0, 0));
    //     board.place((0, 1));
    //     board.place((1, 1));
    //     board.place((2, 2));
    //     board.place((1, 0));
    //     board.place((2, 0));
    //     board.place((1, 2));

    //     board.print(true);
    //     assert!(board.over());
    // }

    // #[test]
    // fn test_draw() {
    //     let mut board = Board::new(3, 3, 3);

    //     board.place((0, 0)); // X
    //     board.place((0, 1)); // O
    //     board.place((1, 0)); // X
    //     board.place((0, 2)); // O
    //     board.place((1, 1)); // X
    //     board.place((2, 0)); // O
    //     board.place((2, 1)); // X
    //     board.place((2, 2)); // O
    //     board.place((1, 2)); // x

    //     board.print(true);
    //     assert!(!board.draw());
    // }

    #[test]
    fn test_algo3() {
        let mut game = Game::new(3, 3, 3);

        // assert_eq!(board.internal_best_move(), (0, 0));
        game.place((0, 0)); // X
        game.board.print(true);
        game.place((0, 1)); // O
        assert_ne!(game.best_move(), (0, 2));
    }


    #[test]
    fn test_algo4() {
        let mut game = Game::new(3, 3, 3);

        game.place((0, 0)); // X
        game.place((0, 1)); // O
        game.place((1, 1)); // X
        game.place((2, 2)); // O
        // assert_eq!(board.internal_best_move(), (1, 0)); // X's turn
        let best_move = game.best_move();
        assert!(best_move == (1, 0) || best_move == (2, 0));
        // board.place(1, 0); // X
        // board.place(2, 1); // O
        // println!("{:?}", board.internal_best_move());
        // panic!();
    }

    #[test]
    fn test_algo5() {
        let mut game = Game::new(3, 3, 3);

        game.place((0, 0)); // X
        game.place((1, 1)); // O
        game.place((0, 1)); // X
        game.place((0, 2)); // O
        game.board.print(true);

        assert_eq!(game.best_move(), (2, 0));
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

        assert_eq!(game.best_move(), (2, 1));
    }

    #[test]
    fn test_algo7() {
        let mut game = Game::new(3, 3, 3);

        game.place((1, 1));
        game.place((0, 0));
        game.place((0, 1));

        game.board.print(true);

        assert_eq!(game.best_move(), (2, 1));
    }

    // #[test]
    // fn test_clear() {
    //     let mut board = Board::new(3, 3, 3);

    //     board.place((0, 0));

    //     board.print(true);

    //     board.remove(0, 0);

    //     board.print(true);

    //     assert_eq!(board.bitboards()[0], 0)
    // }

    // #[test]
    // #[should_panic]
    // fn test_clear2() {
    //     let mut board = Board::new(3, 3, 3);

    //     board.place((0, 0));

    //     board.print(true);

    //     board.remove(0, 1);

    //     board.print(true);
    // }


    // #[test]
    // fn test_heuristics() {
    //     let mut board = Board::new(3, 3, 3);
    //     board.place((0, 0));
    //     board.place((1, 0));
    //     board.place((0, 1));
    //     board.place((2, 0));
    //     board.place((1, 1));
    //     board.place((1, 2));

    //     board.print(true);

    //     assert_eq!(board.asdf(Player::X), 3);
    // }
}
