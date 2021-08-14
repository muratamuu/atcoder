fn main() {
    let _n = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim().parse::<i32>().unwrap()
    };
    let mut xs: Vec<i32> = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim().split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect()
    };
    let mut a = 0;
    let mut b = 0;
    while xs.len() > 0 {
        // a
        let (i, v) = max_index(&xs);
        a += v;
        xs.remove(i);
        if xs.len() == 0 {
            break;
        }
        // b
        let (i, v) = max_index(&xs);
        b += v;
        xs.remove(i);
    }
    println!("{}", a - b);
}

fn max_index(xs: &Vec<i32>) -> (usize, i32) {
    let mut max_idx = 0;
    let mut max_val = 0;
    for (i, v) in xs.iter().enumerate() {
        if *v > max_val {
            max_idx = i;
            max_val = *v;
        }
    }
    (max_idx, max_val)
}
