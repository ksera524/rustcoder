use proconio::{fastout, input};
use proconio::marker::*;
#[fastout]
fn main() {
    input! {
        s:Chars,
    }
    

    println!("{}",s.iter().filter(|&x|*x=='1').count());
}
