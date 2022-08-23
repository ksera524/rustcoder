use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
    n:i32,
    y:i32,
    }
    

    let mut ans = 0;

    for i in 0..=n{
        for j in 0..=(n){
            ans =  1000 * i + 5000 * j + 10000 * (n - i - j);
            if ans == y &&  (n - i - j) >= 0{
                println!("{} {} {}",(n - j - i),j,i);
                return ;
            }
        }
    }

    println!("{} {} {}",-1,-1,-1);
}