use proconio::{fastout, input};
use proconio::marker::*;
#[fastout]
fn main() {
    input! {
        a:i64,
        b:i64,
        c:i64,
        k:i64,
    }
    if k % 2 == 0{
        println!("{}", a-b );
    }else{
        println!("{}", b-a );
    }


}
