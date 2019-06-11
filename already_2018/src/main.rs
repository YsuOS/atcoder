fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(& mut s).unwrap();
    let s = &s;
    let ans = s.replace("2017", "2018");
    println!("{}", ans.trim());
}
