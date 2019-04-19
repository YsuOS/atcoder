fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    let n: u32 = n.trim().parse().unwrap();
    let mut a = String::new();
    std::io::stdin().read_line(&mut a).unwrap();
    let mut v: Vec<u32> = a.trim().split_whitespace()
        .map(|x| x.parse().unwrap()).collect();
    v.sort_by(|a, b| b.cmp(a));

    let mut alice = 0;
    let mut bob = 0;
    let mut tmp = 0;
    for x in v {
        if (tmp % 2 == 0) {
            alice += x;
        }
        if (tmp % 2 == 1) {
            bob += x;
        }
        tmp += 1;
    }
    let ans = alice - bob;
    println!("{}", ans);
}
