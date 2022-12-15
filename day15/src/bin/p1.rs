use std::io::Read;

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let solution = day15::p1::solve(&input, 2000000);
    println!("{solution}");
}
