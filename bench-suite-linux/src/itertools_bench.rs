use test::black_box;

use itertools;
use itertools::Stride;
use itertools::Itertools;
use itertools::ZipSlices;

use std::iter::repeat;
use std::cmp;
use std::collections::BTreeMap;

pub fn run_all() -> BTreeMap<String, Vec<u64>> {
    let mut results = BTreeMap::new();

    for _ in 0..3 {
        results.entry("slice_iter".to_string()).or_insert(vec![]).push(slice_iter());
        results.entry("slice_iter_rev".to_string()).or_insert(vec![]).push(slice_iter_rev());
        results.entry("stride_iter".to_string()).or_insert(vec![]).push(stride_iter());
        results.entry("stride_iter_rev".to_string()).or_insert(vec![]).push(stride_iter_rev());
        results.entry("zip_default_zip".to_string()).or_insert(vec![]).push(zip_default_zip());
        results.entry("zipdot_i32_default_zip".to_string())
               .or_insert(vec![])
               .push(zipdot_i32_default_zip());
        results.entry("zipdot_f32_default_zip".to_string())
               .or_insert(vec![])
               .push(zipdot_f32_default_zip());
        results.entry("zip_default_zip3".to_string()).or_insert(vec![]).push(zip_default_zip3());
        results.entry("zipslices".to_string()).or_insert(vec![]).push(zipslices());
        results.entry("zipslices_mut".to_string()).or_insert(vec![]).push(zipslices_mut());
        results.entry("zipdot_i32_zipslices".to_string())
               .or_insert(vec![])
               .push(zipdot_i32_zipslices());
        results.entry("zipdot_f32_zipslices".to_string())
               .or_insert(vec![])
               .push(zipdot_f32_zipslices());
        results.entry("zip_checked_counted_loop".to_string())
               .or_insert(vec![])
               .push(zip_checked_counted_loop());
        results.entry("zipdot_i32_checked_counted_loop".to_string())
               .or_insert(vec![])
               .push(zipdot_i32_checked_counted_loop());
        results.entry("zipdot_f32_checked_counted_loop".to_string())
               .or_insert(vec![])
               .push(zipdot_f32_checked_counted_loop());
        results.entry("zipdot_f32_checked_counted_unrolled_loop".to_string())
               .or_insert(vec![])
               .push(zipdot_f32_checked_counted_unrolled_loop());
        results.entry("zip_unchecked_counted_loop".to_string())
               .or_insert(vec![])
               .push(zip_unchecked_counted_loop());
        results.entry("zipdot_i32_unchecked_counted_loop".to_string())
               .or_insert(vec![])
               .push(zipdot_i32_unchecked_counted_loop());
        results.entry("zipdot_f32_unchecked_counted_loop".to_string())
               .or_insert(vec![])
               .push(zipdot_f32_unchecked_counted_loop());
        results.entry("zip_unchecked_counted_loop3".to_string())
               .or_insert(vec![])
               .push(zip_unchecked_counted_loop3());
        results.entry("group_by_lazy_1".to_string()).or_insert(vec![]).push(group_by_lazy_1());
        results.entry("group_by_lazy_2".to_string()).or_insert(vec![]).push(group_by_lazy_2());
        results.entry("slice_chunks".to_string()).or_insert(vec![]).push(slice_chunks());
        results.entry("chunks_lazy_1".to_string()).or_insert(vec![]).push(chunks_lazy_1());
        results.entry("equal".to_string()).or_insert(vec![]).push(equal());
        results.entry("merge_default".to_string()).or_insert(vec![]).push(merge_default());
        results.entry("merge_by_cmp".to_string()).or_insert(vec![]).push(merge_by_cmp());
        results.entry("merge_by_lt".to_string()).or_insert(vec![]).push(merge_by_lt());
    }

    results
}

fn slice_iter() -> u64 {
    let xs: Vec<_> = repeat(1i32).take(20).collect();
    bench!(100_000_000, {
        for elt in xs.iter() {
            black_box(elt);
        }
    })
}

fn slice_iter_rev() -> u64 {
    let xs: Vec<_> = repeat(1i32).take(20).collect();
    bench!(100_000_000, {
        for elt in xs.iter().rev() {
            black_box(elt);
        }
    })
}

fn stride_iter() -> u64 {
    let xs: Vec<_> = repeat(1i32).take(20).collect();
    bench!(100_000_000, {
        for elt in Stride::from_slice(&xs, 1) {
            black_box(elt);
        }
    })
}

fn stride_iter_rev() -> u64 {
    let xs: Vec<_> = repeat(1i32).take(20).collect();
    bench!(100_000_000, {
        for elt in Stride::from_slice(&xs, 1).rev() {
            black_box(elt);
        }
    })
}

fn zip_default_zip() -> u64 {
    let xs = vec![0; 1024];
    let ys = vec![0; 768];
    let xs = black_box(xs);
    let ys = black_box(ys);

    bench!(1_000_000, {
        for (&x, &y) in xs.iter().zip(&ys) {
            black_box(x);
            black_box(y);
        }
    })
}

fn zipdot_i32_default_zip() -> u64 {
    let xs = vec![2; 1024];
    let ys = vec![2; 768];
    let xs = black_box(xs);
    let ys = black_box(ys);

    bench!(1_000_000_000, {
        let mut s = 0;
        for (&x, &y) in xs.iter().zip(&ys) {
            s += x * y;
        }
    })
}

fn zipdot_f32_default_zip() -> u64 {
    let xs = vec![2f32; 1024];
    let ys = vec![2f32; 768];
    let xs = black_box(xs);
    let ys = black_box(ys);

    bench!(100_000_000, {
        let mut s = 0.;
        for (&x, &y) in xs.iter().zip(&ys) {
            s += x * y;
        }
        s
    })
}

fn zip_default_zip3() -> u64 {
    let xs = vec![0; 1024];
    let ys = vec![0; 768];
    let zs = vec![0; 766];
    let xs = black_box(xs);
    let ys = black_box(ys);
    let zs = black_box(zs);

    bench!(1_000_000, {
        for ((&x, &y), &z) in xs.iter().zip(&ys).zip(&zs) {
            black_box(x);
            black_box(y);
            black_box(z);
        }
    })
}

fn zipslices() -> u64 {
    let xs = vec![0; 1024];
    let ys = vec![0; 768];
    let xs = black_box(xs);
    let ys = black_box(ys);

    bench!(1_000_000, {
        for (&x, &y) in ZipSlices::new(&xs, &ys) {
            black_box(x);
            black_box(y);
        }
    })
}

fn zipslices_mut() -> u64 {
    let xs = vec![0; 1024];
    let ys = vec![0; 768];
    let xs = black_box(xs);
    let mut ys = black_box(ys);

    bench!(1_000_000, {
        for (&x, &mut y) in ZipSlices::from_slices(&xs[..], &mut ys[..]) {
            black_box(x);
            black_box(y);
        }
    })
}

fn zipdot_i32_zipslices() -> u64 {
    let xs = vec![2; 1024];
    let ys = vec![2; 768];
    let xs = black_box(xs);
    let ys = black_box(ys);

    bench!(1_000_000_000, {
        let mut s = 0i32;
        for (&x, &y) in ZipSlices::new(&xs, &ys) {
            s += x * y;
        }
        s
    })
}

fn zipdot_f32_zipslices() -> u64 {
    let xs = vec![2f32; 1024];
    let ys = vec![2f32; 768];
    let xs = black_box(xs);
    let ys = black_box(ys);

    bench!(100_000_000, {
        let mut s = 0.;
        for (&x, &y) in ZipSlices::new(&xs, &ys) {
            s += x * y;
        }
        s
    })
}

fn zip_checked_counted_loop() -> u64 {
    let xs = vec![0; 1024];
    let ys = vec![0; 768];
    let xs = black_box(xs);
    let ys = black_box(ys);

    bench!(1_000_000, {
        // Must slice to equal lengths, and then bounds checks are eliminated!
        let len = cmp::min(xs.len(), ys.len());
        let xs = &xs[..len];
        let ys = &ys[..len];

        for i in 0..len {
            let x = xs[i];
            let y = ys[i];
            black_box(x);
            black_box(y);
        }
    })
}

fn zipdot_i32_checked_counted_loop() -> u64 {
    let xs = vec![2; 1024];
    let ys = vec![2; 768];
    let xs = black_box(xs);
    let ys = black_box(ys);

    bench!(1_000_000_000, {
        // Must slice to equal lengths, and then bounds checks are eliminated!
        let len = cmp::min(xs.len(), ys.len());
        let xs = &xs[..len];
        let ys = &ys[..len];

        let mut s = 0i32;

        for i in 0..len {
            s += xs[i] * ys[i];
        }
        s
    })
}

fn zipdot_f32_checked_counted_loop() -> u64 {
    let xs = vec![2f32; 1024];
    let ys = vec![2f32; 768];
    let xs = black_box(xs);
    let ys = black_box(ys);

    bench!(100_000_000, {
        // Must slice to equal lengths, and then bounds checks are eliminated!
        let len = cmp::min(xs.len(), ys.len());
        let xs = &xs[..len];
        let ys = &ys[..len];

        let mut s = 0.;

        for i in 0..len {
            s += xs[i] * ys[i];
        }
        s
    })
}

fn zipdot_f32_checked_counted_unrolled_loop() -> u64 {
    let xs = vec![2f32; 1024];
    let ys = vec![2f32; 768];
    let xs = black_box(xs);
    let ys = black_box(ys);

    bench!(100_000_000, {
        // Must slice to equal lengths, and then bounds checks are eliminated!
        let len = cmp::min(xs.len(), ys.len());
        let mut xs = &xs[..len];
        let mut ys = &ys[..len];

        let mut s = 0.;
        let (mut p0,
             mut p1,
             mut p2,
             mut p3,
             mut p4,
             mut p5,
             mut p6,
             mut p7) = (0., 0., 0., 0., 0., 0., 0., 0.);

        // how to unroll and have bounds checks eliminated (by cristicbz)
        // split sum into eight parts to enable vectorization (by bluss)
        while xs.len() >= 8 {
            p0 += xs[0] * ys[0];
            p1 += xs[1] * ys[1];
            p2 += xs[2] * ys[2];
            p3 += xs[3] * ys[3];
            p4 += xs[4] * ys[4];
            p5 += xs[5] * ys[5];
            p6 += xs[6] * ys[6];
            p7 += xs[7] * ys[7];

            xs = &xs[8..];
            ys = &ys[8..];
        }
        s += p0 + p4;
        s += p1 + p5;
        s += p2 + p6;
        s += p3 + p7;

        for i in 0..xs.len() {
            s += xs[i] * ys[i];
        }
        s
    })
}

fn zip_unchecked_counted_loop() -> u64 {
    let xs = vec![0; 1024];
    let ys = vec![0; 768];
    let xs = black_box(xs);
    let ys = black_box(ys);

    bench!(1_000_000, {
        let len = cmp::min(xs.len(), ys.len());
        for i in 0..len {
            unsafe {
                let x = *xs.get_unchecked(i);
                let y = *ys.get_unchecked(i);
                black_box(x);
                black_box(y);
            }
        }
    })
}

fn zipdot_i32_unchecked_counted_loop() -> u64 {
    let xs = vec![2; 1024];
    let ys = vec![2; 768];
    let xs = black_box(xs);
    let ys = black_box(ys);

    bench!(1_000_000_000, {
        let len = cmp::min(xs.len(), ys.len());
        let mut s = 0i32;
        for i in 0..len {
            unsafe {
                let x = *xs.get_unchecked(i);
                let y = *ys.get_unchecked(i);
                s += x * y;
            }
        }
        s
    })
}

fn zipdot_f32_unchecked_counted_loop() -> u64 {
    let xs = vec![2.; 1024];
    let ys = vec![2.; 768];
    let xs = black_box(xs);
    let ys = black_box(ys);

    bench!(100_000_000, {
        let len = cmp::min(xs.len(), ys.len());
        let mut s = 0f32;
        for i in 0..len {
            unsafe {
                let x = *xs.get_unchecked(i);
                let y = *ys.get_unchecked(i);
                s += x * y;
            }
        }
        s
    })
}

fn zip_unchecked_counted_loop3() -> u64 {
    let xs = vec![0; 1024];
    let ys = vec![0; 768];
    let zs = vec![0; 766];
    let xs = black_box(xs);
    let ys = black_box(ys);
    let zs = black_box(zs);

    bench!(1_000_000, {
        let len = cmp::min(xs.len(), cmp::min(ys.len(), zs.len()));
        for i in 0..len {
            unsafe {
                let x = *xs.get_unchecked(i);
                let y = *ys.get_unchecked(i);
                let z = *zs.get_unchecked(i);
                black_box(x);
                black_box(y);
                black_box(z);
            }
        }
    })
}

fn group_by_lazy_1() -> u64 {
    let mut data = vec![0; 1024];
    for (index, elt) in data.iter_mut().enumerate() {
        *elt = index / 10;
    }

    let data = black_box(data);

    bench!(1_000_000, {
        for (_key, group) in &data.iter().group_by_lazy(|elt| **elt) {
            for elt in group {
                black_box(elt);
            }
        }
    })
}

fn group_by_lazy_2() -> u64 {
    let mut data = vec![0; 1024];
    for (index, elt) in data.iter_mut().enumerate() {
        *elt = index / 2;
    }

    let data = black_box(data);

    bench!(1_000_000, {
        for (_key, group) in &data.iter().group_by_lazy(|elt| **elt) {
            for elt in group {
                black_box(elt);
            }
        }
    })
}

fn slice_chunks() -> u64 {
    let data = vec![0; 1024];

    let data = black_box(data);
    let sz = black_box(10);

    bench!(1_000_000, {
        for group in data.chunks(sz) {
            for elt in group {
                black_box(elt);
            }
        }
    })
}

fn chunks_lazy_1() -> u64 {
    let data = vec![0; 1024];

    let data = black_box(data);
    let sz = black_box(10);

    bench!(1_000_000, {
        for group in &data.iter().chunks_lazy(sz) {
            for elt in group {
                black_box(elt);
            }
        }
    })
}

fn equal() -> u64 {
    let data = vec![7; 1024];
    let l = data.len();
    let alpha = black_box(&data[1..]);
    let beta = black_box(&data[..l - 1]);
    bench!(100_000_000, {
        itertools::equal(alpha, beta)
    })
}

fn merge_default() -> u64 {
    let mut data1 = vec![0; 1024];
    let mut data2 = vec![0; 800];
    let mut x = 0;
    for (_, elt) in data1.iter_mut().enumerate() {
        *elt = x;
        x += 1;
    }

    let mut y = 0;
    for (i, elt) in data2.iter_mut().enumerate() {
        *elt += y;
        if i % 3 == 0 {
            y += 3;
        } else {
            y += 0;
        }
    }
    let data1 = black_box(data1);
    let data2 = black_box(data2);
    bench!(1_000_000, {
        data1.iter().merge(&data2).count()
    })
}

fn merge_by_cmp() -> u64 {
    let mut data1 = vec![0; 1024];
    let mut data2 = vec![0; 800];
    let mut x = 0;
    for (_, elt) in data1.iter_mut().enumerate() {
        *elt = x;
        x += 1;
    }

    let mut y = 0;
    for (i, elt) in data2.iter_mut().enumerate() {
        *elt += y;
        if i % 3 == 0 {
            y += 3;
        } else {
            y += 0;
        }
    }
    let data1 = black_box(data1);
    let data2 = black_box(data2);
    bench!(1_000_000, {
        data1.iter().merge_by(&data2, PartialOrd::le).count()
    })
}

fn merge_by_lt() -> u64 {
    let mut data1 = vec![0; 1024];
    let mut data2 = vec![0; 800];
    let mut x = 0;
    for (_, elt) in data1.iter_mut().enumerate() {
        *elt = x;
        x += 1;
    }

    let mut y = 0;
    for (i, elt) in data2.iter_mut().enumerate() {
        *elt += y;
        if i % 3 == 0 {
            y += 3;
        } else {
            y += 0;
        }
    }
    let data1 = black_box(data1);
    let data2 = black_box(data2);
    bench!(1_000_000, {
        data1.iter().merge_by(&data2, |a, b| a <= b).count()
    })
}
