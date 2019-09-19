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
    let n: u32 = read();
    let k: usize = read();
    let s: &str = read::<String>();

    let mut ans = s[k];
    if ans.is_uppercase() { 
        ans = ans.to_lowercase();
    } else { 
        ans = ans.to_uppercase();
    }
    println!("{}", ans);
}

