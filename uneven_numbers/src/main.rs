use std::io::*;
use std::str::FromStr;

fn uneven_numbers(n: u32) -> u32 {
    let mut cnt = 0;
    for mut i in 1..(n+1) {
        let mut digit = 0;
        while i != 0 {
            i = i / 10;
            digit = digit + 1;
        }
        if digit % 2 == 1 {
            cnt = cnt + 1;
        }
    }
    cnt
}

#[test]
fn test() {
    assert_eq!(uneven_numbers(11), 9);
    assert_eq!(uneven_numbers(136), 46);
//    assert_eq!(uneven_numbers(100000), 90909);
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
    let n: u32 = read();


    let ans = uneven_numbers(n);
    println!("{}", ans);
}

