use proconio::{fastout, input};
use proconio::marker::*;
#[fastout]
fn main() {
    input! {
        n:i32,
        y:i32,
    }
    for i in 0..=n{
        for j in 0..=n{
            let k = n - i - j;
            let ans = 10000 * i + 5000* j + 1000 * k;
            if k >= 0 && ans == y{
                println!("{} {} {}",i,j,k);
                return;
            }
        }
    }
    println!("{} {} {}",-1,-1,-1 );
}
