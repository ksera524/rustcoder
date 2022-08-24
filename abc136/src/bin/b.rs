use proconio::{fastout, input};
use proconio::marker::*;
#[fastout]
fn main() {
    input! {
        n:i32,
    }
    let mut ans = 0;
    for mut i in 1..=n{
        let mut cnt = 0;
        while i > 0 {
            cnt += 1;
            i /= 10;
        }
        if cnt % 2 == 1{ ans += 1}
    }

    println!("{}",ans  );
}
