fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut v: Vec<u32> = input.trim().split_whitespace()
        .map(|x| x.parse().unwrap()).collect();
    let mut ans = 0;
    let mut i = 2;
    loop {
        if ( i == 0 ) {
            break;
        }
        if ( v[0] >= v[1]) {
            ans += v[0];
            v[0] -= 1;
        } else {
            ans += v[1];
            v[1] -= 1;
        }
        i -= 1;
    }
    println!("{}", ans);
}
