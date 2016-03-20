#![feature(convert, test)]

extern crate nix;
#[macro_use]
extern crate lazy_static;
extern crate libc;
extern crate num;
extern crate rustc_serialize;
extern crate serde_json;
extern crate test;

#[macro_use]
mod bencher;

extern crate cbor;
mod cbor_bench;

extern crate crc;
mod crc_bench;

extern crate csv;
mod csv_bench;

// extern crate hyper;
// mod hyper_bench;

extern crate itertools;
mod itertools_bench;

extern crate memchr;
mod memchr_bench;

// #[macro_use(s)]
// extern crate ndarray;
// mod ndarray_bench;

extern crate optional;
mod optional_bench;

extern crate permutohedron;
mod permutohedron_bench;

extern crate rand;
mod rand_bench;

extern crate regex;
extern crate regex_syntax;
mod regex_bench;

// extern crate simd;
// mod simd_bench;

extern crate suffix;
mod suffix_bench;

extern crate uuid;
mod uuid_bench;

use std::collections::BTreeMap;
use std::env::args;

// borrowing example from hwloc's repository
fn main() {
    let mut cpu_set = nix::sched::CpuSet::new();
    cpu_set.set(0);
    nix::sched::sched_setaffinity(0, &cpu_set).unwrap();

    let mut results: BTreeMap<String, BTreeMap<String, Vec<u64>>> = BTreeMap::new();

    // skip the first argument, probably the program name
    let mut args = args();
    args.next();
    for arg in args {
        let arg = arg.as_str();
        let _ = match arg {
            "cbor" => results.insert("cbor".to_string(), cbor_bench::run_all()),
            "crc" => results.insert("crc".to_string(), crc_bench::run_all()),
            "csv" => results.insert("csv".to_string(), csv_bench::run_all()),
            "itertools" => results.insert("itertools".to_string(), itertools_bench::run_all()),
            "memchr" => results.insert("memchr".to_string(), memchr_bench::run_all()),
            "optional" => results.insert("optional".to_string(), optional_bench::run_all()),
            "permutohedron" => {
                results.insert("permutohedron".to_string(), permutohedron_bench::run_all())
            }
            "rand" => results.insert("rand".to_string(), rand_bench::run_all()),
            "regex" => results.insert("regex".to_string(), regex_bench::run_all()),
            "suffix" => results.insert("suffix".to_string(), suffix_bench::run_all()),
            "uuid" => results.insert("uuid".to_string(), uuid_bench::run_all()),
            _ => panic!("invalid benchmark selected: {}", arg),
        };
    }

    println!("{}", serde_json::to_string(&results).unwrap());
}
