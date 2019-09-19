use std::io::*;
use std::str::FromStr;

#[test]
fn test() {
    assert_eq!(a_137(-13, 3), -10);
    assert_eq!(a_137(1, -33), 34);
    assert_eq!(a_137(13, 3), 39);
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

fn a_137(a: i32, b: i32) -> i32 {
    let mut ans;
    ans = a + b;
    if a - b > ans {
        ans = a - b;
    }
    if a * b > ans {
        ans = a * b;
    }
    ans
}
fn main() {
    let a: i32 = read();
    let b: i32 = read();

    println!("{}", a_137(a, b));

}

