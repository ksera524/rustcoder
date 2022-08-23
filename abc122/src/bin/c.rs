use proconio::{fastout, input};
use proconio::marker::*;
#[fastout]
fn main() {
    input! {
        n:usize,
        q:usize,
        s:Chars,
    }
    let mut v = vec![0;n];
    for i in 1..n{
        if s[i -1] == 'A' && s[i] == 'C'{
            v[i] = v[i-1] + 1;
        }else {
            v[i] = v[i-1];
        }
    }
    for _ in 0..q{
        input! {
            l:usize,
            r:usize,
        }
        println!("{}",v[r-1] - v[l-1]);
    }
}
