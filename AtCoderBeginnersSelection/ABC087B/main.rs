fn main() {
    let a: i64 = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim().parse().unwrap()
    };
    let b: i64 = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim().parse().unwrap()
    };
    let c: i64 = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim().parse().unwrap()
    };
    let x: i64 = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim().parse().unwrap()
    };
    let mut ans = 0;
    for i in 0..=a {
        let a_yen = i * 500;
        if a_yen == x {
            ans += 1;
            break;
        }
        for j in 0..=b {
            let b_yen = j * 100;
            if a_yen + b_yen == x {
                ans += 1;
                break;
            }
            for k in 0..=c {
                let c_yen = k * 50;
                if a_yen + b_yen + c_yen == x {
                    ans += 1;
                    break;
                }
            }
        }
    }
    println!("{}", ans);
}
