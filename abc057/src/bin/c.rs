use proconio::{fastout, input};
use proconio::marker::*;
#[fastout]
fn main() {
    input! {
        n:i64,
    }
    let mut ans = 1000000000;
    for i in 1..=n{
        if i * i > n{
            break;
        }
        if n % i == 0{
            ans = std::cmp::min(ans,f(i,n/i));
        }
    }
    println!("{}", ans);
}

fn f(a:i64,b:i64) -> i64{
    let a_num = a.to_string().len();
    let b_num = b.to_string().len();
    return std::cmp::max(a_num,b_num) as i64
}
