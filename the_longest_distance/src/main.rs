fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(& mut n).unwrap();
    let n: u32 = n.trim().parse().unwrap();
    let mut v = Vec::new();
    for _i in 0..n {
        let mut xy = String::new();
        std::io::stdin().read_line(& mut xy).unwrap();
        let tmp: Vec<f64> = xy.trim().split_whitespace()
                    .map(|x| x.parse().unwrap()).collect();
        v.push(tmp);
    }
    let mut tmp: Vec<f64> = Vec::new();
    for i in 0..n {
        for j in i+1..n {
            tmp.push(((v[i as usize][0] - v[j as usize][0]).powf(2f64) 
                      + (v[i as usize][1] - v[j as usize][1]).powf(2f64)).sqrt());
        }
    }
    let ans = tmp.iter().fold(0.0/0.0, |m, v| v.max(m));
    println!("{}", ans);
//    println!("{:?}", tmp);
//    println!("{}", (4.0 as f64).sqrt());    
}
