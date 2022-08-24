use proconio::{fastout, input};
use proconio::marker::*;
#[fastout]
fn main() {
    input! {
        n:i32,
        s:Chars,
    }
    let mut ans = 0;
    for i in 2..n{
        if s[(i-2) as usize] == 'A'{
            if s[(i-1) as usize] == 'B'{
                if s[i as usize] == 'C'{
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans );
}
