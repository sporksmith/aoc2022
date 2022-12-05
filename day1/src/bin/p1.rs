use std::io::Read;

use day1::p1;

pub fn main() {
    let input = {
        let mut s = String::new();
        std::io::stdin().read_to_string(&mut s).unwrap();
        s
    };

    let parsed = p1::parse(&input);
    let totals = p1::totals(&parsed);
    let solution = totals.iter().max().unwrap();

    println!("{}", solution);
}