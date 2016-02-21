use std::collections::BTreeMap;
use uuid::Uuid;

pub fn run_all() -> BTreeMap<String, Vec<u64>> {
    let mut results = BTreeMap::new();

    for _ in 0..3 {
        results.entry("bench_parse".to_string()).or_insert(vec![]).push(bench_parse());
        results.entry("bench_parse_invalid_len".to_string())
               .or_insert(vec![])
               .push(bench_parse_invalid_len());
        results.entry("bench_parse_invalid_character".to_string())
               .or_insert(vec![])
               .push(bench_parse_invalid_character());
        results.entry("bench_parse_invalid_group_len".to_string())
               .or_insert(vec![])
               .push(bench_parse_invalid_group_len());
        results.entry("bench_parse_invalid_groups".to_string())
               .or_insert(vec![])
               .push(bench_parse_invalid_groups());
        results.entry("bench_valid_hyphenated".to_string())
               .or_insert(vec![])
               .push(bench_valid_hyphenated());
        results.entry("bench_valid_short".to_string()).or_insert(vec![]).push(bench_valid_short());
    }

    results
}

fn bench_parse() -> u64 {
    bench!(1_000_000, {
        let _ = Uuid::parse_str("");
        let _ = Uuid::parse_str("!");
        let _ = Uuid::parse_str("F9168C5E-CEB2-4faa-B6BF-329BF39FA1E45");
        let _ = Uuid::parse_str("F9168C5E-CEB2-4faa-BBF-329BF39FA1E4");
        let _ = Uuid::parse_str("F9168C5E-CEB2-4faa-BGBF-329BF39FA1E4");
        let _ = Uuid::parse_str("F9168C5E-CEB2-4faa-B6BFF329BF39FA1E4");
        let _ = Uuid::parse_str("F9168C5E-CEB2-4faa");
        let _ = Uuid::parse_str("F9168C5E-CEB2-4faaXB6BFF329BF39FA1E4");
        let _ = Uuid::parse_str("F9168C5E-CEB-24fa-eB6BFF32-BF39FA1E4");
        let _ = Uuid::parse_str("01020304-1112-2122-3132-41424344");
        let _ = Uuid::parse_str("67e5504410b1426f9247bb680e5fe0c88");
        let _ = Uuid::parse_str("67e5504410b1426f9247bb680e5fe0cg8");
        let _ = Uuid::parse_str("67e5504410b1426%9247bb680e5fe0c8");

        // Valid
        let _ = Uuid::parse_str("00000000000000000000000000000000");
        let _ = Uuid::parse_str("67e55044-10b1-426f-9247-bb680e5fe0c8");
        let _ = Uuid::parse_str("67e55044-10b1-426f-9247-bb680e5fe0c8");
        let _ = Uuid::parse_str("F9168C5E-CEB2-4faa-B6BF-329BF39FA1E4");
        let _ = Uuid::parse_str("67e5504410b1426f9247bb680e5fe0c8");
        let _ = Uuid::parse_str("01020304-1112-2122-3132-414243444546");
        let _ = Uuid::parse_str("urn:uuid:67e55044-10b1-426f-9247-bb680e5fe0c8");

        // Nil
        let _ = Uuid::parse_str("00000000000000000000000000000000");
        let _ = Uuid::parse_str("00000000-0000-0000-0000-000000000000");

        // Test error reporting
        let _ = Uuid::parse_str("67e5504410b1426f9247bb680e5fe0c");
        let _ = Uuid::parse_str("67e550X410b1426f9247bb680e5fe0cd");
        let _ = Uuid::parse_str("67e550-4105b1426f9247bb680e5fe0c");
        let _ = Uuid::parse_str("F9168C5E-CEB2-4faa-B6BF1-02BF39FA1E4");
    })
}

fn bench_parse_invalid_len() -> u64 {
    bench!(100_000_000, {
        let _ = Uuid::parse_str("F9168C5E-CEB2-4faa-BBF-329BF39FA1E4");
    })
}

fn bench_parse_invalid_character() -> u64 {
    bench!(10_000_000, {
        let _ = Uuid::parse_str("F9168C5E-CEB2-4faa-BGBF-329BF39FA1E4");
    })
}

fn bench_parse_invalid_group_len() -> u64 {
    bench!(10_000_000, {
        let _ = Uuid::parse_str("01020304-1112-2122-3132-41424344");
    })
}

fn bench_parse_invalid_groups() -> u64 {
    bench!(10_000_000, {
        let _ = Uuid::parse_str("F9168C5E-CEB2-4faa-B6BFF329BF39FA1E4");
    })
}

fn bench_valid_hyphenated() -> u64 {
    bench!(10_000_000, {
        let _ = Uuid::parse_str("67e55044-10b1-426f-9247-bb680e5fe0c8");
    })
}

fn bench_valid_short() -> u64 {
    bench!(10_000_000, {
        let _ = Uuid::parse_str("67e5504410b1426f9247bb680e5fe0c8");
    })
}
