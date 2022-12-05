use std::io::Read;

use day1::p1;

pub fn main() {
    let input = {
        let mut s = String::new();
        std::io::stdin().read_to_string(&mut s).unwrap();
        s
    };

    let parsed = p1::parse(&input);
    let mut totals = p1::totals(&parsed);

    totals.sort();
    let top3sum : u32 = totals.iter().rev().take(3).sum::<u32>();
    println!("{top3sum}");
}