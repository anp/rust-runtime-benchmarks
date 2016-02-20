use std::collections::BTreeMap;
use suffix::SuffixTable;

pub fn run_all() -> BTreeMap<String, Vec<u64>> {
    let mut results = BTreeMap::new();

    for _ in 0..3 {
        results.entry("naive_small".to_string()).or_insert(vec![]).push(naive_small());
        results.entry("sais_small".to_string()).or_insert(vec![]).push(sais_small());
        results.entry("naive_dna_small".to_string()).or_insert(vec![]).push(naive_dna_small());
        results.entry("sais_dna_small".to_string()).or_insert(vec![]).push(sais_dna_small());
        results.entry("naive_dna_medium".to_string()).or_insert(vec![]).push(naive_dna_medium());
        results.entry("sais_dna_medium".to_string()).or_insert(vec![]).push(sais_dna_medium());
        results.entry("search_scan_not_exists".to_string())
               .or_insert(vec![])
               .push(search_scan_not_exists());
        results.entry("search_suffix_not_exists".to_string())
               .or_insert(vec![])
               .push(search_suffix_not_exists());
        results.entry("search_suffix_not_exists_contains".to_string())
               .or_insert(vec![])
               .push(search_suffix_not_exists_contains());
        results.entry("search_scan_exists_one".to_string())
               .or_insert(vec![])
               .push(search_scan_exists_one());
        results.entry("search_suffix_exists_one".to_string())
               .or_insert(vec![])
               .push(search_suffix_exists_one());
        results.entry("search_suffix_exists_one_contains".to_string())
               .or_insert(vec![])
               .push(search_suffix_exists_one_contains());
        results.entry("search_scan_exists_many".to_string())
               .or_insert(vec![])
               .push(search_scan_exists_many());
        results.entry("search_suffix_exists_many".to_string())
               .or_insert(vec![])
               .push(search_suffix_exists_many());
        results.entry("search_suffix_exists_many_contains".to_string())
               .or_insert(vec![])
               .push(search_suffix_exists_many_contains());
    }

    results
}

fn naive_small() -> u64 {
    let s = "mississippi";
    bench!(10_000_000, {
        SuffixTable::new_naive(s);
    })
}

fn sais_small() -> u64 {
    let s = "mississippi";
    bench!(1_000_000, {
        SuffixTable::new(s);
    })
}

fn naive_dna_small() -> u64 {
    let s = include_str!("AP009048_10000.fasta");
    bench!(1_000, {
        SuffixTable::new_naive(s);
    })
}

fn sais_dna_small() -> u64 {
    let s = include_str!("AP009048_10000.fasta");
    bench!(1_000, {
        SuffixTable::new(s);
    })
}

fn naive_dna_medium() -> u64 {
    let s = include_str!("AP009048_100000.fasta");
    bench!(100_000_000, {
        SuffixTable::new_naive(s);
    })
}

fn sais_dna_medium() -> u64 {
    let s = include_str!("AP009048_100000.fasta");
    bench!(1_000_000, {
        SuffixTable::new(s);
    })
}

fn search_scan_not_exists() -> u64 {
    let s = include_str!("AP009048_100000.fasta");
    bench!(100_000_000, {
        s.contains("H");
    })
}

fn search_suffix_not_exists() -> u64 {
    let s = include_str!("AP009048_100000.fasta");
    let st = SuffixTable::new(s);
    bench!(100_000_000, {
        st.positions("H");
    })
}

fn search_suffix_not_exists_contains() -> u64 {
    let s = include_str!("AP009048_100000.fasta");
    let st = SuffixTable::new(s);
    bench!(100_000_000, {
        st.contains("H");
    })
}

fn search_scan_exists_one() -> u64 {
    let s = include_str!("AP009048_100000.fasta");
    bench!(100_000_000, {
        s.contains("C");
    })
}

fn search_suffix_exists_one() -> u64 {
    let s = include_str!("AP009048_100000.fasta");
    let st = SuffixTable::new(s);
    bench!(100_000_000, {
        st.positions("C");
    })
}

fn search_suffix_exists_one_contains() -> u64 {
    let s = include_str!("AP009048_100000.fasta");
    let st = SuffixTable::new(s);
    bench!(100_000_000, {
        st.contains("C");
    })
}

fn search_scan_exists_many() -> u64 {
    let s = include_str!("AP009048_100000.fasta");
    bench!(10_000_000, {
        s.contains("ACTTACGTGTCTGC");
    })
}

fn search_suffix_exists_many() -> u64 {
    let s = include_str!("AP009048_100000.fasta");
    let st = SuffixTable::new(s);
    bench!(100_000_000, {
        st.positions("ACTTACGTGTCTGC");
    })
}

fn search_suffix_exists_many_contains() -> u64 {
    let s = include_str!("AP009048_100000.fasta");
    let st = SuffixTable::new(s);
    bench!(100_000_000, {
        st.contains("ACTTACGTGTCTGC");
    })
}
