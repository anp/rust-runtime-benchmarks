use test::black_box as bb;
use simd::{f32x4, u32x4};
use std::collections::BTreeMap;

pub fn run_all() -> BTreeMap<String, Vec<u64>> {
    let mut results = BTreeMap::new();

    for _ in 0..3 {
        results.entry("multiply_naive".to_string()).or_insert(vec![]).push(multiply_naive());
        results.entry("multiply_simd4".to_string()).or_insert(vec![]).push(multiply_simd4());
        results.entry("inverse_naive".to_string()).or_insert(vec![]).push(inverse_naive());
        results.entry("inverse_simd4".to_string()).or_insert(vec![]).push(inverse_simd4());
        results.entry("transpose_naive".to_string()).or_insert(vec![]).push(transpose_naive());
        results.entry("transpose_simd4".to_string()).or_insert(vec![]).push(transpose_simd4());
        results.entry("mandel_naive".to_string()).or_insert(vec![]).push(mandel_naive());
        results.entry("mandel_simd4".to_string()).or_insert(vec![]).push(mandel_simd4());
    }

    results
}

fn multiply_naive() -> u64 {
    let x = [[1.0_f32; 4]; 4];
    let y = [[2.0; 4]; 4];
    bench!(1_000_000, {
        for _ in 0..100 {
        let (x, y) = bb((&x, &y));

        bb(&[[x[0][0] * y[0][0] + x[1][0] * y[0][1] + x[2][0] * y[0][2] + x[3][0] * y[0][3],
            x[0][1] * y[0][0] + x[1][1] * y[0][1] + x[2][1] * y[0][2] + x[3][1] * y[0][3],
            x[0][2] * y[0][0] + x[1][2] * y[0][1] + x[2][2] * y[0][2] + x[3][2] * y[0][3],
            x[0][3] * y[0][0] + x[1][3] * y[0][1] + x[2][3] * y[0][2] + x[3][3] * y[0][3]],
           [x[0][0] * y[1][0] + x[1][0] * y[1][1] + x[2][0] * y[1][2] + x[3][0] * y[1][3],
            x[0][1] * y[1][0] + x[1][1] * y[1][1] + x[2][1] * y[1][2] + x[3][1] * y[1][3],
            x[0][2] * y[1][0] + x[1][2] * y[1][1] + x[2][2] * y[1][2] + x[3][2] * y[1][3],
            x[0][3] * y[1][0] + x[1][3] * y[1][1] + x[2][3] * y[1][2] + x[3][3] * y[1][3]],
           [x[0][0] * y[2][0] + x[1][0] * y[2][1] + x[2][0] * y[2][2] + x[3][0] * y[2][3],
            x[0][1] * y[2][0] + x[1][1] * y[2][1] + x[2][1] * y[2][2] + x[3][1] * y[2][3],
            x[0][2] * y[2][0] + x[1][2] * y[2][1] + x[2][2] * y[2][2] + x[3][2] * y[2][3],
            x[0][3] * y[2][0] + x[1][3] * y[2][1] + x[2][3] * y[2][2] + x[3][3] * y[2][3]],
           [x[0][0] * y[3][0] + x[1][0] * y[3][1] + x[2][0] * y[3][2] + x[3][0] * y[3][3],
            x[0][1] * y[3][0] + x[1][1] * y[3][1] + x[2][1] * y[3][2] + x[3][1] * y[3][3],
            x[0][2] * y[3][0] + x[1][2] * y[3][1] + x[2][2] * y[3][2] + x[3][2] * y[3][3],
            x[0][3] * y[3][0] + x[1][3] * y[3][1] + x[2][3] * y[3][2] + x[3][3] * y[3][3]],
             ]);
        }
    })
}

fn multiply_simd4() -> u64 {
    let x = [f32x4::splat(1.0_f32); 4];
    let y = [f32x4::splat(2.0); 4];
    bench!(1_000_000, {
        for _ in 0..100 {
        let (x, y) = bb((&x, &y));

        let y0 = y[0];
        let y1 = y[1];
        let y2 = y[2];
        let y3 = y[3];
        bb(&[f32x4::splat(y0.extract(0)) * x[0] +
             f32x4::splat(y0.extract(1)) * x[1] +
             f32x4::splat(y0.extract(2)) * x[2] +
             f32x4::splat(y0.extract(3)) * x[3],
             f32x4::splat(y1.extract(0)) * x[0] +
             f32x4::splat(y1.extract(1)) * x[1] +
             f32x4::splat(y1.extract(2)) * x[2] +
             f32x4::splat(y1.extract(3)) * x[3],
             f32x4::splat(y2.extract(0)) * x[0] +
             f32x4::splat(y2.extract(1)) * x[1] +
             f32x4::splat(y2.extract(2)) * x[2] +
             f32x4::splat(y2.extract(3)) * x[3],
             f32x4::splat(y3.extract(0)) * x[0] +
             f32x4::splat(y3.extract(1)) * x[1] +
             f32x4::splat(y3.extract(2)) * x[2] +
             f32x4::splat(y3.extract(3)) * x[3],
             ]);
        }
    })
}

fn inverse_naive() -> u64 {
    let mut x = [[0_f32; 4]; 4];
    for i in 0..4 { x[i][i] = 1.0 }

    bench!(100_000, {
        for _ in 0..100 {
            let x = bb(&x);

            let mut t = [[0_f32; 4]; 4];
            for i in 0..4 {
                t[0][i] = x[i][0];
                t[1][i] = x[i][1];
                t[2][i] = x[i][2];
                t[3][i] = x[i][3];
            }

            let _0 = t[2][2] * t[3][3];
            let _1 = t[2][3] * t[3][2];
            let _2 = t[2][1] * t[3][3];
            let _3 = t[2][3] * t[3][1];
            let _4 = t[2][1] * t[3][2];
            let _5 = t[2][2] * t[3][1];
            let _6 = t[2][0] * t[3][3];
            let _7 = t[2][3] * t[3][0];
            let _8 = t[2][0] * t[3][2];
            let _9 = t[2][2] * t[3][0];
            let _10 = t[2][0] * t[3][1];
            let _11 = t[2][1] * t[3][0];

            let d00 = _0 * t[1][1] + _3 * t[1][2] + _4 * t[1][3] -
                (_1 * t[1][1] + _2 * t[1][2] + _5 * t[1][3]);
            let d01 = _1 * t[1][0] + _6 * t[1][2] + _9 * t[1][3] -
                (_0 * t[1][0] + _7 * t[1][2] + _8 * t[1][3]);
            let d02 = _2 * t[1][0] + _7 * t[1][1] + _10 * t[1][3] -
                (_3 * t[1][0] + _6 * t[1][1] + _11 * t[1][3]);
            let d03 = _5 * t[1][0] + _8 * t[1][1] + _11 * t[1][2] -
                (_4 * t[1][0] + _9 * t[1][1] + _10 * t[1][2]);
            let d10 = _1 * t[0][1] + _2 * t[0][2] + _5 * t[0][3] -
                (_0 * t[0][1] + _3 * t[0][2] + _4 * t[0][3]);
            let d11 = _0 * t[0][0] + _7 * t[0][2] + _8 * t[0][3] -
                (_1 * t[0][0] + _6 * t[0][2] + _9 * t[0][3]);
            let d12 = _3 * t[0][0] + _6 * t[0][1] + _11 * t[0][3] -
                (_2 * t[0][0] + _7 * t[0][1] + _10 * t[0][3]);
            let d13 = _4 * t[0][0] + _9 * t[0][1] + _10 * t[0][2] -
                (_5 * t[0][0] + _8 * t[0][1] + _11 * t[0][2]);

            let _0 = t[0][2] * t[1][3];
            let _1 = t[0][3] * t[1][2];
            let _2 = t[0][1] * t[1][3];
            let _3 = t[0][3] * t[1][1];
            let _4 = t[0][1] * t[1][2];
            let _5 = t[0][2] * t[1][1];
            let _6 = t[0][0] * t[1][3];
            let _7 = t[0][3] * t[1][0];
            let _8 = t[0][0] * t[1][2];
            let _9 = t[0][2] * t[1][0];
            let _10 = t[0][0] * t[1][1];
            let _11 = t[0][1] * t[1][0];

            let d20  = _0*t[3][1]  + _3*t[3][2]  + _4*t[3][3]-
                (_1*t[3][1]  + _2*t[3][2]  + _5*t[3][3]);
            let d21  = _1*t[3][0]  + _6*t[3][2]  + _9*t[3][3]-
                (_0*t[3][0]  + _7*t[3][2]  + _8*t[3][3]);
            let d22 = _2*t[3][0]  + _7*t[3][1]  + _10*t[3][3]-
                (_3*t[3][0]  + _6*t[3][1]  + _11*t[3][3]);
            let d23 = _5*t[3][0]  + _8*t[3][1]  + _11*t[3][2]-
                (_4*t[3][0]  + _9*t[3][1]  + _10*t[3][2]);
            let d30 = _2*t[2][2]  + _5*t[2][3]  + _1*t[2][1]-
                (_4*t[2][3]  + _0*t[2][1]   + _3*t[2][2]);
            let d31 = _8*t[2][3]  + _0*t[2][0]   + _7*t[2][2]-
                (_6*t[2][2]  + _9*t[2][3]  + _1*t[2][0]);
            let d32 = _6*t[2][1]   + _11*t[2][3] + _3*t[2][0]-
                (_10*t[2][3] + _2*t[2][0]   + _7*t[2][1]);
            let d33 = _10*t[2][2] + _4*t[2][0]   + _9*t[2][1]-
                (_8*t[2][1]   + _11*t[2][2] + _5*t[2][0]);

            let det = t[0][0] * d00 + t[0][1] * d01 + t[0][2] * d02 + t[0][3] * d03;

            let det = 1.0 / det;
            let mut ret = [[d00, d01, d02, d03],
                           [d10, d11, d12, d13],
                           [d20, d21, d22, d23],
                           [d30, d31, d32, d33]];
            for i in 0..4 {
                for j in 0..4 {
                    ret[i][j] *= det;
                }
            }
            bb(&ret);
        }
    })
}

fn inverse_simd4() -> u64 {
    let mut x = [f32x4::splat(0_f32); 4];
    for i in 0..4 { x[i] = x[i].replace(i as u32, 1.0); }

    fn shuf0145(v: f32x4, w: f32x4) -> f32x4 {
        f32x4::new(v.extract(0), v.extract(1),
                   w.extract(4 - 4), w.extract(5 - 4))
    }
    fn shuf0246(v: f32x4, w: f32x4) -> f32x4 {
        f32x4::new(v.extract(0), v.extract(2),
                   w.extract(4 - 4), w.extract(6 - 4))
    }
    fn shuf1357(v: f32x4, w: f32x4) -> f32x4 {
        f32x4::new(v.extract(1), v.extract(3),
                   w.extract(5 - 4), w.extract(7 - 4))
    }
    fn shuf2367(v: f32x4, w: f32x4) -> f32x4 {
        f32x4::new(v.extract(2), v.extract(3),
                   w.extract(6 - 4), w.extract(7 - 4))
    }

    fn swiz1032(v: f32x4) -> f32x4 {
        f32x4::new(v.extract(1), v.extract(0),
                   v.extract(3), v.extract(2))
    }
    fn swiz2301(v: f32x4) -> f32x4 {
        f32x4::new(v.extract(2), v.extract(3),
                   v.extract(0), v.extract(1))
    }

    bench!(1_000_000, {
        for _ in 0..100 {
            let src0;
            let src1;
            let src2;
            let src3;
            let mut tmp1;
            let row0;
            let mut row1;
            let mut row2;
            let mut row3;
            let mut minor0;
            let mut minor1;
            let mut minor2;
            let mut minor3;
            let mut det;

            let x = bb(&x);
            src0 = x[0];
            src1 = x[1];
            src2 = x[2];
            src3 = x[3];

            tmp1 = shuf0145(src0, src1);
            row1 = shuf0145(src2, src3);
            row0 = shuf0246(tmp1, row1);
            row1 = shuf1357(row1, tmp1);

            tmp1 = shuf2367(src0, src1);
            row3 = shuf2367(src2, src3);
            row2 = shuf0246(tmp1, row3);
            row3 = shuf0246(row3, tmp1);


            tmp1 = row2 * row3;
            tmp1 = swiz1032(tmp1);
            minor0 = row1 * tmp1;
            minor1 = row0 * tmp1;
            tmp1 = swiz2301(tmp1);
            minor0 = (row1 * tmp1) - minor0;
            minor1 = (row0 * tmp1) - minor1;
            minor1 = swiz2301(minor1);


            tmp1 = row1 * row2;
            tmp1 = swiz1032(tmp1);
            minor0 = (row3 * tmp1) + minor0;
            minor3 = row0 * tmp1;
            tmp1 = swiz2301(tmp1);

            minor0 = minor0 - row3 * tmp1;
            minor3 = row0 * tmp1 - minor3;
            minor3 = swiz2301(minor3);


            tmp1 = row3 * swiz2301(row1);
            tmp1 = swiz1032(tmp1);
            row2 = swiz2301(row2);
            minor0 = row2 * tmp1 + minor0;
            minor2 = row0 * tmp1;
            tmp1 = swiz2301(tmp1);
            minor0 = minor0 - row2 * tmp1;
            minor2 = row0 * tmp1 - minor2;
            minor2 = swiz2301(minor2);


            tmp1 = row0 * row1;
            tmp1 = swiz1032(tmp1);
            minor2 = minor2 + row3 * tmp1;
            minor3 = row2 * tmp1 - minor3;
            tmp1 = swiz2301(tmp1);
            minor2 = row3 * tmp1 - minor2;
            minor3 = minor3 - row2 * tmp1;



            tmp1 = row0 * row3;
            tmp1 = swiz1032(tmp1);
            minor1 = minor1 - row2 * tmp1;
            minor2 = row1 * tmp1 + minor2;
            tmp1 = swiz2301(tmp1);
            minor1 = row2 * tmp1 + minor1;
            minor2 = minor2 - row1 * tmp1;

            tmp1 = row0 * row2;
            tmp1 = swiz1032(tmp1);
            minor1 = row3 * tmp1 + minor1;
            minor3 = minor3 - row1 * tmp1;
            tmp1 = swiz2301(tmp1);
            minor1 = minor1 - row3 * tmp1;
            minor3 = row1 * tmp1 + minor3;

            det = row0 * minor0;
            det = swiz2301(det) + det;
            det = swiz1032(det) + det;
            //tmp1 = det.approx_reciprocal(); det = tmp1 * (f32x4::splat(2.0) - det * tmp1);
            det = f32x4::splat(1.0) / det;

            bb(&[minor0 * det, minor1 * det, minor2 * det, minor3 * det]);
        }
     })

}

fn transpose_naive() -> u64 {
    let x = [[0_f32; 4]; 4];
    bench!(1_000_000, {
        for _ in 0..100 {
            let x = bb(&x);
            bb(&[[x[0][0], x[1][0], x[2][0], x[3][0]],
                 [x[0][1], x[1][1], x[2][1], x[3][1]],
                 [x[0][2], x[1][2], x[2][2], x[3][2]],
                 [x[0][3], x[1][3], x[2][3], x[3][3]]]);
        }
    })
}

fn transpose_simd4() -> u64 {
    let x = [f32x4::splat(0_f32); 4];

    fn shuf0246(v: f32x4, w: f32x4) -> f32x4 {
        f32x4::new(v.extract(0), v.extract(2),
                   w.extract(4 - 4), w.extract(6 - 4))
    }
    fn shuf1357(v: f32x4, w: f32x4) -> f32x4 {
        f32x4::new(v.extract(1), v.extract(3),
                   w.extract(5 - 4), w.extract(7 - 4))
    }
    bench!(1_000_000, {
        for _ in 0..100 {
            let x = bb(&x);
            let x0 = x[0];
            let x1 = x[1];
            let x2 = x[2];
            let x3 = x[3];

            let a0 = shuf0246(x0, x1);
            let a1 = shuf0246(x2, x3);
            let a2 = shuf1357(x0, x1);
            let a3 = shuf1357(x2, x3);

            let b0 = shuf0246(a0, a1);
            let b1 = shuf0246(a2, a3);
            let b2 = shuf1357(a0, a1);
            let b3 = shuf1357(a2, a3);
            bb(&[b0, b1, b2, b3]);
        }
    })
}

fn naive(c_x: f32, c_y: f32, max_iter: u32) -> u32 {
    let mut x = c_x;
    let mut y = c_y;
    let mut count = 0;
    while count < max_iter {
        let xy = x * y;
        let xx = x * x;
        let yy = y * y;
        let sum = xx + yy;
        if sum > 4.0 {
            break
        }
        count += 1;
        x = xx - yy + c_x;
        y = xy * 2.0 + c_y;
    }
    count
}

fn simd4(c_x: f32x4, c_y: f32x4, max_iter: u32) -> u32x4 {
    let mut x = c_x;
    let mut y = c_y;

    let mut count = u32x4::splat(0);
    for _ in 0..max_iter as usize {
        let xy = x * y;
        let xx = x * x;
        let yy = y * y;
        let sum = xx + yy;
        let mask = sum.lt(f32x4::splat(4.0));

        if !mask.any() { break }
        count = count + mask.to_i().select(u32x4::splat(1), u32x4::splat(0));

        x = xx - yy + c_x;
        y = xy + xy + c_y;
    }
    count
}

const SCALE: f32 = 3.0 / 100.0;
const N: u32 = 100;
fn mandel_naive() -> u64 {
    bench!(1_000, {
        for j in 0..100 {
            let y = -1.5 + (j as f32) * SCALE;
            for i in 0..100 {
                let x = -2.2 + (i as f32) * SCALE;
                bb(naive(x, y, N));
            }
        }
    })
}

fn mandel_simd4() -> u64 {
    let tweak = u32x4::new(0, 1, 2, 3);
    bench!(10_000, {
        for j in 0..100 {
            let y = f32x4::splat(-1.5) + f32x4::splat(SCALE) * u32x4::splat(j).to_f32();
            for i in 0..25 {
                let i = u32x4::splat(i * 4) + tweak;
                let x = f32x4::splat(-2.2) + f32x4::splat(SCALE) * i.to_f32();
                bb(simd4(x, y, N));
            }
        }
    })
}
