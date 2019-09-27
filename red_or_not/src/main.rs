use std::io::*;
use std::str::FromStr;

#[test]
fn test() {
    assert_eq!(red_or_not(3200, &"pink".to_string()), "pink".to_string());
    assert_eq!(red_or_not(3199, &"pink".to_string()), "red".to_string());
    assert_eq!(red_or_not(4049, &"red".to_string()), "red".to_string());
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

fn red_or_not(a: u32, s: &String) -> String {
    if a < 3200 {
        return "red".to_string();
    }
    s.to_string()
}

fn main() {
    let a: u32 = read();
    let s: String = read();
    println!("{}", red_or_not(a, &s));

}

