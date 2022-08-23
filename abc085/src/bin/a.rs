use proconio::{fastout, input};
use proconio::marker::*;
#[fastout]
fn main() {
    input! {
        s:String,
    }
    
    println!("{}", s.replace("2017","2018") );
}
