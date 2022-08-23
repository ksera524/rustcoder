use proconio::{fastout, input};
use std::collections::HashSet;
#[fastout]
fn main() {
    input! {
        n:usize,
        d:[i32;n],
    }
    let y:HashSet<_> =  d.iter().collect();
    
    println!("{}", y.len());
}
