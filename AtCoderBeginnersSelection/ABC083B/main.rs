fn main() {
    let (n, a, b) = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        let mut iter = s.trim().split_whitespace();
        (
            iter.next().unwrap().parse::<i32>().unwrap(),
            iter.next().unwrap().parse::<i32>().unwrap(),
            iter.next().unwrap().parse::<i32>().unwrap(),
        )
    };
    let sum: i32 = (1..=n).map(|x| (x, x.to_string().chars().map(|c| c as i32 - 48).sum()))
        .filter(|(_, sum)| a <= *sum && *sum <= b).map(|(x, _)| x).sum();
    println!("{}", sum);
}
