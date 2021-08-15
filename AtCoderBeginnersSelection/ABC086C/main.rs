fn main() {
    let n: i64 = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim().parse().unwrap()
    };
    let positions: Vec<(i64, i64, i64)> = {
        let mut v = Vec::new();
        for _ in 0..n {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            let mut iter = s.trim().split_whitespace();
            v.push(
            (
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap(),
            ));
        }
        v
    };

    let mut step = 0;
    let mut x1 = 0;
    let mut y1 = 0;
    for (n, x2, y2) in positions {
        if check((x1, y1), (x2, y2), n - step) {
            x1 = x2;
            y1 = y2;
            step = n;
        } else {
            println!("No");
            return;
        }
    }
    println!("Yes");
    return;
}

fn check((x1, y1): (i64, i64), (x2, y2): (i64, i64), n: i64) -> bool {
    let diff = (x1 - x2).abs() + (y1 - y2).abs();
    let mut m = n;
    loop {
        if diff == m {
            return true;
        }
        m -= 2;
        if m < 0 {
            return false;
        }
    }
}
