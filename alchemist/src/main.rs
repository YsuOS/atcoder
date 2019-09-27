use std::io::*;
use std::str::FromStr;

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

fn alchemist(n: usize, mut v: Vec<u32>) -> f32 {
    v.sort();
    let mut ans = v[0] as f32;
    for i in 1..n {
        ans = (ans + v[i] as f32) / 2.0
    }
    ans 
}

fn main() {
    let n: usize = read();
    let mut v = Vec::new();
    for _i in 0..n {
        v.push(read::<u32>());
    }
    println!("{}", alchemist(n, v));
//    println!("{:?}", v);

}

