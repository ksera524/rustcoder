use proconio::{fastout, input};
use proconio::marker::*;
#[fastout]
fn main() {
    input! {
        s:Chars,
    }
    if s.len() == 1{
        println!("{}",s[0]);
        return ;
    }
    if  s.len() % 2 == 1
    {println!("{}",s[s.len() /2 ])} 
    else {println!("{}",s[s.len() /2])}
    
    
}
