use test::black_box;
use std::collections::BTreeMap;

use permutohedron::{Heap, heap_recursive};

pub fn run_all() -> BTreeMap<String, Vec<u64>> {
    let mut results = BTreeMap::new();

    for _ in 0..3 {
        results.entry("heap_iterative_7".to_string()).or_insert(vec![]).push(heap_iterative_7());
        results.entry("heap_iterative_7_iter".to_string())
               .or_insert(vec![])
               .push(heap_iterative_7_iter());
        results.entry("heap_recursive_7".to_string()).or_insert(vec![]).push(heap_recursive_7());
    }

    results
}

fn heap_iterative_7() -> u64 {
    let mut data = [0; 7];
    bench!(1_000_000, {
        let mut heap = Heap::new(&mut data);
        while let Some(elt) = heap.next_permutation() {
            black_box(elt[0]);
        }
    })
}

fn heap_iterative_7_iter() -> u64 {
    let mut data = [0; 7];
    bench!(1_000_000, {
        let heap = Heap::new(&mut data);
        for elt in heap {
            black_box(elt[0]);
        }
    })
}

fn heap_recursive_7() -> u64 {
    let mut data = [0; 7];
    bench!(1_000_000, {
        heap_recursive(&mut data, |elt| {
            black_box(elt[0]);
        });
    })
}
