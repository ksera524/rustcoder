#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(dead_code)]
use itertools::{iproduct, Itertools};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

const MOD: usize = 1e9 as usize + 7;
// const MOD: usize = 998244353;
// const MOD: usize = 2147483647;

#[macro_export]
macro_rules! max {
    ($x: expr) => ($x);
    ($x: expr, $( $y: expr ),+) => {
        std::cmp::max($x, max!($( $y ),+))
    }
}
#[macro_export]
macro_rules! min {
    ($x: expr) => ($x);
    ($x: expr, $( $y: expr ),+) => {
        std::cmp::min($x, min!($( $y ),+))
    }
}
#[derive(Default)]
struct Solver {}
impl Solver {
    #[fastout]
    fn solve(&mut self) {
        // let mut stdin = LineSource::new(BufReader::new(io::stdin()));
        // macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));
        input! {
            X:i32
        }
        let A = X / 1000;
        let B = (X / 100) % 10;
        let C = (X / 10) % 10;
        let D = X % 10;

        let result: Vec<_> = iproduct!(0..=1, 0..=1, 0..=1)
            .filter(|(a, b, c)| {
                let ans = A
                    + if *a == 1 { B } else { -B }
                    + if *b == 1 { C } else { -C }
                    + if *c == 1 { D } else { -D };
                ans == 7
            })
            .collect();

        if let Some(&(a, b, c)) = result.first() {
            let ops = [
                if a == 1 { '+' } else { '-' },
                if b == 1 { '+' } else { '-' },
                if c == 1 { '+' } else { '-' },
            ];
            println!("{}{}{}{}{}{}{}=7", A, ops[0], B, ops[1], C, ops[2], D);
        } else {
            println!("No solution");
        }
    }
}

fn main() {
    std::thread::Builder::new()
        .stack_size(128 * 1024 * 1024)
        .spawn(|| Solver::default().solve())
        .unwrap()
        .join()
        .unwrap();
}
