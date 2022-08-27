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
    if a + b > c * 2{
        println!("{}", a * x + b * y );
    } else{
        println!("{}", c * (x + y) );
    }
}
