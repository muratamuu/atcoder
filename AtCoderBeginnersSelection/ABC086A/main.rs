fn main() {
    let (a, b): (i64, i64) = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        let mut iter = s.trim().split_whitespace();
        (
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
        )
    };
    let ans = if (a * b) % 2 == 0 { "Even" } else { "Odd" };
    println!("{}", ans);
}
