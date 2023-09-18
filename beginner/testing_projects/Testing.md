# Benchmarking Rust
rust has benchmarking crate built in but is unstable (only available in nightly rust)


## Steps for testing w/external crate
for now, can use `criterion`

1. within `cargo.toml`
```toml
[dev-dependencies]
criterion = "0.3"

[[bench]]
name = "sorting_benchmark"
harness = false
```

2. `mkdir benches && cd benches`
3. `touch sorting_benchmark.rs`
4. setup file like below (sub benchmarking functions as needed)
```rust
use sorting::sort_arr;
use criterion::{
    black_box, criterion_group, criterion_main, Criterion
};

fn sort_arr_benchmark(c: &mut Criterion) {
    // black box prevents rust from optimizing array
    let mut arr: [i32] = black_box([43,5,-4,3,0, -5]);

    c.bench_function(
        id: "sorting_algo",
        f: | b: &mut Bencher | b.iter(routine: || sort_arr(&mut arr))
    )
}


criterion_group!(benches, sort_arr_benchmark);
criterion_main!(benches);
```
5. Execute using `cargo bench`