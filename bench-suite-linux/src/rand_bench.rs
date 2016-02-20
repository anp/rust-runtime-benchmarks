const RAND_BENCH_N: u64 = 1000;

use std::collections::BTreeMap;
use test::black_box;
use rand::{XorShiftRng, StdRng, IsaacRng, Isaac64Rng, Rng, OsRng, weak_rng};
use rand;
use rand::distributions::IndependentSample;
use rand::distributions::gamma::Gamma;
use rand::distributions::exponential::Exp;
use rand::distributions::Sample;
use rand::distributions::normal::Normal;

pub fn run_all() -> BTreeMap<String, Vec<u64>> {
    let mut results = BTreeMap::new();

    for _ in 0..3 {
        results.entry("rand_xorshift".to_string()).or_insert(vec![]).push(rand_xorshift());
        results.entry("rand_isaac".to_string()).or_insert(vec![]).push(rand_isaac());
        results.entry("rand_isaac64".to_string()).or_insert(vec![]).push(rand_isaac64());
        results.entry("rand_std".to_string()).or_insert(vec![]).push(rand_std());
        results.entry("rand_f32".to_string()).or_insert(vec![]).push(rand_f32());
        results.entry("rand_f64".to_string()).or_insert(vec![]).push(rand_f64());
        results.entry("rand_shuffle_100".to_string()).or_insert(vec![]).push(rand_shuffle_100());
        results.entry("bench_gamma_large_shape".to_string())
               .or_insert(vec![])
               .push(bench_gamma_large_shape());
        results.entry("bench_gamma_small_shape".to_string())
               .or_insert(vec![])
               .push(bench_gamma_small_shape());
        results.entry("rand_exp".to_string()).or_insert(vec![]).push(rand_exp());
        results.entry("rand_normal".to_string()).or_insert(vec![]).push(rand_normal());
    }

    results
}

fn rand_xorshift() -> u64 {
    let mut rng: XorShiftRng = OsRng::new().unwrap().gen();
    bench!(1_000_000, {
        for _ in 0..RAND_BENCH_N {
            black_box(rng.gen::<usize>());
        }
    })
}

fn rand_isaac() -> u64 {
    let mut rng: IsaacRng = OsRng::new().unwrap().gen();
    bench!(100_000, {
        for _ in 0..RAND_BENCH_N {
            black_box(rng.gen::<usize>());
        }
    })
}

fn rand_isaac64() -> u64 {
    let mut rng: Isaac64Rng = OsRng::new().unwrap().gen();
    bench!(1_000_000, {
        for _ in 0..RAND_BENCH_N {
            black_box(rng.gen::<usize>());
        }
    })
}

fn rand_std() -> u64 {
    let mut rng = StdRng::new().unwrap();
    bench!(1_000_000, {
        for _ in 0..RAND_BENCH_N {
            black_box(rng.gen::<usize>());
        }
    })
}

fn rand_f32() -> u64 {
    let mut rng = StdRng::new().unwrap();
    bench!(1_000_000, {
        for _ in 0..RAND_BENCH_N {
            black_box(rng.next_f32());
        }
    })
}

fn rand_f64() -> u64 {
    let mut rng = StdRng::new().unwrap();
    bench!(1_000_000, {
        for _ in 0..RAND_BENCH_N {
            black_box(rng.next_f64());
        }
    })
}

fn rand_shuffle_100() -> u64 {
    let mut rng = weak_rng();
    let x: &mut [usize] = &mut [1; 100];
    bench!(1_000_000, {
        rng.shuffle(x);
    })
}

fn bench_gamma_large_shape() -> u64 {
    let gamma = Gamma::new(10., 1.0);
    let mut rng = rand::weak_rng();

    bench!(100_000, {
        for _ in 0..RAND_BENCH_N {
            gamma.ind_sample(&mut rng);
        }
    })
}

fn bench_gamma_small_shape() -> u64 {
    let gamma = Gamma::new(0.1, 1.0);
    let mut rng = rand::weak_rng();

    bench!(100_000, {
        for _ in 0..RAND_BENCH_N {
            gamma.ind_sample(&mut rng);
        }
    })
}

fn rand_exp() -> u64 {
    let mut rng = rand::weak_rng();
    let mut exp = Exp::new(2.71828 * 3.14159);

    bench!(1_000_000, {
        for _ in 0..RAND_BENCH_N {
            exp.sample(&mut rng);
        }
    })
}

fn rand_normal() -> u64 {
    let mut rng = rand::weak_rng();
    let mut normal = Normal::new(-2.71828, 3.14159);

    bench!(1_000_000, {
        for _ in 0..RAND_BENCH_N {
            normal.sample(&mut rng);
        }
    })
}
