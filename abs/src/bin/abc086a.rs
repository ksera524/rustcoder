use proconio::{fastout, input};
use proconio::marker::*;
#[fastout]
fn main() {
    input! {
        a:i32,
        b:i32,
    }
    
    println!("{}",if a * b % 2 == 0 {"Even"}else{"Odd"});
}
