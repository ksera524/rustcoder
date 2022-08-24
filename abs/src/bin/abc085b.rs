use proconio::{fastout, input};
use proconio::marker::*;
use std::collections::HashSet;
#[fastout]
fn main() {
    input! {
        n:usize,
        a:[i32;n],
    }
    let ans:HashSet<_> = a.iter().collect();

    println!("{}",ans.len() );
}
