use proconio::{fastout, input};
use proconio::marker::*;
#[fastout]
fn main() {
    input! {
        a:i32,
        b:i32,
        c:i32,
        x:i32,
    }
    let mut ans = 0;
    for i in 0..=a{
        for j in 0..=b{
            for k in 0..=c{
                let num = i * 500 + j * 100 + 50 * k;
                if x == num {ans += 1;}
            }
        }
    }
    println!("{}", ans );
}
