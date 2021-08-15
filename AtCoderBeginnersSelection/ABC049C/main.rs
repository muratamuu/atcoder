fn main() {
    let s = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim().to_owned()
    };
    let mut st = 0;
    let slen = s.len();
    loop {
        if st >= slen {
            println!("YES");
            return;
        }
        if let Some(sz) = check(&s[st..slen]) {
            st += sz;
        } else {
            println!("NO");
            return;
        }
    }
}

fn check(s: &str) -> Option<usize> {
    if s.starts_with("eraser") {
        Some(6)
    } else if s.starts_with("erase") {
        Some(5)
    } else if s.starts_with("dreamer") {
        if s.len() == 7 {
            Some(7)
        } else {
            let ss = &s[7..s.len()];
            if ss.starts_with("d") || ss.starts_with("e") {
                Some(7)
            } else {
                Some(5)
            }
        }
    } else if s.starts_with("dream") {
        Some(5)
    } else {
        None
    }
}

