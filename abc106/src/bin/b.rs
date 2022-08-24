use proconio::{fastout, input};
use proconio::marker::*;
#[fastout]
fn main() {
    input! {
        n:i32,
    }
    let mut ans = 0;
    for i in 1..=n{
        let mut cnt = 0;
        if i % 2 == 0{continue}
        for j in 1..=i{
            if i % j == 0{cnt += 1}
        }
        if cnt == 8 {ans += 1}
    }

    println!("{}", ans);
}
