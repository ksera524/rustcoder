use proconio::{fastout, input};
use proconio::marker::*;
#[fastout]
fn main() {
    input! {
        a:i32,
        b:i32,
        c:i32,
    }
    
    if a + b + 1 >= c{
        println!("{}", b + c);
    }else{
        println!("{}", b*2 + a + 1);
    }

}
