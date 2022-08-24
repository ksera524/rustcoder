use proconio::{fastout, input};
use proconio::marker::*;
use std::cmp::Reverse;
#[fastout]
fn main() {
    input! {
        n:i32,
        a:[i32;n],
    }
    let mut x = a;
    x.sort_by_key(|&x| Reverse(x));
    let mut alice = 0;
    let mut bob = 0;
    for i in 0..n{
        if i % 2 == 0{
            alice += &x[i as usize] ;
        }else{
            bob += &x[i as usize];
        }
    }
    println!("{}",alice - bob);
}
