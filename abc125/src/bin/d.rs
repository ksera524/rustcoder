use num::abs;
use proconio::{fastout, input};
use proconio::marker::*;
#[fastout]
fn main() {
    input! {
        n:usize,
        a:[i64;n],
    }
    let num = a.iter().filter(|&x| *x < 0).count();
    let max:i64 = a.iter().map(|&x|x.abs()).sum::<i64>();
    let min:i64 = a.iter().map(|&x|x.abs()).min().unwrap();


    if num % 2 == 0{
        println!("{}", max);
    }else {
        println!("{}",max - min * 2);
    }
    
}
