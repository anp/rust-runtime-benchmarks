#![allow(unused_imports)]

use ndarray::{OwnedArray, zeros};
use ndarray::{arr0, arr1, arr2};

use std::collections::BTreeMap;
use test::black_box;

pub fn run_all() -> BTreeMap<String, Vec<u64>> {
    let mut results = BTreeMap::new();

    for _ in 0..3 {
        results.entry("small_iter_1d".to_string()).or_insert(vec![]).push(small_iter_1d());
        results.entry("small_iter_1d_raw".to_string()).or_insert(vec![]).push(small_iter_1d_raw());
        results.entry("small_iter_2d".to_string()).or_insert(vec![]).push(small_iter_2d());
        results.entry("sum_1d_regular".to_string()).or_insert(vec![]).push(sum_1d_regular());
        results.entry("sum_1d_raw".to_string()).or_insert(vec![]).push(sum_1d_raw());
        results.entry("sum_2d_regular".to_string()).or_insert(vec![]).push(sum_2d_regular());
        results.entry("sum_2d_by_row".to_string()).or_insert(vec![]).push(sum_2d_by_row());
        results.entry("sum_2d_cutout".to_string()).or_insert(vec![]).push(sum_2d_cutout());
        results.entry("sum_2d_cutout_fold".to_string())
               .or_insert(vec![])
               .push(sum_2d_cutout_fold());
        results.entry("scalar_sum_2d_regular".to_string())
               .or_insert(vec![])
               .push(scalar_sum_2d_regular());
        results.entry("scalar_sum_2d_cutout".to_string())
               .or_insert(vec![])
               .push(scalar_sum_2d_cutout());
        results.entry("sum_2d_cutout_by_row".to_string())
               .or_insert(vec![])
               .push(sum_2d_cutout_by_row());
        results.entry("sum_2d_cutout_outer_iter".to_string())
               .or_insert(vec![])
               .push(sum_2d_cutout_outer_iter());
        results.entry("sum_2d_transpose_regular".to_string())
               .or_insert(vec![])
               .push(sum_2d_transpose_regular());
        results.entry("sum_2d_transpose_by_row".to_string())
               .or_insert(vec![])
               .push(sum_2d_transpose_by_row());
        results.entry("scalar_sum_2d_float".to_string())
               .or_insert(vec![])
               .push(scalar_sum_2d_float());
        results.entry("scalar_sum_2d_float_cutout".to_string())
               .or_insert(vec![])
               .push(scalar_sum_2d_float_cutout());
        results.entry("add_2d_regular".to_string()).or_insert(vec![]).push(add_2d_regular());
        results.entry("add_2d_cutout".to_string()).or_insert(vec![]).push(add_2d_cutout());
        results.entry("add_2d_broadcast_1_to_2".to_string())
               .or_insert(vec![])
               .push(add_2d_broadcast_1_to_2());
        results.entry("add_2d_broadcast_0_to_2".to_string())
               .or_insert(vec![])
               .push(add_2d_broadcast_0_to_2());
        results.entry("add_2d_0_to_2_iadd_scalar".to_string())
               .or_insert(vec![])
               .push(add_2d_0_to_2_iadd_scalar());
        results.entry("add_2d_transposed".to_string()).or_insert(vec![]).push(add_2d_transposed());
        results.entry("add_2d_f32_regular".to_string())
               .or_insert(vec![])
               .push(add_2d_f32_regular());
        results.entry("assign_scalar_2d_large".to_string())
               .or_insert(vec![])
               .push(assign_scalar_2d_large());
        results.entry("assign_scalar_2d_cutout".to_string())
               .or_insert(vec![])
               .push(assign_scalar_2d_cutout());
        results.entry("assign_scalar_2d_transposed_large".to_string())
               .or_insert(vec![])
               .push(assign_scalar_2d_transposed_large());
        results.entry("assign_scalar_2d_raw_large".to_string())
               .or_insert(vec![])
               .push(assign_scalar_2d_raw_large());
        results.entry("bench_iter_diag".to_string()).or_insert(vec![]).push(bench_iter_diag());
        results.entry("bench_row_iter".to_string()).or_insert(vec![]).push(bench_row_iter());
        results.entry("bench_col_iter".to_string()).or_insert(vec![]).push(bench_col_iter());
        results.entry("bench_mat_mul_large".to_string())
               .or_insert(vec![])
               .push(bench_mat_mul_large());
        results.entry("create_iter_4d".to_string()).or_insert(vec![]).push(create_iter_4d());
        results.entry("bench_to_owned_n".to_string()).or_insert(vec![]).push(bench_to_owned_n());
        results.entry("bench_to_owned_t".to_string()).or_insert(vec![]).push(bench_to_owned_t());
        results.entry("equality_i32".to_string()).or_insert(vec![]).push(equality_i32());
        results.entry("equality_f32".to_string()).or_insert(vec![]).push(equality_f32());
        results.entry("dot".to_string()).or_insert(vec![]).push(dot());
        results.entry("means".to_string()).or_insert(vec![]).push(means());
    }

    results
}

fn small_iter_1d() -> u64 {
    let a = arr1::<f32>(&[1., 2., 2., 3., 4., 4., 3., 4., 4., 3., 4., 4., 5., 6., 6.]);
    bench!(1_000_000_000, {
        for &elt in a.iter() {
            black_box(elt);
        }
    })
}

fn small_iter_1d_raw() -> u64 {
    let a = arr1::<f32>(&[1., 2., 2., 3., 4., 4., 3., 4., 4., 3., 4., 4., 5., 6., 6.]);
    bench!(1_000_000_000, {
        for &elt in a.raw_data().iter() {
            black_box(elt);
        }
    })
}

fn small_iter_2d() -> u64 {
    let a = arr2::<f32, _>(&[[1., 2., 2.], [3., 4., 4.], [3., 4., 4.], [3., 4., 4.], [5., 6., 6.]]);
    bench!(1_000_000_000, {
        for &elt in a.iter() {
            black_box(elt);
        }
    })
}

fn sum_1d_regular() -> u64 {
    let a = OwnedArray::<i32, _>::zeros(64 * 64);
    let a = black_box(a);
    bench!(1_000_000_000, {
        let mut sum = 0;
        for &elt in a.iter() {
            sum += elt;
        }
        sum
    })
}

fn sum_1d_raw() -> u64 {
    // this is autovectorized to death (= great performance)
    let a = OwnedArray::<i32, _>::zeros(64 * 64);
    let a = black_box(a);
    bench!(1_000_000_000, {
        let mut sum = 0;
        for &elt in a.raw_data() {
            sum += elt;
        }
        sum
    })
}

fn sum_2d_regular() -> u64 {
    let a = OwnedArray::<i32, _>::zeros((64, 64));
    let a = black_box(a);
    bench!(10_000_000, {
        let mut sum = 0;
        for &elt in a.iter() {
            sum += elt;
        }
        sum
    })
}

fn sum_2d_by_row() -> u64 {
    let a = OwnedArray::<i32, _>::zeros((64, 64));
    let a = black_box(a);
    bench!(10_000_000, {
        let mut sum = 0;
        for row in a.inner_iter() {
            for &elt in row {
                sum += elt;
            }
        }
        sum
    })
}

fn sum_2d_cutout() -> u64 {
    let a = OwnedArray::<i32, _>::zeros((66, 66));
    let av = a.slice(s![1..-1, 1..-1]);
    let a = black_box(av);
    bench!(100_000, {
        let mut sum = 0;
        for &elt in a.iter() {
            sum += elt;
        }
        sum
    })
}

fn sum_2d_cutout_fold() -> u64 {
    let a = OwnedArray::<i32, _>::zeros((66, 66));
    let av = a.slice(s![1..-1, 1..-1]);
    let a = black_box(av);
    bench!(10_000_000, {
        a.fold(0, |acc, elt| acc + *elt)
    })
}

fn scalar_sum_2d_regular() -> u64 {
    let a = OwnedArray::<i32, _>::zeros((64, 64));
    let a = black_box(a);
    bench!(1_000_000_000, {
        a.scalar_sum()
    })
}

fn scalar_sum_2d_cutout() -> u64 {
    let a = OwnedArray::<i32, _>::zeros((66, 66));
    let av = a.slice(s![1..-1, 1..-1]);
    let a = black_box(av);
    bench!(1_000_000_000, {
        a.scalar_sum()
    })
}

fn sum_2d_cutout_by_row() -> u64 {
    let a = OwnedArray::<i32, _>::zeros((66, 66));
    let av = a.slice(s![1..-1, 1..-1]);
    let a = black_box(av);
    bench!(10_000_000, {
        let mut sum = 0;
        for row in 0..a.shape()[0] {
            for &elt in a.row(row) {
                sum += elt;
            }
        }
        sum
    })
}

fn sum_2d_cutout_outer_iter() -> u64 {
    let a = OwnedArray::<i32, _>::zeros((66, 66));
    let av = a.slice(s![1..-1, 1..-1]);
    let a = black_box(av);
    bench!(10_000_000, {
        let mut sum = 0;
        for row in a.inner_iter() {
            for &elt in row {
                sum += elt;
            }
        }
        sum
    })
}

fn sum_2d_transpose_regular() -> u64 {
    let mut a = OwnedArray::<i32, _>::zeros((64, 64));
    a.swap_axes(0, 1);
    let a = black_box(a);
    bench!(100_000, {
        let mut sum = 0;
        for &elt in a.iter() {
            sum += elt;
        }
        sum
    })
}

fn sum_2d_transpose_by_row() -> u64 {
    let mut a = OwnedArray::<i32, _>::zeros((64, 64));
    a.swap_axes(0, 1);
    let a = black_box(a);
    bench!(100_000, {
        let mut sum = 0;
        for row in 0..a.shape()[0] {
            for &elt in a.row(row) {
                sum += elt;
            }
        }
        sum
    })
}

fn scalar_sum_2d_float() -> u64 {
    let a = OwnedArray::<f32, _>::zeros((64, 64));
    let a = black_box(a.view());
    bench!(1_000_000_000, {
        a.scalar_sum()
    })
}

fn scalar_sum_2d_float_cutout() -> u64 {
    let a = OwnedArray::<f32, _>::zeros((66, 66));
    let av = a.slice(s![1..-1, 1..-1]);
    let a = black_box(av);
    bench!(1_000_000_000, {
        a.scalar_sum()
    })
}

fn add_2d_regular() -> u64 {
    let mut a = OwnedArray::<i32, _>::zeros((64, 64));
    let b = OwnedArray::<i32, _>::zeros((64, 64));
    let bv = b.view();
    bench!(10_000_000, {
        let _x = black_box(a.view_mut() + bv);
    })
}

fn add_2d_cutout() -> u64 {
    let mut a = OwnedArray::<i32, _>::zeros((66, 66));
    let mut acut = a.slice_mut(s![1..-1, 1..-1]);
    let b = OwnedArray::<i32, _>::zeros((64, 64));
    let bv = b.view();
    bench!(10_000_000, {
        let _x = black_box(acut.view_mut() + bv);
    })
}

fn add_2d_broadcast_1_to_2() -> u64 {
    let mut a = OwnedArray::<i32, _>::zeros((64, 64));
    let b = OwnedArray::<i32, _>::zeros(64);
    let bv = b.view();
    bench!(10_000_000, {
        let _x = black_box(a.view_mut() + bv);
    })
}

fn add_2d_broadcast_0_to_2() -> u64 {
    let mut a = OwnedArray::<i32, _>::zeros((64, 64));
    let b = OwnedArray::<i32, _>::zeros(());
    let bv = b.view();
    bench!(10_000_000, {
        let _x = black_box(a.view_mut() + bv);
    })
}

// This is for comparison with add_2d_broadcast_0_to_2
fn add_2d_0_to_2_iadd_scalar() -> u64 {
    let mut a = OwnedArray::<i32, _>::zeros((64, 64));
    let n = black_box(0);
    bench!(10_000_000, {
        a.iadd_scalar(&n);
    })
}

fn add_2d_transposed() -> u64 {
    let mut a = OwnedArray::<i32, _>::zeros((64, 64));
    a.swap_axes(0, 1);
    let b = OwnedArray::<i32, _>::zeros((64, 64));
    let bv = b.view();
    bench!(100_000, {
        let _x = black_box(a.view_mut() + bv);
    })
}

fn add_2d_f32_regular() -> u64 {
    let mut a = OwnedArray::<f32, _>::zeros((64, 64));
    let b = OwnedArray::<f32, _>::zeros((64, 64));
    let bv = b.view();
    bench!(10_000_000, {
        let _x = black_box(a.view_mut() + bv);
    })
}

fn assign_scalar_2d_large() -> u64 {
    let a = OwnedArray::zeros((64, 64));
    let mut a = black_box(a);
    let s = 3.;
    bench!(100_000, {
        a.assign_scalar(&s)
    })
}

fn assign_scalar_2d_cutout() -> u64 {
    let mut a = OwnedArray::zeros((66, 66));
    let a = a.slice_mut(s![1..-1, 1..-1]);
    let mut a = black_box(a);
    let s = 3.;
    bench!(100_000, {
        a.assign_scalar(&s)
    })
}

fn assign_scalar_2d_transposed_large() -> u64 {
    let mut a = OwnedArray::zeros((64, 64));
    a.swap_axes(0, 1);
    let mut a = black_box(a);
    let s = 3.;
    bench!(100_000, {
        a.assign_scalar(&s)
    })
}

fn assign_scalar_2d_raw_large() -> u64 {
    let a = OwnedArray::zeros((64, 64));
    let mut a = black_box(a);
    let s = 3.;
    bench!(10_000_000, {
        for elt in a.raw_data_mut() {
            *elt = s;
        }
    })
}

fn bench_iter_diag() -> u64 {
    let a = OwnedArray::<f32, _>::zeros((1024, 1024));
    bench!(10_000_000, {
        for elt in a.diag() {
            black_box(elt);
        }
    })
}

fn bench_row_iter() -> u64 {
    let a = OwnedArray::<f32, _>::zeros((1024, 1024));
    let it = a.row(17);
    bench!(10_000_000, {
        for elt in it.clone() {
            black_box(elt);
        }
    })
}

fn bench_col_iter() -> u64 {
    let a = OwnedArray::<f32, _>::zeros((1024, 1024));
    let it = a.column(17);
    bench!(10_000_000, {
        for elt in it.clone() {
            black_box(elt);
        }
    })
}

fn bench_mat_mul_large() -> u64 {
    let a = OwnedArray::<f32, _>::zeros((64, 64));
    let b = OwnedArray::<f32, _>::zeros((64, 64));
    let a = black_box(a.view());
    let b = black_box(b.view());
    bench!(10_000, {
        a.mat_mul(&b)
    })
}

fn create_iter_4d() -> u64 {
    let mut a = OwnedArray::from_elem((4, 5, 3, 2), 1.0);
    a.swap_axes(0, 1);
    a.swap_axes(2, 1);
    let v = black_box(a.view());

    bench!(1_000_000_000, {
        v.into_iter()
    })
}

fn bench_to_owned_n() -> u64 {
    let a = zeros::<f32, _>((32, 32));
    bench!(10_000_000, {
        a.to_owned()
    })
}

fn bench_to_owned_t() -> u64 {
    let mut a = zeros::<f32, _>((32, 32));
    a.swap_axes(0, 1);
    bench!(100_000, {
        a.to_owned()
    })
}

fn equality_i32() -> u64 {
    let a = OwnedArray::<i32, _>::zeros((64, 64));
    let b = OwnedArray::<i32, _>::zeros((64, 64));
    bench!(1_000_000_000, {
        a == b
    })
}

fn equality_f32() -> u64 {
    let a = OwnedArray::<f32, _>::zeros((64, 64));
    let b = OwnedArray::<f32, _>::zeros((64, 64));
    bench!(1_000_000_000, {
        a == b
    })
}

fn dot() -> u64 {
    let a = OwnedArray::<f32, _>::zeros(256);
    let b = OwnedArray::<f32, _>::zeros(256);
    bench!(1_000_000_000, {
        a.dot(&b)
    })
}

fn means() -> u64 {
    let a = OwnedArray::from_iter(0..100_000i64);
    let a = a.into_shape((100, 1000)).unwrap();
    bench!(100_000, {
        a.mean(0)
    })
}
