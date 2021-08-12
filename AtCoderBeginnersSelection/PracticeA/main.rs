fn main() {
    let a: i32 = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim().parse().unwrap()
    };
    let (b, c): (i32, i32) = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        let mut iter = s.trim().split_whitespace();
        (
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
        )
    };
    let s: String = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim().to_string()
    };
    println!("{} {}", a+b+c, s);
}
