use proconio::{fastout, input};
use proconio::marker::*;
#[fastout]
fn main() {
    input! {
        n:usize,
    }
    let mut p = 0;
    let mut x = 0;
    let mut y = 0;

    for _ in 0..n{
        input! {
            t:usize,
            xi:usize,
            yi:usize,
        }
        if t < xi + yi{
            println!("{}","No");
            return;
        }
        if t % 2 != (xi + yi) % 2{
            println!("{}","No");
            return;
        }
    }

    println!("{}", "Yes" );
}
