use criterion::{black_box, criterion_group, criterion_main, Criterion};
use solver::Game;

pub fn bench_3_3_3(c: &mut Criterion) {
    let mut game = Game::new_xo(black_box(3), black_box(3), black_box(3));
    c.bench_function("3 3 3 benchmark", |b| b.iter(|| {
        game.solver.clear_transpositions();
        game.best_move()
    }));
}

pub fn bench_3_3_3_c4(c: &mut Criterion) {
    let mut game = Game::new_connect_four(black_box(3), black_box(3), black_box(3));
    c.bench_function("3 3 3 Connect Four benchmark", |b| b.iter(|| {
        game.solver.clear_transpositions();
        game.best_move()
    }));
}

pub fn bench_4_4_3(c: &mut Criterion) {
    let mut game = Game::new_xo(black_box(4), black_box(4), black_box(3));
    c.bench_function("4 4 3 benchmark", |b| b.iter(|| {
        game.solver.clear_transpositions();
        game.best_move()
    }));
}

pub fn bench_4_4_4(c: &mut Criterion) {
    let mut game = Game::new_xo(black_box(4), black_box(4), black_box(4));
    c.bench_function("4 4 4 benchmark", |b| b.iter(|| {
        game.solver.clear_transpositions();
        game.best_move()
    }));
}

pub fn bench_4_4_4_c4(c: &mut Criterion) {
    let mut game = Game::new_connect_four(black_box(4), black_box(4), black_box(4));
    c.bench_function("4 4 4 Connect Four benchmark", |b| b.iter(|| {
        game.solver.clear_transpositions();
        game.best_move()
    }));
}

pub fn bench_5_5_3_o(c: &mut Criterion) {
    let mut game = Game::new_xo(black_box(5), black_box(5), black_box(3));
    game.place(black_box((2, 2)));
    c.bench_function("5 5 3 benchmark as O", |b| b.iter(|| {
        game.solver.clear_transpositions();
        game.best_move()
    }));
}

pub fn bench_7_6_4_c4(c: &mut Criterion) {
    let mut game = Game::new_connect_four(7, 6, 4);
    c.bench_function("7 6 4 Connect Four benchmark", |b| b.iter(|| {
        game.solver.clear_transpositions();
        game.best_move()
    }));
}

criterion_group!(benches, bench_3_3_3, bench_3_3_3_c4, bench_4_4_3, bench_4_4_4, bench_4_4_4_c4, bench_5_5_3_o, bench_7_6_4_c4);
criterion_main!(benches);
