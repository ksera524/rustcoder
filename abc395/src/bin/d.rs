#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(dead_code)]
use im_rc::HashMap;
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
            N:i32,
            Q:i32,
        }

        let mut bard = BTreeMap::new(); //key 鳩番号 value 鳩の巣番号
        let mut nests = BTreeMap::new(); //key 鳩の巣番号 value 鳩番号
        for i in 0..N {
            bard.insert(i, i);
            nests.insert(i, vec![i]);
        }

        for _ in 0..Q {
            input! {
                op:usize,
            }
            match op {
                1 => {
                    input! {
                        a:i32,
                        b:i32,
                    }
                    let a = a - 1;
                    let b = b - 1;
                    let bardInNest = bard.get(&a).unwrap().clone();
                    nests.insert(
                        bardInNest,
                        nests
                            .get(&bardInNest)
                            .unwrap()
                            .iter()
                            .filter(|&&x| x != a)
                            .map(|&x| x)
                            .collect(),
                    );
                    nests.get_mut(&b).unwrap().push(a);
                    bard.insert(a, b);
                }
                2 => {
                    input! {
                        a:i32,
                        b:i32,
                    }
                    let a = a - 1;
                    let b = b - 1;

                    let bardsInA = nests
                        .get(&a)
                        .unwrap()
                        .iter()
                        .map(|&x| x)
                        .collect::<Vec<i32>>();
                    if let bardsInB = nests
                        .get(&b)
                        .unwrap()
                        .iter()
                        .map(|&x| x)
                        .collect::<Vec<i32>>()
                    {}

                    nests.insert(a, bardsInB.clone());
                    nests.insert(b, bardsInA.clone());

                    for i in bardsInA {
                        bard.insert(i, b);
                    }
                    for i in bardsInB {
                        bard.insert(i, a);
                    }
                }
                3 => {
                    input! {
                        a:i32,
                    }
                    let a = a - 1;
                    println!("{}", bard.get(&a).unwrap() + 1);
                }
                _ => {}
            }
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
