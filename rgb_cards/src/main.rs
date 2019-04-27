fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(& mut input).unwrap();
    let v: Vec<u32> = input.trim().split_whitespace()
            .map(|x| x.parse().unwrap()).collect();

    let rgb = v[0] * 100 + v[1] * 10 + v[2];

    let ans = if rgb % 4 == 0 { "YES" } else { "NO" };
    println!("{}", ans);
}
