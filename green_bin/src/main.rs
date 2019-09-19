use std::io::*;
use std::str::FromStr;
use std::collections::HashMap;

#[test]
fn test() {
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

fn green_bin(n: usize, s: Vec<Vec<char>>) -> u64 {
    let mut ans = 0;
    let mut map = HashMap::new();
    for i in 0..n {
        *map.entry(&s[i]).or_insert(0) += 1;
    }
//    println!("{:?}", map);
    for (_, n) in map {
        ans += n * (n - 1) / 2;
    }
    ans
}

fn main() {
    let n: usize = read();
    let mut s = Vec::new();
    for _i in 0..n {
        let mut si: Vec<char> = read::<String>().chars().collect();
        si.sort();
        s.push(si);
    }
    println!("{}", green_bin(n, s));

}

