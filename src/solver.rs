use std::{cmp::{max, min}, ops::Not};
use ahash::AHashMap;

type Bitboard = u128; // Maximum board size is 10x10
pub type Move = u128;

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

fn generate_top_mask(width: u32, height: u32) -> Bitboard {
    let mut top_mask = 0;
    for i in 1..width + 2 {
        top_mask |= 1 << ((height + 1) * i)
    }
    top_mask >> 1
}

pub struct Game {
    pub board: Board,
    pub solver: Solver,
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
                top_mask: generate_top_mask(width, height),
            },
            solver: Solver {
                transpositions: AHashMap::new(),
            }
        }
    }

    pub fn place(&mut self, pos: (u32, u32)) {
        let mov = 1 << self.board.get_index(pos.0, pos.1);
        self.board.placebit(mov);
    }

    pub fn best_move(&mut self) -> Move {
        self.solver.best_move(&self.board)
    }

    pub fn can_play(&self, pos: (u32, u32)) -> bool {
        let mov = 1 << self.board.get_index(pos.0, pos.1);
        return !self.board.over() && !self.board.occupied(mov)
    }
}

#[derive(Clone, Copy)]
pub struct Board {
    width: u32,
    height: u32,
    row: u32,
    bitboards: [Bitboard; 2],
    player: Player,
    top_mask: Bitboard,
}

impl Board {
    fn get_index(&self, row: u32, column: u32) -> u32 {
        let height = self.height + 1;
        let row = row + 1;
        height - 1 - row + (column * height)
    }

    fn render(&self, debug: bool) -> String {
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

    fn occupied(&self, mov: Move) -> bool {
        (self.bitboards[0] | self.bitboards[1]) & mov != 0
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
            player: Player::X,
            top_mask: generate_top_mask(width, height)
        }
    }

    // fn place(&mut self, mov: Move) -> bool {
    //     let idx = self.get_index(mov.0, mov.1);
    //     self.placeidx(idx)
    // }

    fn draw(&self) -> bool {
        (self.bitboards[Player::X as usize] | self.bitboards[Player::O as usize])
            .count_ones() == self.height * self.width && !self.has_won(Player::X) && !self.has_won(Player::O)
    }

    pub fn placebit(&mut self, mov: Move) {
        self.bitboards[self.player as usize] |= mov;
        self.player = !self.player;
    }

    fn placeidx(&mut self, idx: u32) -> bool {
        let mask = 1 << idx;
        let mut won = self.has_won(Player::X) || self.has_won(Player::O);
        if !self.occupied(mask) && !won {
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

pub struct Solver {
    transpositions: AHashMap<([Bitboard; 2], i32), Score>
}

impl Solver {
    fn generate_moves(&self, board: &Board) -> Move {
        (!(board.bitboards[0] | board.bitboards[1])) & !((!0 << (board.width * (board.height + 1))) | board.top_mask)
    }

    fn best_move(&mut self, board: &Board) -> Move {
        let player: i8 = match board.player {
            Player::O => -1,
            Player::X => 1,
        };
        let mut best_score = NEGINFINITY;
        let mut best_move: Option<Move> = None;
        let moves = self.generate_moves(board);
        // moves.sort_by(|a, b| score_move(self, b).cmp(&score_move(self, a)));
        // moves.sort_by_cached_key(|a| score_move(self, a));
        // moves.reverse();
        for i in 0..128 {
            // log!("{}-{}", row, col);
            let mov = ((moves >> i) & 1) << i;
            if mov != 0 { 
                let mut board2 = board.clone();
                board2.placebit(mov);
                let score = -self.negamax(board2, INFINITY, NEGINFINITY, INFINITY, player);
                // println!("{:?} score: {}", mov, score);
                if score > best_score {
                    best_score = score;
                    best_move = Some(mov)
                }
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


        if self.transpositions.contains_key(&(board.bitboards, depth)) {
            let position = self.transpositions.get(&(board.bitboards, depth)).unwrap();
            match position.flag {
                ScoreType::Exact => return position.value,
                ScoreType::LowerBound => alpha = max(alpha, position.value),
                ScoreType::UpperBound => beta = min(beta, position.value),
            }

            if alpha >= beta {
                return position.value
            }
        }

        // let moves: Vec<Move> = order_moves(board.clone(), generate_moves(&board), player);
        let moves: Move = self.generate_moves(&board);
        // moves.sort_by_cached_key(|a| score_move(&mut board, a));
        // moves.reverse();
        // moves.sort_by(|a, b| score_move(&mut board, b).cmp(&score_move(&mut board, a)));
        assert_ne!(moves, 0);

        let mut value = NEGINFINITY;
        for i in 0..128 {
            let mov: u128 = ((moves >> i) & 1) << i;
            if mov != 0 { 
                let mut board2 = board.clone();
                board2.placebit(mov);
                let ngresult = -self.negamax(board2, depth - 1, -beta, -alpha, -player);
                // value = max(value, -negamax(board2, depth - 1, -beta, -alpha, -player)); // Beta and Alpha are swapped here
                value = max(value, ngresult);
                alpha = max(value, alpha);
                if alpha >= beta {
                    // print!("snip!");
                    break;
                }
            }
        }

        self.transpositions.insert((board.bitboards, depth), Score {
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

    pub fn clear_transpositions(&mut self) {
        self.transpositions.clear();
    }
}
