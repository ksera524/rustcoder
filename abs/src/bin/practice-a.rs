use proconio::{fastout, input};
use proconio::marker::*;
#[fastout]
fn main() {
    input! {
        a:i32,
        b:i32,
        c:i32,
        s:String,
    }
    
    println!("{} {}",(a+b+c),s);
}
