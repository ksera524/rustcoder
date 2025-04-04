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
            n:usize,
            m:usize,
        }

        let sc = (0..m)
            .map(|_| {
                input! {
                    a:usize,
                    b:usize,
                }
                (a, b)
            })
            .collect::<Vec<_>>();
        let ans = (0..1000)
            .filter(|&i| {
                let s = i.to_string();
                check(&s, &sc, n)
            })
            .min();

        match ans {
            Some(i) => println!("{}", i),
            None => println!("-1"),
        }
    }
}

fn check(s: &str, sc: &[(usize, usize)], n: usize) -> bool {
    if s.len() != n {
        return false;
    }

    sc.iter().all(|&(a, b)| {
        s.chars()
            .nth(a - 1)
            .map(|ch| ch == std::char::from_digit(b as u32, 10).unwrap())
            .unwrap_or(false)
    })
}

fn main() {
    std::thread::Builder::new()
        .stack_size(128 * 1024 * 1024)
        .spawn(|| Solver::default().solve())
        .unwrap()
        .join()
        .unwrap();
}
