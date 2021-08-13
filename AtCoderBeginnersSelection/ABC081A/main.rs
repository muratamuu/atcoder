fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let sum: u32 = s.trim().chars().map(|c| c.to_digit(10).unwrap()).sum();
    println!("{}", sum);
}
