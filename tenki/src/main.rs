use std::io::*;
use std::str::FromStr;

#[test]
fn test() {
    assert_eq!(tenki("CSS".to_string(),"CSR".to_string()), 2);
    assert_eq!(tenki("SSR".to_string(),"SSR".to_string()), 3);
    assert_eq!(tenki("RRR".to_string(),"SSS".to_string()), 0);

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

fn tenki(s: String, t: String) -> u8 {
    let mut ans = 0;
    let sv :Vec<char> = s.chars().collect();
    let tv :Vec<char> = t.chars().collect();
    for i in 0..3 {
        if sv[i] == tv[i] {
            ans += 1;
        }
    }
    ans
}

fn main() {
    let s: String = read();
    let t: String = read();
    println!("{}", tenki(s, t));

}

