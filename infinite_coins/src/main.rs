fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(& mut n).unwrap();
    let mut a = String::new();
    std::io::stdin().read_line(& mut a).unwrap();
    let n: i32 = n.trim().parse().unwrap();
    let a: i32 = a.trim().parse().unwrap();

    let ans = if n % 500 - a <= 0 { "Yes" } else { "No" };
    println!("{}", ans);
}
