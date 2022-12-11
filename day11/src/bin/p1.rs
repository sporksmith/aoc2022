use std::io::Read;

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let solution = day11::p1::solve(&input);
    println!("{solution}");
}
