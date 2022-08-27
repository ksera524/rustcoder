use proconio::{fastout, input};
use proconio::marker::*;
#[fastout]
fn main() {
    input! {
        n:i64,
    }

    let mut point = 0;
    let mut cnt = 0;
    let mut pt = 0;
    for _ in 0..n{
        input! {
            t:i64,
            x:i64,
            a:i64,
        }
        if x - point<= t -pt{
            cnt += a;
            point = x;
            pt = t;
        }
    }
    println!("{}", cnt );
}
