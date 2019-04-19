fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    let n: u8 = n.trim().parse().unwrap();
    let mut h = String::new();
    std::io::stdin().read_line(&mut h).unwrap();
    let hs: Vec<u8> = h.trim().split_whitespace()
            .map(|x| x.parse().unwrap()).collect();
    let mut ans = 1;
    let mut i = 1;

    while ( i < n ) {
        let mut j = 1;
        while ( j <= i && (i-j) >= 0 ) {
            if ( hs[(i-j) as usize] <= hs[i as usize]) {
                j += 1;
            } else {
                break;
            }
            if ( j > i ) {
                ans += 1;
                break;
            }
        }
        i += 1;
    }
    println!("{}", ans);
}
