fn main() {
    let n: i64 = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim().parse().unwrap()
    };
    let q = if n <= 125 {
        4
    } else if n <= 211 {
        6
    } else {
        8
    };
    println!("{}", q);
}
