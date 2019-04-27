fn main() {
    let mut ab = String::new();
    std::io::stdin().read_line(& mut ab).unwrap();
    let ab: Vec<f32> = ab.trim().split_whitespace()
            .map(|x| x.parse().unwrap()).collect();
    
    let mean = (ab[0] + ab[1]) / 2.0;
    let ans = mean.ceil();
    println!("{}",ans);
}
