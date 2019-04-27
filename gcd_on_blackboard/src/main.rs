use std::io::*;
use std::str::FromStr;

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
fn gcd(mut m: u32, mut n: u32) -> u32 {
    let mut tmp = 0;
    if ( m < n ) {
        tmp = m;
        m = n;
        n = tmp;
    }
    while ( n != 0 ) {
        tmp = m % n;
        m = n;
        n = tmp;
    }
    m
}

fn main() {
    let n: usize = read();
    let mut a = Vec::new();
    for _i in 0..n {
        let ai: u32 = read();
        a.push(ai);
    }

    let mut gcd_max = 0;
    for i in 0..n {
        for j in i+1..n {
            if gcd_max < gcd(a[i], a[j]) { 
                gcd_max = gcd(a[i], a[j]);
            }
        }
    }
    println!("{}", gcd_max);
}

