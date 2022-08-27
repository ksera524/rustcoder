use proconio::{fastout, input};
use proconio::marker::*;
#[fastout]
fn main() {
    input! {
        a:i32,
        b:i32,
        c:i32,
        x:i32,
        y:i32,
    }
    let mut ans = 1000000000;
    for i in 0..=210000{
        let mut  cnt = i * c ;
        cnt += std::cmp::max(0,x - i / 2 ) * a;
        cnt += std::cmp::max(0,y - i / 2 ) * b;

        ans = std::cmp::min(ans,cnt);
    }
    println!("{}",ans);
}
