use std::io::*;
use std::str::FromStr;

#[test]
fn test() {
    assert_eq!(power_socket(4, 10), 3);
    assert_eq!(power_socket(8, 9), 2);
    assert_eq!(power_socket(8, 8), 1);
    assert_eq!(power_socket(2, 1), 0);
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

fn power_socket(a: u8, b: u8) -> u8 {
    let mut ans = 0;
    let mut cnt = 1;
    while cnt < b {
        ans += 1;
        cnt += a-1;
    }
    ans
}

fn main() {
    let a: u8 = read();
    let b: u8 = read();
    println!("{}", power_socket(a, b));

}

