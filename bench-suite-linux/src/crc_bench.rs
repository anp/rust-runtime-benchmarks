use std::collections::BTreeMap;
use crc::{crc32, crc64};

pub fn run_all() -> BTreeMap<String, Vec<u64>> {
    let mut results = BTreeMap::new();

    for _ in 0..3 {
        results.entry("bench_crc32_make_table".to_string())
               .or_insert(vec![])
               .push(bench_crc32_make_table());
        results.entry("bench_crc32_update_megabytes".to_string())
               .or_insert(vec![])
               .push(bench_crc32_update_megabytes());
        results.entry("bench_crc64_make_table".to_string())
               .or_insert(vec![])
               .push(bench_crc64_make_table());
        results.entry("bench_crc64_update_megabytes".to_string())
               .or_insert(vec![])
               .push(bench_crc64_update_megabytes());
    }

    results
}

fn bench_crc32_make_table() -> u64 {
    bench!(1_000_000, {
        crc32::make_table(crc32::IEEE);
    })
}

fn bench_crc32_update_megabytes() -> u64 {
    let table = crc32::make_table(crc32::IEEE);
    let bytes = Box::new([0u8; 1_000_000]);
    bench!(1_000, {
        crc32::update(0, &table, &*bytes);
    })
}

fn bench_crc64_make_table() -> u64 {
    bench!(1_000_000, {
        crc64::make_table(crc64::ECMA);
    })
}

fn bench_crc64_update_megabytes() -> u64 {
    let table = crc64::make_table(crc64::ECMA);
    let bytes = Box::new([0u8; 1_000_000]);
    bench!(1_000, {
        crc64::update(0, &table, &*bytes);
    })
}
