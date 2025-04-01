#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(dead_code)]
use itertools::Itertools;
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
            c: [usize; 9],
        }
        if (0..=100).any(|a1| check_conditions(a1, &c)) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

fn check_conditions(a1: usize, c: &[usize]) -> bool {
    let b1 = match c[0].checked_sub(a1) {
        Some(v) => v,
        None => return false,
    };

    let a2 = match c[3].checked_sub(b1) {
        Some(v) => v,
        None => return false,
    };

    let b2 = match c[4].checked_sub(a2) {
        Some(v) => v,
        None => return false,
    };

    let a3 = match c[6].checked_sub(b1) {
        Some(v) => v,
        None => return false,
    };

    let b3 = match c[2].checked_sub(a1) {
        Some(v) => v,
        None => return false,
    };

    (c[0] == a1 + b1)
        && (c[1] == a1 + b2)
        && (c[2] == a1 + b3)
        && (c[3] == a2 + b1)
        && (c[4] == a2 + b2)
        && (c[5] == a2 + b3)
        && (c[6] == a3 + b1)
        && (c[7] == a3 + b2)
        && (c[8] == a3 + b3)
}

fn main() {
    std::thread::Builder::new()
        .stack_size(128 * 1024 * 1024)
        .spawn(|| Solver::default().solve())
        .unwrap()
        .join()
        .unwrap();
}
