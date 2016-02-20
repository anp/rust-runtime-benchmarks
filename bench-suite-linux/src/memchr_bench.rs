use std::collections::BTreeMap;
use std::iter;

use memchr;

fn bench_data() -> Vec<u8> {
    iter::repeat(b'z').take(10000).collect()
}

pub fn run_all() -> BTreeMap<String, Vec<u64>> {
    let mut results = BTreeMap::new();

    for _ in 0..3 {
        results.entry("iterator_memchr".to_string()).or_insert(vec![]).push(iterator_memchr());
        results.entry("optimized_memchr".to_string()).or_insert(vec![]).push(optimized_memchr());
        results.entry("iterator_memrchr".to_string()).or_insert(vec![]).push(iterator_memrchr());
        results.entry("optimized_memrchr".to_string()).or_insert(vec![]).push(optimized_memrchr());
        results.entry("iterator_memchr2".to_string()).or_insert(vec![]).push(iterator_memchr2());
        results.entry("optimized_memchr2".to_string()).or_insert(vec![]).push(optimized_memchr2());
        results.entry("iterator_memchr3".to_string()).or_insert(vec![]).push(iterator_memchr3());
        results.entry("optimized_memchr3".to_string()).or_insert(vec![]).push(optimized_memchr3());
    }

    results
}

fn iterator_memchr() -> u64 {
    let haystack = bench_data();
    let needle = b'a';
    bench!(100_000, {
        assert!(haystack.iter().position(|&b| b == needle).is_none());
    })
}

fn optimized_memchr() -> u64 {
    let haystack = bench_data();
    let needle = b'a';
    bench!(1_000_000, {
        assert!(memchr::memchr(needle, &haystack).is_none());
    })
}

fn iterator_memrchr() -> u64 {
    let haystack = bench_data();
    let needle = b'a';
    bench!(100_000, {
        assert!(haystack.iter().rposition(|&b| b == needle).is_none());
    })
}

fn optimized_memrchr() -> u64 {
    let haystack = bench_data();
    let needle = b'a';
    bench!(10_000_000, {
        assert!(memchr::memrchr(needle, &haystack).is_none());
    })
}

fn iterator_memchr2() -> u64 {
    let haystack = bench_data();
    let (needle1, needle2) = (b'a', b'b');
    bench!(100_000, {
        assert!(haystack.iter()
                        .position(|&b| b == needle1 || b == needle2)
                        .is_none());
    })
}

fn optimized_memchr2() -> u64 {
    let haystack = bench_data();
    let (needle1, needle2) = (b'a', b'b');
    bench!(1_000_000, {
        assert!(memchr::memchr2(needle1, needle2, &haystack).is_none());
    })
}

fn iterator_memchr3() -> u64 {
    let haystack = bench_data();
    let (needle1, needle2, needle3) = (b'a', b'b', b'c');
    bench!(100_000, {
        assert!(haystack.iter()
                        .position(|&b| b == needle1 || b == needle2 || b == needle3)
                        .is_none());
    })
}

fn optimized_memchr3() -> u64 {
    let haystack = bench_data();
    let (needle1, needle2, needle3) = (b'a', b'b', b'c');
    bench!(1_000_000, {
        assert!(memchr::memchr3(needle1, needle2, needle3, &haystack).is_none());
    })
}
