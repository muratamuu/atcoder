fn main() {
    let mut xs: Vec<i64> = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.clear();
        std::io::stdin().read_line(&mut s).unwrap();
        let iter = s.trim().split_whitespace();
        iter.map(|s| s.parse().unwrap()).collect()
    };
    let mut count = 0;
    while xs.iter().all(|&x| x % 2 == 0) {
        xs = xs.iter().map(|x| x / 2).collect();
        count += 1;
    }
    println!("{}", count);
}
