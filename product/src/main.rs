fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let v: Vec<u32> = s.trim().split_whitespace().map(|x| x.parse().unwrap())
                        .collect();
    let a = v[0];
    let b = v[1];

    let ans = if ( (a * b) % 2 == 0 ) { "Even" } else { "Odd" };
    println!("{}", ans);
}

