use proconio::{fastout, input};
use proconio::marker::*;
#[fastout]
fn main() {
    input! {
        n:i32,
        a:i32,
        b:i32,
    }
    let mut ans = 0; 
    for i in 1..=n{
        let mut x = i;
        let mut cnt = 0;
        while x > 0{
            cnt += x % 10;
            x /= 10;
        }
        if cnt >= a && b >= cnt{
            ans += i;
        }
    }
    
    println!("{}", ans );
}
