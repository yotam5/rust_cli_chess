use criterion::{black_box, criterion_group, criterion_main, Criterion};
use cte::chess::board;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("board_creation", |b| b.iter(board::Board::new));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
