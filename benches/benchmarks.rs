use criterion::{*};
use tin::interpreter::*;
use rand::Rng;

pub fn naive_primality_benchmark(c: &mut Criterion) {
    let mut intrp = TinInterpreter::new();
    let program = intrp.parse("→n(.nι``.n%∀1.n>)∀←n");

    let mut group = c.benchmark_group("Naive primality");

    for i in (0..10001).step_by(1000) {
        group.bench_with_input(BenchmarkId::from_parameter(i), &i, |b, &i| {
            b.iter(|| {
                let mut stack = vec!(TinValue::INT(black_box(i)));
                intrp.execute(&program, Option::None, &mut stack);
            });
        });
    }
    group.finish();
}

pub fn iterative_fibonacci_benchmark(c: &mut Criterion) {
    let mut intrp = TinInterpreter::new();
    let program = intrp.parse("!!→n1<?⟨2ι→r ⊲ι{(.r1↓ .r∑)→.r}.r1↓→.n⟩.n←r←n");

    let mut group = c.benchmark_group("Iterative fibonacci");

    for i in (0..51).step_by(2) {
        group.bench_with_input(BenchmarkId::from_parameter(i), &i, |b, &i| {
            b.iter(|| {
                let mut stack = vec!(TinValue::INT(black_box(i)));
                intrp.execute(&program, Option::None, &mut stack);
            });
        });
    }
    group.finish();
}

pub fn recursive_fibonacci_benchmark(c: &mut Criterion) {
    let mut intrp = TinInterpreter::new();
    let program = intrp.parse("!1<?⟨⊲!⊲∇↶∇+⟩");

    let mut group = c.benchmark_group("Recursive fibonacci");

    for i in (0..26).step_by(2) {
        group.bench_with_input(BenchmarkId::from_parameter(i), &i, |b, &i| {
            b.iter(|| {
                let mut stack = vec!(TinValue::INT(black_box(i)));
                intrp.execute(&program, Option::None, &mut stack);
            });
        });
    }
    group.finish();
}

pub fn mode_benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();

    let mut intrp = TinInterpreter::new();
    let program = intrp.parse("→n(.n{.n↶#})!⌈º0↓.n↶↓←n");

    let mut group = c.benchmark_group("Mode");

    for i in (50..1001).step_by(50) {
        group.bench_with_input(BenchmarkId::from_parameter(i), &i, |b, &i| {
            b.iter(|| {
                let mut v = vec!();

                for _ in 0..i{
                    v.push(TinValue::INT(rng.gen_range(0..10)));
                }

                let mut stack = vec!(black_box(TinValue::VECTOR(v)));
                intrp.execute(&program, Option::None, &mut stack);
            });
        });
    }
    group.finish();
}

criterion_group!(benches, naive_primality_benchmark,
                          iterative_fibonacci_benchmark,
                          recursive_fibonacci_benchmark,
                          mode_benchmark);
criterion_main!(benches);