use proconio::{fastout, input};
use proconio::marker::*;
#[fastout]
fn main() {
    input! {
        s:Chars,
    }
    let mut ans = 0;
    let mut cnt = 0;
    for i in 0..s.len(){
        if s[i as usize] == 'A' || s[i as usize] == 'G'|| s[i as usize] == 'C' || s[i as usize] == 'T'{
            cnt += 1;
        }else{
            cnt = 0;
        }
        ans = std::cmp::max(ans,cnt)
    }

    println!("{}", ans );
}
