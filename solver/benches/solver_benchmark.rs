use criterion::{black_box, criterion_group, criterion_main, Criterion};
use solver::{FunnyTable, Game};

const TT_SIZE: usize = 2_000_000_000 / 8; // 2 GB

pub fn bench_funnytable(c: &mut Criterion) {
    c.bench_function("FunnyTable benchmark", |b| b.iter(|| FunnyTable::default()));
}

pub fn bench_3_3_3(c: &mut Criterion) {
    let mut game = Game::xo_with_size(black_box(3), black_box(3), black_box(3), black_box(TT_SIZE));
    c.bench_function("3 3 3 benchmark", |b| {
        b.iter(|| {
            game.solver.clear_transpositions();
            game.best_move()
        })
    });
}

pub fn bench_3_3_3_c4(c: &mut Criterion) {
    let mut game =
        Game::connect_four_with_size(black_box(3), black_box(3), black_box(3), black_box(TT_SIZE));
    c.bench_function("3 3 3 Connect Four benchmark", |b| {
        game.solver.clear_transpositions();
        b.iter(|| game.best_move())
    });
}

pub fn bench_4_4_3(c: &mut Criterion) {
    let mut game = Game::xo_with_size(black_box(4), black_box(4), black_box(3), black_box(TT_SIZE));
    c.bench_function("4 4 3 benchmark", |b| {
        b.iter(|| {
            game.solver.clear_transpositions();
            game.best_move()
        })
    });
}

pub fn bench_4_4_4(c: &mut Criterion) {
    let mut game = Game::xo_with_size(black_box(4), black_box(4), black_box(4), black_box(TT_SIZE));
    c.bench_function("4 4 4 benchmark", |b| {
        b.iter(|| {
            game.solver.clear_transpositions();
            game.best_move()
        })
    });
}

pub fn bench_4_4_4_c4(c: &mut Criterion) {
    let mut game =
        Game::connect_four_with_size(black_box(4), black_box(4), black_box(4), black_box(TT_SIZE));
    c.bench_function("4 4 4 Connect Four benchmark", |b| {
        b.iter(|| {
            game.solver.clear_transpositions();
            game.best_move()
        })
    });
}

pub fn bench_5_5_3_o(c: &mut Criterion) {
    let mut game = Game::xo_with_size(black_box(5), black_box(5), black_box(3), black_box(TT_SIZE));
    game.place(black_box((2, 2)));
    c.bench_function("5 5 3 benchmark as O", |b| {
        b.iter(|| {
            game.solver.clear_transpositions();
            game.best_move()
        })
    });
}

pub fn bench_7_6_4_c4(c: &mut Criterion) {
    let mut game =
        Game::connect_four_with_size(black_box(7), black_box(6), black_box(4), black_box(TT_SIZE));
    c.bench_function("7 6 4 Connect Four benchmark", |b| {
        b.iter(|| {
            game.solver.clear_transpositions();
            game.best_move()
        })
    });
}

criterion_group!(
    benches,
    bench_3_3_3,
    bench_3_3_3_c4,
    bench_4_4_3,
    bench_4_4_4,
    bench_4_4_4_c4,
    bench_5_5_3_o,
    bench_funnytable,
    // bench_7_6_4_c4
);
criterion_main!(benches);
