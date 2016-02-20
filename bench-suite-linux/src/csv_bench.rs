use std::collections::BTreeMap;
use std::fmt::{Debug, Display};
use std::fs;
use std::io::{self, Read, Seek};

use csv::Reader;

static CSV_DATA: &'static str = "./src/csv_bench.csv";

fn ordie<T, E: Debug + Display>(r: Result<T, E>) -> T {
    r.or_else(|e: E| -> Result<T, E> { panic!(format!("{:?}", e)) }).unwrap()
}

fn file_to_mem(fp: &str) -> io::Cursor<Vec<u8>> {
    let mut f = ordie(fs::File::open(fp));
    let mut bs = vec![];
    ordie(f.read_to_end(&mut bs));
    io::Cursor::new(bs)
}

fn reader<'a>(rdr: &'a mut io::Cursor<Vec<u8>>) -> Reader<&'a mut io::Cursor<Vec<u8>>> {
    let _ = ordie(rdr.seek(io::SeekFrom::Start(0)));
    Reader::from_reader(rdr.by_ref())
}

pub fn run_all() -> BTreeMap<String, Vec<u64>> {
    let mut results = BTreeMap::new();

    for _ in 0..3 {
        results.entry("raw_records".to_string()).or_insert(vec![]).push(raw_records());
        results.entry("byte_records".to_string()).or_insert(vec![]).push(byte_records());
        results.entry("string_records".to_string()).or_insert(vec![]).push(string_records());
        results.entry("decoded_records".to_string()).or_insert(vec![]).push(decoded_records());
    }

    results
}

fn raw_records() -> u64 {
    let mut data = file_to_mem(CSV_DATA);
    bench!(1_000, {
        let mut dec = reader(&mut data);
        while !dec.done() {
            while let Some(r) = dec.next_bytes().into_iter_result() {
                r.unwrap();
            }
        }
    })
}

fn byte_records() -> u64 {
    let mut data = file_to_mem(CSV_DATA);
    bench!(1_000, {
        let mut dec = reader(&mut data);
        for r in dec.byte_records() {
            let _ = r.unwrap();
        }
    })
}

fn string_records() -> u64 {
    let mut data = file_to_mem(CSV_DATA);
    bench!(100, {
        let mut dec = reader(&mut data);
        for r in dec.records() {
            let _ = r.unwrap();
        }
    })
}

#[allow(dead_code)]
#[derive(RustcDecodable)]
struct Play {
    gameid: String,
    qtr: i32,
    min: Option<i32>,
    sec: Option<i32>,
    team_off: String,
    team_def: String,
    down: Option<i32>,
    togo: Option<i32>,
    ydline: Option<i32>,
    description: String,
    offscore: i32,
    defscore: i32,
    season: i32,
}

fn decoded_records() -> u64 {
    let mut data = file_to_mem(CSV_DATA);
    bench!(100, {
        let mut dec = reader(&mut data);
        for r in dec.decode::<Play>() {
            let _ = r.unwrap();
        }
    })
}
