fn main() {

    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    let v: Vec<char> = s.chars().collect();
    let mut ans = 0;
    if v[0] == '1' { ans += 1; }
    if v[1] == '1' { ans += 1; }
    if v[2] == '1' { ans += 1; }
    println!("{}", ans);
}
