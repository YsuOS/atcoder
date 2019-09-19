use std::io::*;
use std::str::FromStr;

fn transfer(a: u8, b: u8, c: u8) -> u8 {
    if b + c < a {
        0
    } else {
        (b + c) - a
    }
}

#[test]
fn test() {
    assert_eq!(transfer(6, 4, 3), 1);
    assert_eq!(transfer(8, 3, 9), 4);
    assert_eq!(transfer(12, 3, 7), 0);
}

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
    let a: u8 = read();
    let b: u8 = read();
    let c: u8 = read();


    let ans = transfer(a, b, c);
    println!("{}", ans);
}

