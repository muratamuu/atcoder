use std::collections::HashSet;

fn main() {
    let n: i32 = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim().parse().unwrap()
    };
    let ds: HashSet<i32> = {
        let mut v = HashSet::new();
        for _ in 0..n {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            v.insert(s.trim().parse().unwrap());
        }
        v
    };

    println!("{}", ds.len());
}
