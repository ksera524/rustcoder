use proconio::{fastout, input};
use proconio::marker::*;
#[fastout]
fn main() {
    input! {
        s:String,
    }
    let mut z = s;
    let x = ["eraser","erase","dreamer","dream",];
    for i in &x{
        z = z.replace(i,"").to_string();
    }

    println!("{}", if z.is_empty() {"YES"}else{"NO"});
}
