#![feature(test)]

extern crate hwloc;
extern crate libc;
extern crate rustc_serialize;
extern crate test;

#[macro_use]
mod bencher;

extern crate cbor;
mod cbor_bench;

extern crate crc;
mod crc_bench;

extern crate csv;
mod csv_bench;

extern crate hyper;
mod hyper_bench;

extern crate itertools;
mod itertools_bench;

extern crate memchr;
mod memchr_bench;

#[macro_use(s)]
extern crate ndarray;
mod ndarray_bench;

use hwloc::{Topology, CPUBIND_PROCESS, TopologyObject, ObjectType};
use std::collections::BTreeMap;

// borrowing example from hwloc's repository
fn main() {
    let mut topo = Topology::new();

    // load the current pid through libc
    let pid = unsafe { libc::getpid() };

    // Grab last core and exctract its CpuSet
    let mut cpuset = last_core(&mut topo).cpuset().unwrap();

    // Get only one logical processor (in case the core is SMT/hyper-threaded).
    cpuset.singlify();

    // Bind to one core.
    topo.set_cpubind_for_process(pid, cpuset, CPUBIND_PROCESS).unwrap();

    println!("Process bound to core: {:?}", topo.get_cpubind_for_process(pid, CPUBIND_PROCESS).unwrap());

    let mut results = BTreeMap::new();
    //results.insert("cbor".to_string(), cbor_bench::run_all());
    //results.insert("crc".to_string(), crc_bench::run_all());
    //results.insert("csv".to_string(), csv_bench::run_all());
    //results.insert("hyper".to_string(), hyper_bench::run_all());
    //results.insert("itertools".to_string(), itertools_bench::run_all());
    //results.insert("memchr".to_string(), memchr_bench::run_all());
    results.insert("ndarray".to_string(), ndarray_bench::run_all());

    println!("{:#?}", results);
}

/// Find the last core
fn last_core(topo: &mut Topology) -> &TopologyObject {
    let core_depth = topo.depth_or_below_for_type(&ObjectType::Core).unwrap();
    let all_cores = topo.objects_at_depth(core_depth);
    all_cores.last().unwrap()
}
