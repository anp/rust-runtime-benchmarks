use optional;
use std::collections::BTreeMap;
use test;

pub fn run_all() -> BTreeMap<String, Vec<u64>> {
    let mut results = BTreeMap::new();

    for _ in 0..3 {
        results.entry("bench_iter_opt_u8".to_string()).or_insert(vec![]).push(bench_iter_opt_u8());
        results.entry("bench_as_slice_iter_opt_u8".to_string())
               .or_insert(vec![])
               .push(bench_as_slice_iter_opt_u8());
        results.entry("bench_iter_std_u8".to_string()).or_insert(vec![]).push(bench_iter_std_u8());
        results.entry("bench_iter_opt_u16".to_string())
               .or_insert(vec![])
               .push(bench_iter_opt_u16());
        results.entry("bench_as_slice_iter_opt_u16".to_string())
               .or_insert(vec![])
               .push(bench_as_slice_iter_opt_u16());
        results.entry("bench_iter_std_u16".to_string())
               .or_insert(vec![])
               .push(bench_iter_std_u16());
        results.entry("bench_iter_opt_u32".to_string())
               .or_insert(vec![])
               .push(bench_iter_opt_u32());
        results.entry("bench_as_slice_iter_opt_u32".to_string())
               .or_insert(vec![])
               .push(bench_as_slice_iter_opt_u32());
        results.entry("bench_iter_std_u32".to_string())
               .or_insert(vec![])
               .push(bench_iter_std_u32());
        results.entry("bench_iter_opt_u64".to_string())
               .or_insert(vec![])
               .push(bench_iter_opt_u64());
        results.entry("bench_as_slice_iter_opt_u64".to_string())
               .or_insert(vec![])
               .push(bench_as_slice_iter_opt_u64());
        results.entry("bench_iter_std_u64".to_string())
               .or_insert(vec![])
               .push(bench_iter_std_u64());
        results.entry("bench_iter_opt_i8".to_string()).or_insert(vec![]).push(bench_iter_opt_i8());
        results.entry("bench_as_slice_iter_opt_i8".to_string())
               .or_insert(vec![])
               .push(bench_as_slice_iter_opt_i8());
        results.entry("bench_iter_std_i8".to_string()).or_insert(vec![]).push(bench_iter_std_i8());
        results.entry("bench_iter_opt_i16".to_string())
               .or_insert(vec![])
               .push(bench_iter_opt_i16());
        results.entry("bench_as_slice_iter_opt_i16".to_string())
               .or_insert(vec![])
               .push(bench_as_slice_iter_opt_i16());
        results.entry("bench_iter_std_i16".to_string())
               .or_insert(vec![])
               .push(bench_iter_std_i16());
        results.entry("bench_iter_opt_i32".to_string())
               .or_insert(vec![])
               .push(bench_iter_opt_i32());
        results.entry("bench_as_slice_iter_opt_i32".to_string())
               .or_insert(vec![])
               .push(bench_as_slice_iter_opt_i32());
        results.entry("bench_iter_std_i32".to_string())
               .or_insert(vec![])
               .push(bench_iter_std_i32());
        results.entry("bench_iter_opt_i64".to_string())
               .or_insert(vec![])
               .push(bench_iter_opt_i64());
        results.entry("bench_as_slice_iter_opt_i64".to_string())
               .or_insert(vec![])
               .push(bench_as_slice_iter_opt_i64());
        results.entry("bench_iter_std_i64".to_string())
               .or_insert(vec![])
               .push(bench_iter_std_i64());
        results.entry("bench_iter_opt_f32".to_string())
               .or_insert(vec![])
               .push(bench_iter_opt_f32());
        results.entry("bench_as_slice_iter_opt_f32".to_string())
               .or_insert(vec![])
               .push(bench_as_slice_iter_opt_f32());
        results.entry("bench_iter_std_f32".to_string())
               .or_insert(vec![])
               .push(bench_iter_std_f32());
        results.entry("bench_iter_opt_f64".to_string())
               .or_insert(vec![])
               .push(bench_iter_opt_f64());
        results.entry("bench_as_slice_iter_opt_f64".to_string())
               .or_insert(vec![])
               .push(bench_as_slice_iter_opt_f64());
        results.entry("bench_iter_std_f64".to_string())
               .or_insert(vec![])
               .push(bench_iter_std_f64());
        results.entry("bench_is_some_opt_u8".to_string())
               .or_insert(vec![])
               .push(bench_is_some_opt_u8());
        results.entry("bench_is_some_std_u8".to_string())
               .or_insert(vec![])
               .push(bench_is_some_std_u8());
        results.entry("bench_is_some_opt_u16".to_string())
               .or_insert(vec![])
               .push(bench_is_some_opt_u16());
        results.entry("bench_is_some_std_u16".to_string())
               .or_insert(vec![])
               .push(bench_is_some_std_u16());
        results.entry("bench_is_some_opt_u32".to_string())
               .or_insert(vec![])
               .push(bench_is_some_opt_u32());
        results.entry("bench_is_some_std_u32".to_string())
               .or_insert(vec![])
               .push(bench_is_some_std_u32());
        results.entry("bench_is_some_opt_u64".to_string())
               .or_insert(vec![])
               .push(bench_is_some_opt_u64());
        results.entry("bench_is_some_std_u64".to_string())
               .or_insert(vec![])
               .push(bench_is_some_std_u64());
        results.entry("bench_is_some_opt_f32".to_string())
               .or_insert(vec![])
               .push(bench_is_some_opt_f32());
        results.entry("bench_is_some_std_f32".to_string())
               .or_insert(vec![])
               .push(bench_is_some_std_f32());
        results.entry("bench_is_some_opt_f64".to_string())
               .or_insert(vec![])
               .push(bench_is_some_opt_f64());
        results.entry("bench_is_some_std_f64".to_string())
               .or_insert(vec![])
               .push(bench_is_some_std_f64());
        results.entry("bench_map_opt_u8".to_string()).or_insert(vec![]).push(bench_map_opt_u8());
        results.entry("bench_map_std_u8".to_string()).or_insert(vec![]).push(bench_map_std_u8());
        results.entry("bench_map_opt_u16".to_string()).or_insert(vec![]).push(bench_map_opt_u16());
        results.entry("bench_map_std_u16".to_string()).or_insert(vec![]).push(bench_map_std_u16());
        results.entry("bench_map_opt_u32".to_string()).or_insert(vec![]).push(bench_map_opt_u32());
        results.entry("bench_map_std_u32".to_string()).or_insert(vec![]).push(bench_map_std_u32());
        results.entry("bench_map_opt_u64".to_string()).or_insert(vec![]).push(bench_map_opt_u64());
        results.entry("bench_map_std_u64".to_string()).or_insert(vec![]).push(bench_map_std_u64());
        results.entry("bench_map_opt_f32".to_string()).or_insert(vec![]).push(bench_map_opt_f32());
        results.entry("bench_map_std_f32".to_string()).or_insert(vec![]).push(bench_map_std_f32());
        results.entry("bench_map_opt_f64".to_string()).or_insert(vec![]).push(bench_map_opt_f64());
        results.entry("bench_map_std_f64".to_string()).or_insert(vec![]).push(bench_map_std_f64());
        results.entry("bench_map_or_std_u8".to_string())
               .or_insert(vec![])
               .push(bench_map_or_std_u8());
        results.entry("bench_map_or_std_u16".to_string())
               .or_insert(vec![])
               .push(bench_map_or_std_u16());
        results.entry("bench_map_or_std_u32".to_string())
               .or_insert(vec![])
               .push(bench_map_or_std_u32());
        results.entry("bench_map_or_std_u64".to_string())
               .or_insert(vec![])
               .push(bench_map_or_std_u64());
        results.entry("bench_map_or_std_f32".to_string())
               .or_insert(vec![])
               .push(bench_map_or_std_f32());
        results.entry("bench_map_or_std_f64".to_string())
               .or_insert(vec![])
               .push(bench_map_or_std_f64());
        results.entry("bench_map_or_opt_u8".to_string())
               .or_insert(vec![])
               .push(bench_map_or_opt_u8());
        results.entry("bench_map_or_opt_u16".to_string())
               .or_insert(vec![])
               .push(bench_map_or_opt_u16());
        results.entry("bench_map_or_opt_u32".to_string())
               .or_insert(vec![])
               .push(bench_map_or_opt_u32());
        results.entry("bench_map_or_opt_u64".to_string())
               .or_insert(vec![])
               .push(bench_map_or_opt_u64());
        results.entry("bench_map_or_opt_f32".to_string())
               .or_insert(vec![])
               .push(bench_map_or_opt_f32());
        results.entry("bench_map_or_opt_f64".to_string())
               .or_insert(vec![])
               .push(bench_map_or_opt_f64());
    }

    results
}

fn bench_iter_opt_u8() -> u64 {
    fn iter_opt_u8() {
        for o in [optional::some(1u8), optional::none()]
                     .iter()
                     .cycle()
                     .take(1200) {
            for b in o.iter() {
                test::black_box(b);
            }
        }
    }

    bench!(1_000_000, {
        iter_opt_u8()
    })
}

fn bench_as_slice_iter_opt_u8() -> u64 {
    fn as_slice_iter_opt_u8() {
        for o in [optional::some(1u8), optional::none()]
                     .iter()
                     .cycle()
                     .take(1200) {
            for b in o.as_slice().iter() {
                test::black_box(b);
            }
        }
    }

    bench!(1_000_000, {
        as_slice_iter_opt_u8()
    })
}

fn bench_iter_std_u8() -> u64 {
    fn iter_std_u8() {
        for o in [Option::Some(1u8), Option::None]
                     .iter()
                     .cycle()
                     .take(1200) {
            for b in o.iter() {
                test::black_box(b);
            }
        }
    }

    bench!(1_000_000, {
        iter_std_u8()
    })
}

fn bench_iter_opt_u16() -> u64 {
    fn iter_opt_u16() {
        for o in [optional::some(1u16), optional::none()]
                     .iter()
                     .cycle()
                     .take(1200) {
            for b in o.iter() {
                test::black_box(b);
            }
        }
    }

    bench!(1_000_000, {
        iter_opt_u16()
    })
}

fn bench_as_slice_iter_opt_u16() -> u64 {
    fn as_slice_iter_opt_u16() {
        for o in [optional::some(1u16), optional::none()]
                     .iter()
                     .cycle()
                     .take(1200) {
            for b in o.as_slice().iter() {
                test::black_box(b);
            }
        }
    }

    bench!(1_000_000, {
        as_slice_iter_opt_u16()
    })
}

fn bench_iter_std_u16() -> u64 {
    fn iter_std_u16() {
        for o in [Option::Some(1u16), Option::None]
                     .iter()
                     .cycle()
                     .take(1200) {
            for b in o.iter() {
                test::black_box(b);
            }
        }
    }

    bench!(1_000_000, {
        iter_std_u16()
    })
}

fn bench_iter_opt_u32() -> u64 {
    fn iter_opt_u32() {
        for o in [optional::some(1u32), optional::none()]
                     .iter()
                     .cycle()
                     .take(1200) {
            for b in o.iter() {
                test::black_box(b);
            }
        }
    }

    bench!(1_000_000, {
        iter_opt_u32()
    })
}

fn bench_as_slice_iter_opt_u32() -> u64 {
    fn as_slice_iter_opt_u32() {
        for o in [optional::some(1u32), optional::none()]
                     .iter()
                     .cycle()
                     .take(1200) {
            for b in o.as_slice().iter() {
                test::black_box(b);
            }
        }
    }

    bench!(1_000_000, {
        as_slice_iter_opt_u32()
    })
}

fn bench_iter_std_u32() -> u64 {
    fn iter_std_u32() {
        for o in [Option::Some(1u32), Option::None]
                     .iter()
                     .cycle()
                     .take(1200) {
            for b in o.iter() {
                test::black_box(b);
            }
        }
    }

    bench!(1_000_000, {
        iter_std_u32()
    })
}

fn bench_iter_opt_u64() -> u64 {
    fn iter_opt_u64() {
        for o in [optional::some(1u64), optional::none()]
                     .iter()
                     .cycle()
                     .take(1200) {
            for b in o.iter() {
                test::black_box(b);
            }
        }
    }

    bench!(1_000_000, {
        iter_opt_u64()
    })
}

fn bench_as_slice_iter_opt_u64() -> u64 {
    fn as_slice_iter_opt_u64() {
        for o in [optional::some(1u64), optional::none()]
                     .iter()
                     .cycle()
                     .take(1200) {
            for b in o.as_slice().iter() {
                test::black_box(b);
            }
        }
    }

    bench!(1_000_000, {
        as_slice_iter_opt_u64()
    })
}

fn bench_iter_std_u64() -> u64 {
    fn iter_std_u64() {
        for o in [Option::Some(1u64), Option::None]
                     .iter()
                     .cycle()
                     .take(1200) {
            for b in o.iter() {
                test::black_box(b);
            }
        }
    }

    bench!(1_000_000, {
        iter_std_u64()
    })
}

fn bench_iter_opt_i8() -> u64 {
    fn iter_opt_i8() {
        for o in [optional::some(1i8), optional::none()]
                     .iter()
                     .cycle()
                     .take(1200) {
            for b in o.iter() {
                test::black_box(b);
            }
        }
    }

    bench!(1_000_000, {
        iter_opt_i8()
    })
}

fn bench_as_slice_iter_opt_i8() -> u64 {
    fn as_slice_iter_opt_i8() {
        for o in [optional::some(1i8), optional::none()]
                     .iter()
                     .cycle()
                     .take(1200) {
            for b in o.as_slice().iter() {
                test::black_box(b);
            }
        }
    }

    bench!(1_000_000, {
        as_slice_iter_opt_i8()
    })
}

fn bench_iter_std_i8() -> u64 {
    fn iter_std_i8() {
        for o in [Option::Some(1i8), Option::None]
                     .iter()
                     .cycle()
                     .take(1200) {
            for b in o.iter() {
                test::black_box(b);
            }
        }
    }

    bench!(1_000_000, {
        iter_std_i8()
    })
}

fn bench_iter_opt_i16() -> u64 {
    fn iter_opt_i16() {
        for o in [optional::some(1i16), optional::none()]
                     .iter()
                     .cycle()
                     .take(1200) {
            for b in o.iter() {
                test::black_box(b);
            }
        }
    }

    bench!(1_000_000, {
        iter_opt_i16()
    })
}

fn bench_as_slice_iter_opt_i16() -> u64 {
    fn as_slice_iter_opt_i16() {
        for o in [optional::some(1i16), optional::none()]
                     .iter()
                     .cycle()
                     .take(1200) {
            for b in o.as_slice().iter() {
                test::black_box(b);
            }
        }
    }

    bench!(1_000_000, {
        as_slice_iter_opt_i16()
    })
}

fn bench_iter_std_i16() -> u64 {
    fn iter_std_i16() {
        for o in [Option::Some(1i16), Option::None]
                     .iter()
                     .cycle()
                     .take(1200) {
            for b in o.iter() {
                test::black_box(b);
            }
        }
    }

    bench!(1_000_000, {
        iter_std_i16()
    })
}

fn bench_iter_opt_i32() -> u64 {
    fn iter_opt_i32() {
        for o in [optional::some(1i32), optional::none()]
                     .iter()
                     .cycle()
                     .take(1200) {
            for b in o.iter() {
                test::black_box(b);
            }
        }
    }

    bench!(1_000_000, {
        iter_opt_i32()
    })
}

fn bench_as_slice_iter_opt_i32() -> u64 {
    fn as_slice_iter_opt_i32() {
        for o in [optional::some(1i32), optional::none()]
                     .iter()
                     .cycle()
                     .take(1200) {
            for b in o.as_slice().iter() {
                test::black_box(b);
            }
        }
    }

    bench!(1_000_000, {
        as_slice_iter_opt_i32()
    })
}

fn bench_iter_std_i32() -> u64 {
    fn iter_std_i32() {
        for o in [Option::Some(1i32), Option::None]
                     .iter()
                     .cycle()
                     .take(1200) {
            for b in o.iter() {
                test::black_box(b);
            }
        }
    }

    bench!(1_000_000, {
        iter_std_i32()
    })
}

fn bench_iter_opt_i64() -> u64 {
    fn iter_opt_i64() {
        for o in [optional::some(1i64), optional::none()]
                     .iter()
                     .cycle()
                     .take(1200) {
            for b in o.iter() {
                test::black_box(b);
            }
        }
    }

    bench!(1_000_000, {
        iter_opt_i64()
    })
}

fn bench_as_slice_iter_opt_i64() -> u64 {
    fn as_slice_iter_opt_i64() {
        for o in [optional::some(1i64), optional::none()]
                     .iter()
                     .cycle()
                     .take(1200) {
            for b in o.as_slice().iter() {
                test::black_box(b);
            }
        }
    }

    bench!(1_000_000, {
        as_slice_iter_opt_i64()
    })
}

fn bench_iter_std_i64() -> u64 {
    fn iter_std_i64() {
        for o in [Option::Some(1i64), Option::None]
                     .iter()
                     .cycle()
                     .take(1200) {
            for b in o.iter() {
                test::black_box(b);
            }
        }
    }

    bench!(1_000_000, {
        iter_std_i64()
    })
}

fn bench_iter_opt_f32() -> u64 {
    fn iter_opt_f32() {
        for o in [optional::some(1f32), optional::none()]
                     .iter()
                     .cycle()
                     .take(1200) {
            for b in o.iter() {
                test::black_box(b);
            }
        }
    }

    bench!(1_000_000, {
        iter_opt_f32()
    })
}

fn bench_as_slice_iter_opt_f32() -> u64 {
    fn as_slice_iter_opt_f32() {
        for o in [optional::some(1f32), optional::none()]
                     .iter()
                     .cycle()
                     .take(1200) {
            for b in o.as_slice().iter() {
                test::black_box(b);
            }
        }
    }

    bench!(1_000_000, {
        as_slice_iter_opt_f32()
    })
}

fn bench_iter_std_f32() -> u64 {
    fn iter_std_f32() {
        for o in [Option::Some(1f32), Option::None]
                     .iter()
                     .cycle()
                     .take(1200) {
            for b in o.iter() {
                test::black_box(b);
            }
        }
    }

    bench!(1_000_000, {
        iter_std_f32()
    })
}

fn bench_iter_opt_f64() -> u64 {
    fn iter_opt_f64() {
        for o in [optional::some(1f64), optional::none()]
                     .iter()
                     .cycle()
                     .take(1200) {
            for b in o.iter() {
                test::black_box(b);
            }
        }
    }

    bench!(1_000_000, {
        iter_opt_f64()
    })
}

fn bench_as_slice_iter_opt_f64() -> u64 {
    fn as_slice_iter_opt_f64() {
        for o in [optional::some(1f64), optional::none()]
                     .iter()
                     .cycle()
                     .take(1200) {
            for b in o.as_slice().iter() {
                test::black_box(b);
            }
        }
    }

    bench!(1_000_000, {
        as_slice_iter_opt_f64()
    })
}

fn bench_iter_std_f64() -> u64 {
    fn iter_std_f64() {
        for o in [Option::Some(1f64), Option::None]
                     .iter()
                     .cycle()
                     .take(1200) {
            for b in o.iter() {
                test::black_box(b);
            }
        }
    }

    bench!(1_000_000, {
        iter_std_f64()
    })
}

fn bench_is_some_opt_u8() -> u64 {
    fn is_some_std_u8() {
        for o in [optional::some(1u8), optional::none()]
                     .iter()
                     .cycle()
                     .take(1200) {
            test::black_box(o.is_some());
        }
    }

    bench!(1_000_000, {
        is_some_std_u8()
    })
}

fn bench_is_some_std_u8() -> u64 {
    fn is_some_std_u8() {
        for o in [Option::Some(1u8), Option::None]
                     .iter()
                     .cycle()
                     .take(1200) {
            test::black_box(o.is_some());
        }
    }

    bench!(1_000_000, {
        is_some_std_u8()
    })
}

fn bench_is_some_opt_u16() -> u64 {
    fn is_some_std_u16() {
        for o in [optional::some(1u16), optional::none()]
                     .iter()
                     .cycle()
                     .take(1200) {
            test::black_box(o.is_some());
        }
    }

    bench!(1_000_000, {
        is_some_std_u16()
    })
}

fn bench_is_some_std_u16() -> u64 {
    fn is_some_std_u16() {
        for o in [Option::Some(1u16), Option::None]
                     .iter()
                     .cycle()
                     .take(1200) {
            test::black_box(o.is_some());
        }
    }

    bench!(1_000_000, {
        is_some_std_u16()
    })
}

fn bench_is_some_opt_u32() -> u64 {
    fn is_some_std_u32() {
        for o in [optional::some(1u32), optional::none()]
                     .iter()
                     .cycle()
                     .take(1200) {
            test::black_box(o.is_some());
        }
    }

    bench!(1_000_000, {
        is_some_std_u32()
    })
}

fn bench_is_some_std_u32() -> u64 {
    fn is_some_std_u32() {
        for o in [Option::Some(1u32), Option::None]
                     .iter()
                     .cycle()
                     .take(1200) {
            test::black_box(o.is_some());
        }
    }

    bench!(1_000_000, {
        is_some_std_u32()
    })
}

fn bench_is_some_opt_u64() -> u64 {
    fn is_some_std_u64() {
        for o in [optional::some(1u64), optional::none()]
                     .iter()
                     .cycle()
                     .take(1200) {
            test::black_box(o.is_some());
        }
    }

    bench!(1_000_000, {
        is_some_std_u64()
    })
}

fn bench_is_some_std_u64() -> u64 {
    fn is_some_std_u64() {
        for o in [Option::Some(1u64), Option::None]
                     .iter()
                     .cycle()
                     .take(1200) {
            test::black_box(o.is_some());
        }
    }

    bench!(1_000_000, {
        is_some_std_u64()
    })
}

fn bench_is_some_opt_f32() -> u64 {
    fn is_some_std_f32() {
        for o in [optional::some(1f32), optional::none()]
                     .iter()
                     .cycle()
                     .take(1200) {
            test::black_box(o.is_some());
        }
    }

    bench!(1_000_000, {
        is_some_std_f32()
    })
}

fn bench_is_some_std_f32() -> u64 {
    fn is_some_std_f32() {
        for o in [Option::Some(1f32), Option::None]
                     .iter()
                     .cycle()
                     .take(1200) {
            test::black_box(o.is_some());
        }
    }

    bench!(1_000_000, {
        is_some_std_f32()
    })
}

fn bench_is_some_opt_f64() -> u64 {
    fn is_some_std_f64() {
        for o in [optional::some(1f64), optional::none()]
                     .iter()
                     .cycle()
                     .take(1200) {
            test::black_box(o.is_some());
        }
    }

    bench!(1_000_000, {
        is_some_std_f64()
    })
}

fn bench_is_some_std_f64() -> u64 {
    fn is_some_std_f64() {
        for o in [Option::Some(1f64), Option::None]
                     .iter()
                     .cycle()
                     .take(1200) {
            test::black_box(o.is_some());
        }
    }

    bench!(1_000_000, {
        is_some_std_f64()
    })
}

fn bench_map_opt_u8() -> u64 {
    fn map_std_u8() {
        for o in [optional::some(1u8), optional::none()]
                     .iter()
                     .cycle()
                     .take(1200) {
            test::black_box(o.map(|x| x + 1));
        }
    }

    bench!(1_000_000, {
        map_std_u8()
    })
}

fn bench_map_std_u8() -> u64 {
    fn map_std_u8() {
        for o in [Option::Some(1u8), Option::None]
                     .iter()
                     .cycle()
                     .take(1200) {
            test::black_box(o.map(|x| x + 1));
        }
    }

    bench!(1_000_000, {
        map_std_u8()
    })
}

fn bench_map_opt_u16() -> u64 {
    fn map_std_u16() {
        for o in [optional::some(1u16), optional::none()]
                     .iter()
                     .cycle()
                     .take(1200) {
            test::black_box(o.map(|x| x + 1));
        }
    }

    bench!(1_000_000, {
        map_std_u16()
    })
}

fn bench_map_std_u16() -> u64 {
    fn map_std_u16() {
        for o in [Option::Some(1u16), Option::None]
                     .iter()
                     .cycle()
                     .take(1200) {
            test::black_box(o.map(|x| x + 1));
        }
    }

    bench!(1_000_000, {
        map_std_u16()
    })
}

fn bench_map_opt_u32() -> u64 {
    fn map_std_u32() {
        for o in [optional::some(1u32), optional::none()]
                     .iter()
                     .cycle()
                     .take(1200) {
            test::black_box(o.map(|x| x + 1));
        }
    }

    bench!(1_000_000, {
        map_std_u32()
    })
}


fn bench_map_std_u32() -> u64 {
    fn map_std_u32() {
        for o in [Option::Some(1u32), Option::None]
                     .iter()
                     .cycle()
                     .take(1200) {
            test::black_box(o.map(|x| x + 1));
        }
    }

    bench!(1_000_000, {
        map_std_u32()
    })
}

fn bench_map_opt_u64() -> u64 {
    fn map_std_u64() {
        for o in [optional::some(1u64), optional::none()]
                     .iter()
                     .cycle()
                     .take(1200) {
            test::black_box(o.map(|x| x + 1));
        }
    }

    bench!(1_000_000, {
        map_std_u64()
    })
}

fn bench_map_std_u64() -> u64 {
    fn map_std_u64() {
        for o in [Option::Some(1u64), Option::None]
                     .iter()
                     .cycle()
                     .take(1200) {
            test::black_box(o.map(|x| x + 1));
        }
    }

    bench!(1_000_000, {
        map_std_u64()
    })
}

fn bench_map_opt_f32() -> u64 {
    fn map_std_f32() {
        for o in [optional::some(1f32), optional::none()]
                     .iter()
                     .cycle()
                     .take(1200) {
            test::black_box(o.map(|x| x + 1.0));
        }
    }

    bench!(1_000_000, {
        map_std_f32()
    })
}

fn bench_map_std_f32() -> u64 {
    fn map_std_f32() {
        for o in [Option::Some(1f32), Option::None]
                     .iter()
                     .cycle()
                     .take(1200) {
            test::black_box(o.map(|x| x + 1.0));
        }
    }

    bench!(1_000_000, {
        map_std_f32()
    })
}

fn bench_map_opt_f64() -> u64 {
    fn map_std_f64() {
        for o in [optional::some(1f64), optional::none()]
                     .iter()
                     .cycle()
                     .take(1200) {
            test::black_box(o.map(|x| x + 1.0));
        }
    }

    bench!(1_000_000, {
        map_std_f64()
    })
}

fn bench_map_std_f64() -> u64 {
    fn map_std_f64() {
        for o in [Option::Some(1f64), Option::None]
                     .iter()
                     .cycle()
                     .take(1200) {
            test::black_box(o.map(|x| x + 1.0));
        }
    }

    bench!(1_000_000, {
        map_std_f64()
    })
}

fn bench_map_or_std_u8() -> u64 {
    fn map_or_std_u8() {
        for o in [Option::Some(1u8), Option::None]
                     .iter()
                     .cycle()
                     .take(1200) {
            test::black_box(o.map_or(0, |i| i + 1));
        }
    }

    bench!(1_000_000, {
        map_or_std_u8()
    })
}

fn bench_map_or_std_u16() -> u64 {
    fn map_or_std_u16() {
        for o in [Option::Some(1u16), Option::None]
                     .iter()
                     .cycle()
                     .take(1200) {
            test::black_box(o.map_or(0, |i| i + 1));
        }
    }

    bench!(1_000_000, {
        map_or_std_u16()
    })
}

fn bench_map_or_std_u32() -> u64 {
    fn map_or_std_u32() {
        for o in [Option::Some(1u32), Option::None]
                     .iter()
                     .cycle()
                     .take(1200) {
            test::black_box(o.map_or(0, |i| i + 1));
        }
    }

    bench!(1_000_000, {
        map_or_std_u32()
    })
}

fn bench_map_or_std_u64() -> u64 {
    fn map_or_std_u64() {
        for o in [Option::Some(1u64), Option::None]
                     .iter()
                     .cycle()
                     .take(1200) {
            test::black_box(o.map_or(0, |i| i + 1));
        }
    }

    bench!(1_000_000, {
        map_or_std_u64()
    })
}


fn bench_map_or_std_f32() -> u64 {
    fn map_or_std_f32() {
        for o in [Option::Some(1.0f32), Option::None]
                     .iter()
                     .cycle()
                     .take(1200) {
            test::black_box(o.map_or(0.0, |i| i + 1.0));
        }
    }

    bench!(1_000_000, {
        map_or_std_f32()
    })
}

fn bench_map_or_std_f64() -> u64 {
    fn map_or_std_f64() {
        for o in [Option::Some(1.0f64), Option::None]
                     .iter()
                     .cycle()
                     .take(1200) {
            test::black_box(o.map_or(0.0, |i| i + 1.0));
        }
    }

    bench!(1_000_000, {
        map_or_std_f64()
    })
}

fn bench_map_or_opt_u8() -> u64 {
    fn map_or_opt_u8() {
        for o in [optional::some(1u8), optional::none()]
                     .iter()
                     .cycle()
                     .take(1200) {
            test::black_box(o.map_or(0, |i| i + 1));
        }
    }

    bench!(1_000_000, {
        map_or_opt_u8()
    })
}

fn bench_map_or_opt_u16() -> u64 {
    fn map_or_opt_u16() {
        for o in [optional::some(1u16), optional::none()]
                     .iter()
                     .cycle()
                     .take(1200) {
            test::black_box(o.map_or(0, |i| i + 1));
        }
    }

    bench!(1_000_000, {
        map_or_opt_u16()
    })
}

fn bench_map_or_opt_u32() -> u64 {
    fn map_or_opt_u32() {
        for o in [optional::some(1u32), optional::none()]
                     .iter()
                     .cycle()
                     .take(1200) {
            test::black_box(o.map_or(0, |i| i + 1));
        }
    }

    bench!(1_000_000, {
        map_or_opt_u32()
    })
}

fn bench_map_or_opt_u64() -> u64 {
    fn map_or_opt_u64() {
        for o in [optional::some(1u64), optional::none()]
                     .iter()
                     .cycle()
                     .take(1200) {
            test::black_box(o.map_or(0, |i| i + 1));
        }
    }

    bench!(1_000_000, {
        map_or_opt_u64()
    })
}


fn bench_map_or_opt_f32() -> u64 {
    fn map_or_opt_f32() {
        for o in [optional::some(1.0f32), optional::none()]
                     .iter()
                     .cycle()
                     .take(1200) {
            test::black_box(o.map_or(0.0, |i| i + 1.0));
        }
    }

    bench!(1_000_000, {
        map_or_opt_f32()
    })
}

fn bench_map_or_opt_f64() -> u64 {
    fn map_or_opt_f64() {
        for o in [optional::some(1.0f64), optional::none()]
                     .iter()
                     .cycle()
                     .take(1200) {
            test::black_box(o.map_or(0.0, |i| i + 1.0));
        }
    }

    bench!(1_000_000, {
        map_or_opt_f64()
    })
}
