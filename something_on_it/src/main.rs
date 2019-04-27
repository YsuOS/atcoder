fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(& mut s).unwrap();
    
    let o = s.chars().filter(|&c| c == 'o' ).count();
    let ans = 700 + o * 100;
    println!("{}", ans);
}
