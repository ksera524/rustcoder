use proconio::{fastout, input};
use proconio::marker::*;
#[fastout]
fn main() {
    input! {
        n:i64,
    }
    let t:i64 = 998244353;


    println!("{}",n % t );

}
