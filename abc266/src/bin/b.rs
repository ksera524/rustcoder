use proconio::{fastout, input};
use proconio::marker::*;
#[fastout]
fn main() {
    input! {
        n:i64,
    }
    let t:i64 = 998244353;
    let mut ans = n % t;
    if ans < 0{
        ans = ans + t;
    }
    println!("{}",ans );
}
