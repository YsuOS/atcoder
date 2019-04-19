fn main() {
    let mut a = String::new(); //500 yen
    std::io::stdin().read_line(&mut a).unwrap();
    let a = a.trim().parse::<u32>().unwrap();

    let mut b = String::new(); //100 yen
    std::io::stdin().read_line(&mut b).unwrap();
    let b = b.trim().parse::<u32>().unwrap();

    let mut c = String::new(); //50 yen
    std::io::stdin().read_line(&mut c).unwrap();
    let c = c.trim().parse::<u32>().unwrap();

    let mut x = String::new(); // x yen
    std::io::stdin().read_line(&mut x).unwrap();
    let x = x.trim().parse::<u32>().unwrap();

    let mut ans = 0;

    for i in 0..a+1 {
//        println!("{}", 500 * i);
        for j in 0..b+1 {
//            println!("{}", 500 * i + 100 * j);
            for k in 0..c+1 {
//                println!("{}", 500 * i + 100 * j + 50 * k);
                if (x == 500 * i + 100 * j + 50 * k) {
//                    println!("match");
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}
