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
            t:i32,
            xi:i32,
            yi:i32,
        }
        p = t -p;
        x = (x -xi).abs();
        y = (y - yi).abs();

        if p < x + y{
            println!("{}","No");
            return;
        }
        if p % 2 != (x + y) % 2{
            println!("{}","No");
            return;
        }
        p = t;
        x = xi;
        y = yi;
    }

    println!("{}", "Yes" );
}
