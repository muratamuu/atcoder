fn main() {
    let (n, y) = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        let mut iter = s.trim().split_whitespace();
        (
            iter.next().unwrap().parse::<i64>().unwrap(),
            iter.next().unwrap().parse::<i64>().unwrap(),
        )
    };

    for a in 0..=n {
        for b in 0..=n {
            let c = n - a - b;
            if c >= 0 {
                if a * 10000 + b * 5000 + c * 1000  == y {
                    println!("{} {} {}", a, b, c);
                    return;
                }
            }
        }
    }

    println!("-1 -1 -1");
}
