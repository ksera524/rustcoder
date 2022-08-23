use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        s:[String;n],
    }
    let mut dict = std::collections::HashMap::<String,u64>::new();

    for i in s.iter(){
        let mut ss = i.chars().collect::<Vec<char>>();
        ss.sort();
        let st:String = ss.iter().collect();

        *dict.entry(st).or_default() += 1;
    }

    println!("{}",dict.values().map(|v| v * (v - 1)/2 ).sum::<u64>());
}
