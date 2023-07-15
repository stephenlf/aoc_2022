/*run the following command in a new terminal to see reports
cd target\criterion
python -m http.server

[You might have to change back slashes to forward slashes in target\criterion\report\index.html]
*/

use criterion::{criterion_group, criterion_main, Criterion};
use advent_of_code::day_8_algorithms::{new_forest, single, threaded};
use std::sync::Arc;

fn criterion_benchmark(c: &mut Criterion) {
    let mut forest = new_forest();
    c.bench_function("calc-size-single", move |b| b.iter(|| single::calc_visibility(&mut forest)));

    let forest = Arc::new(new_forest());
    c.bench_function("calc-size-multi", |b| b.iter(|| threaded::calc_visibility(&forest)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);