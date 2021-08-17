fn main() {
    let (s, t) = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        let mut iter = s.trim().split_whitespace();
        (
            iter.next().unwrap().parse::<i64>().unwrap(),
            iter.next().unwrap().parse::<i64>().unwrap(),
        )
    };
    let mut count = 0;
    for i in 0..=s {
        for j in 0..=s {
            for k in 0..=s {
                if i + j + k <= s && i * j * k <= t {
                    count += 1;
                }
            }
        }
    }
    println!("{}", count);
}
