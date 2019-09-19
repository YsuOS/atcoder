use std::io::*;
use std::str::FromStr;

#[test]
fn test() {
    assert_eq!(one_clue(3, 7), [5, 6, 7, 8, 9]);
    assert_eq!(one_clue(4, 0), [-3, -2, -1, 0, 1, 2, 3]);
    assert_eq!(one_clue(1, 100), [100]);
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

fn one_clue(k: i64, x: i64) -> Vec<i64> {
    let mut v = Vec::new();
    for i in x-(k-1)..x+k {
        v.push(i);
    }
    v
}
fn main() {
    let k: i64 = read();
    let x: i64 = read();

    for i in one_clue(k, x).iter() {
        print!("{} ", i);
    }
}

