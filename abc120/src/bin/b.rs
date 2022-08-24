use proconio::{fastout, input};
use proconio::marker::*;
#[fastout]
fn main() {
    input! {
        a:i32,
        b:i32,
        k:i32,
    }
    let mut cnt = 0;
    for i in (1..=std::cmp::min(a,b)).rev(){
        if a % i == 0 && b % i == 0{
            cnt += 1;
        }
        if cnt == k {
            println!("{}",i);
            return;
        }        
    }
}
