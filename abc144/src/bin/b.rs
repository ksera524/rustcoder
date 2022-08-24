use proconio::{fastout, input};
use proconio::marker::*;
#[fastout]
fn main() {
    input! {
        n:i32,
    }
    for i in 1..10{
        for j in 1..10{
            if i * j == n{
                println!("{}","Yes");
                return;
            }
        }
    }
    
    println!("{}", "No" );
}
