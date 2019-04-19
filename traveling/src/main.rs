fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    let n: u32 = n.trim().parse().unwrap();
    let mut i = 0;
    let mut txy = Vec::new();
    while ( i < n ) {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let tmp: Vec<u32> = input.trim().split_whitespace()
            .map(|x| x.parse().unwrap()).collect();
        txy.push(tmp);
        i += 1;
    }
    let mut flag = 0;
    let mut i = 0;
    while ( i < n ) {
        if ( txy[i as usize].len() != 3 ) {
            break;
        }
        if ( txy[i as usize][0] < txy[i as usize][1] + txy[i as usize][2] || txy [i as usize][1] + txy[i as usize][2] == 0 ) {
            break;
        }
        let j = txy[i as usize][0];
        if ( j == 0 ) {
            break;
        }
        if ( j % 2 == 1 ) {
            if ( txy[i as usize][1] + txy[i as usize][2] % 2 == 1 ) {
                flag = 1;
            }
        } else if ( j % 2 == 0 ){
            if ( txy[i as usize][1] + txy[i as usize][2] % 2 == 0 ) {
                flag = 1;
            }
        }
        i += 1;
    }
    if ( flag == 1 ) {
        println!("Yes");
    } else {
        println!("No");
    }
}
