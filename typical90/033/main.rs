fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut iter = s.split_whitespace();
    let h: u32 = iter.next().unwrap().parse().unwrap();
    let w: u32 = iter.next().unwrap().parse().unwrap();
    println!("{} {}", h, w);
}
