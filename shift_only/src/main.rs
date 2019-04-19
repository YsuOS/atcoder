fn main() {
    let mut n = String::new();
    let mut a = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    std::io::stdin().read_line(&mut a).unwrap();
    let mut v: Vec<i32> = a.trim().split_whitespace()
        .map(|x| x.parse().unwrap()).collect();

//    println!("{}{}", n, a);
    
    let mut ans = 0;
    loop {
        let mut flag = 0;
        for x in &mut v {
//            println!("before: {}", x);
            if ( *x % 2 != 0 ) {
                flag = 1;
                break;
            }
            *x = *x / 2;
//            println!("{}", x);
        }
        if ( flag == 1 ) {
            break;
        }
        ans = ans + 1;
    }
    println!("{}", ans);
}
