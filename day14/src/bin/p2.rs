use std::io::Read;

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let solution = day14::p2::solve(&input);
    println!("{solution}");
}
