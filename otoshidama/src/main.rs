fn main() {
    let mut ny = String::new();
    std::io::stdin().read_line(&mut ny).unwrap();
    let v: Vec<u32> = ny.trim().split_whitespace()
        .map(|x| x.parse().unwrap()).collect();

    for i in 0..v[0]+1 {
        for j in 0..v[0]+1-i {
            if (10000 * i + 5000 * j + 1000 * (v[0]-i-j) == v[1]) {
                println!("{} {} {}", i, j, v[0]-i-j);
                return;
            }
        }
    }
    println!("-1 -1 -1");
}
