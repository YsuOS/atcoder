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

fn resistors_in_parallel(n: usize, a: Vec<f32>) -> f32 {
    let mut calc: f32 = 0.0;    

    for i in 0..n {
        calc = calc + a[i].recip();
    }
    let ans: f32 = calc.recip();
    ans
}

fn main() {
    let n: usize = read();
    let mut a = Vec::new();
    for _i in 0..n {
        a.push(read::<f32>());
    }
    println!("{}", resistors_in_parallel(n, a));

}

