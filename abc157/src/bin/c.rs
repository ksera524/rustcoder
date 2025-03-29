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

        let mut sc = vec![];
        for _ in 0..m {
            input! {
                a:usize,
                b:usize,
            }
            sc.push((a, b));
        }
        let mut ans = 1000;

        for i in 0..1000 {
            let string_i = i.to_string();
            let result = check(&string_i, &sc, &n, &m);
            if result {
                ans = min!(ans, i);
            }
        }
        if ans == 1000 {
            println!("-1");
        } else {
            println!("{}", ans);
        }
    }
}

fn check(s: &str, sc: &[(usize, usize)], n: &usize, m: &usize) -> bool {
    let chars = s.chars().collect::<Vec<_>>();
    if s.len() != *n {
        return false;
    }

    for j in 0..*m {
        let (a, b) = sc[j];
        if chars[a - 1] != (b as u8 + b'0') as char {
            return false;
        }
    }
    true
}

fn main() {
    std::thread::Builder::new()
        .stack_size(128 * 1024 * 1024)
        .spawn(|| Solver::default().solve())
        .unwrap()
        .join()
        .unwrap();
}
