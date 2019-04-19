fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().unwrap();
    let mut v = Vec::new();
    let mut tmp = n;
    while (tmp != 0) {
        let mut i = String::new();
        std::io::stdin().read_line(&mut i).unwrap();
        let mut d: u32 = i.trim().parse().unwrap();
        v.push(d);
        tmp -= 1;
    }
    v.sort();

    let mut ans = 0;
    let mut x: usize = 0;
    loop {
        if (x == n){
            break;
        }
        if (x == 0){
            ans += 1;
        }else if (v[x-1] < v[x]){
            ans += 1;
        }
        x += 1;
    }
    println!("{}", ans);
}
