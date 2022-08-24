use proconio::{fastout, input};
use proconio::marker::*;
#[fastout]
fn main() {
    input! {
        n:usize,
        mut a:[usize;n],
    }

    let mut ans = 100000;
    for i in 0..n{
        let mut cnt = 0;
        while a[i as usize] % 2 == 0{
            cnt += 1;
            a[i] = a[i] / 2;
        }
        ans = std::cmp::min(ans,cnt);
    }

    println!("{}",ans);
}