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
    let n: usize = read();
    let mut v = Vec::new();
    for i in 0..n {
        let vi: i32 = read();
        v.push(vi);
    }
    let mut c = Vec::new();
    for i in 0..n {
        let ci: i32 = read();
        c.push(ci);
    }

    let mut ans = 0;
    for i in 0..n {
        if v[i] - c[i] > 0 {
            ans += v[i] - c[i];
        }
    }
    println!("{}", ans);
//    println!("{:?}", v);
//    println!("{:?}", c);
//    let ans = if ( (a * b) % 2 == 0 ) { "Even" } else { "Odd" };
}

