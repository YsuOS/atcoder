use std::io::*;
use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
            .bytes()
            .map(|c| c.expect("failed to read char") as char)
            .skip_while(|c| c.is_whitespace())
            .take_while(|c| !c.is_whitespace())
            .collect();
    token.parse().ok().expect("failed to parse token")
}

fn main() {
    let p: u32 = read();
    let q: u32 = read();
    let r: u32 = read();

    //println!("{}, {}, {}", p, q, r);
    let mut ans = p + q;
    if (ans >= q + r) {
        ans = q + r;
    }
    if (ans >= r + p) {
        ans = r + p;
    }
    println!("{}", ans);
}

