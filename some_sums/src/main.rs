fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let v: Vec<u32> = input.trim().split_whitespace()
        .map(|x| x.parse().unwrap()).collect();
    //println!("{}", v[0]+1);
    let mut ans = 0;
    for mut x in 1..v[0]+1 {
//            println!("{}", x);
        let mut sum = 0;
        let mut tmp = x;
        loop {
//            println!("tmp:{}", tmp);
            sum += tmp % 10; 
//            println!("sum:{}", sum);
            if (tmp / 10 == 0) {
                break;
            }
            tmp = tmp / 10;
        }
        if (sum >= v[1] && sum <= v[2]){
            ans += x;
//            println!("ans:{}", ans);
        }
    }
    println!("{}", ans);
}
