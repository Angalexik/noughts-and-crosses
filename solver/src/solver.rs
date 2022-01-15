use std::{cmp::{max, min}, ops::Not};
use arrayvec::ArrayVec;
use fxhash::FxHashMap;

type Bitboard = u64; // Maximum board size is 7x8
pub type Move = u64;

const INFINITY: i32 = i32::MAX;
const NEGINFINITY: i32 = i32::MIN + 1;
const WIN_SCORE: i32 = INFINITY;

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

#[derive(PartialEq, Eq, Debug)]
pub enum Moves {
    XOMoves(Move, u64, u8),
    C4Moves(ArrayVec<Move, 10>, usize),
}

impl Iterator for Moves {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Moves::XOMoves(ref moves, ref mut pos, ref used_bits) => {
                for i in *pos..*used_bits as u64 { // probably wrong, but it works
                    let mov: Move = ((moves >> i) & 1) << i;
                    if mov != 0 {
                        *pos = i + 1;
                        return Some(mov);
                    }
                }
                return None
            },
            Moves::C4Moves(ref moves, ref mut pos) => {
                if *pos < moves.len() {
                    let mov = moves[*pos];
                    *pos += 1;
                    return Some(mov);
                }
                return None
            },
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
    pub fn new_xo(width: u32, height: u32, row: u32) -> Game {
        Game {
            board: Board {
                width,
                height,
                row,
                bitboards: [0, 0],
                player: Player::X,
                top_mask: generate_top_mask(width, height),
                used_bits: (width * (height + 1)) as u8,
                kind: BoardKind::XOBoard,
                col_tops: vec![0; width as usize]
            },
            solver: Solver::new(),
        }
    }

    pub fn new_connect_four(width: u32, height: u32, row: u32) -> Game {
        Game {
            board: Board {
                width,
                height,
                row,
                bitboards: [0, 0],
                player: Player::X,
                top_mask: generate_top_mask(width, height),
                used_bits: (width * (height + 1)) as u8,
                kind: BoardKind::C4Board,
                col_tops: vec![0; width as usize]
            },
            solver: Solver::new(),
        }
    }

    pub fn place(&mut self, pos: (u32, u32)) {
        self.board.placebit(self.pos_to_move(pos));
    }

    pub fn placebit(&mut self, mov: Move) {
        self.board.placebit(mov);
    }

    pub fn pos_to_move(&self, pos: (u32, u32)) -> Move {
        1 << self.board.get_index(pos.0, pos.1)
    }

    pub fn best_move(&mut self) -> Move {
        self.solver.best_move(&mut self.board)
    }

    pub fn can_play(&self, mov: Move) -> bool {
        self.board.can_play(mov)
    }

    pub fn render(&self) -> String {
        self.board.render(false)
    }

    // Doesn't return the raw score, but number of plies to win
    // 0 means a draw, not a won position
    pub fn evaluation(&mut self) -> i32 {
        let player = match self.board.player {
            Player::X => -1,
            Player::O => 1,
        };

        let score = self.solver.negamax(&mut self.board, INFINITY, NEGINFINITY, INFINITY, player) * -player as i32;
        if score == 0 { return 0; }
        match score < 0 {
            true => -WIN_SCORE - score,
            false => WIN_SCORE - score,
        }
    }
}

#[derive(Clone)]
pub enum BoardKind {
    XOBoard,
    C4Board,
}

#[derive(Clone)]
pub struct Board {
    width: u32,
    height: u32,
    row: u32,
    pub bitboards: [Bitboard; 2],
    player: Player,
    top_mask: Bitboard,
    used_bits: u8,
    col_tops: Vec<u64>,
    kind: BoardKind,
}

impl Board {
    fn can_play(&self, mov: Move) -> bool {
        match self.kind {
            BoardKind::XOBoard => !self.over() && !self.occupied(mov),
            BoardKind::C4Board => self.col_tops[mov as usize] < self.height as u64,
        }
    }

    fn get_index(&self, row: u32, column: u32) -> u32 {
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

    fn occupied(&self, mov: Move) -> bool {
        (self.bitboards[0] | self.bitboards[1]) & mov != 0
        // (self.bitboards[Player::X as usize] & mask) >> idx != 0 || (self.bitboards[Player::O as usize] & mask) >> idx != 0
    }

    pub fn has_won(&self, player: Player) -> bool {
        // Modified version of https://github.com/qu1j0t3/fhourstones/blob/bf0e70ed9fe8128eeea8539f17dd41826f2cc6b6/Game.c#L108
        let bitboard = self.bitboards[player as usize];
        let delta = self.row - 2;
        let vert = bitboard & (bitboard >> 1);
        if vert & (vert >> delta) != 0 { return true; }
        let hori = bitboard & (bitboard >> (self.height + 1));
        if hori & (hori >> delta * (self.height + 1)) != 0 { return true; }
        let diag1 = bitboard & (bitboard >> self.height);
        if diag1 & (diag1 >> delta * self.height) != 0 { return true; }
        let diag2 = bitboard & (bitboard >> (self.height + 2));
        if diag2 & (diag2 >> delta * (self.height + 2)) != 0 { return true; }
        return false;
    }

    pub fn over(&self) -> bool {
        self.has_won(Player::X) ||
            self.has_won(Player::O) ||
            self.draw()
    }

    fn draw(&self) -> bool {
        (self.bitboards[Player::X as usize] | self.bitboards[Player::O as usize])
            .count_ones() == self.height * self.width && !self.has_won(Player::X) && !self.has_won(Player::O)
    }

    pub fn placebit(&mut self, mov: Move) {
        match self.kind {
            BoardKind::XOBoard => {
                self.bitboards[self.player as usize] |= mov;
                self.player = !self.player;
            }
            BoardKind::C4Board => {
                self.bitboards[self.player as usize] |= 1 << (self.col_tops[mov as usize] + mov * (self.height + 1) as u64);
                self.col_tops[mov as usize] += 1;
                self.player = !self.player;
            }
        }
    }

    pub fn undo_move(&mut self, mov: Move) {
        match self.kind {
            BoardKind::XOBoard => {
                self.bitboards[(!self.player) as usize] ^= mov;
                self.player = !self.player;
            }
            BoardKind::C4Board => {
                self.col_tops[mov as usize] -= 1;
                self.bitboards[!self.player as usize] ^= 1 << (self.col_tops[mov as usize] + mov * (self.height + 1) as u64);
                self.player = !self.player;
            }
        }
    }

    pub fn generate_moves(&self) -> Moves {
        match self.kind { // Probably not the best way of doing things
            BoardKind::XOBoard => Moves::XOMoves((!(self.bitboards[0] | self.bitboards[1])) & !((!0 << (self.width * (self.height + 1))) | self.top_mask), 0, self.used_bits),
            BoardKind::C4Board => {
                Moves::C4Moves(self.col_tops.iter().enumerate().filter_map(|x| {
                    if *x.1 < self.height as u64 {
                        return Some(x.0 as Move);
                    }
                    None
                }).collect(), 0)
            }
        }
    }
}

enum ScoreKind {
    Exact,
    LowerBound,
    UpperBound,
}

struct Score {
    value: i32,
    kind: ScoreKind,
}

pub struct Solver {
    transpositions: FxHashMap<([Bitboard; 2], i32), Score>
}

impl Solver {
    fn new() -> Solver {
        Solver { transpositions: FxHashMap::default() }
    }

    fn best_move(&mut self, board: &mut Board) -> Move {
        let player: i8 = match board.player {
            Player::O => -1,
            Player::X => 1,
        };
        let mut best_score = NEGINFINITY;
        let mut best_move: Option<Move> = None;
        let moves = board.generate_moves();
        // moves.sort_by(|a, b| score_move(self, b).cmp(&score_move(self, a)));
        // moves.sort_by_cached_key(|a| score_move(self, a));
        // moves.reverse();
        for mov in moves {
            // log!("{}-{}", row, col);
            board.placebit(mov);
            let score = -self.negamax(board, WIN_SCORE, NEGINFINITY, INFINITY, player);
            board.undo_move(mov);
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

    fn negamax(&mut self, board: &mut Board, depth: i32, mut alpha: i32, mut beta: i32, player: i8) -> i32 {
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
            match position.kind {
                ScoreKind::Exact => return position.value,
                ScoreKind::LowerBound => alpha = max(alpha, position.value),
                ScoreKind::UpperBound => beta = min(beta, position.value),
            }

            if alpha >= beta {
                return position.value
            }
        }

        // let moves: Vec<Move> = order_moves(board.clone(), generate_moves(&board), player);
        let moves = board.generate_moves();
        // moves.sort_by_cached_key(|a| score_move(&mut board, a));
        // moves.reverse();
        // moves.sort_by(|a, b| score_move(&mut board, b).cmp(&score_move(&mut board, a)));

        let mut value = NEGINFINITY;
        for mov in moves {
            board.placebit(mov);
            let ngresult = -self.negamax(board, depth - 1, -beta, -alpha, -player);
            board.undo_move(mov);
            // value = max(value, -negamax(board2, depth - 1, -beta, -alpha, -player)); // Beta and Alpha are swapped here
            value = max(value, ngresult);
            alpha = max(value, alpha);
            if alpha >= beta {
                // print!("snip!");
                break;
            }
        }

        self.transpositions.insert((board.bitboards, depth), Score {
            value,
            kind: match value {
                x if x <= orig_alpha => ScoreKind::UpperBound,
                x if x >= beta => ScoreKind::LowerBound,
                _ => ScoreKind::Exact,
            }
        });

        // alpha
        value
    }

    pub fn clear_transpositions(&mut self) {
        self.transpositions.clear();
    }
}
