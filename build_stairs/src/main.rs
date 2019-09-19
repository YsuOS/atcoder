use std::io::*;
use std::str::FromStr;

#[test]
fn test() {
    assert_eq!(build_stairs(5, [1, 2, 1, 1, 3].to_vec()), "Yes");
    assert_eq!(build_stairs(4, [1, 3, 2, 1].to_vec()), "No");
    assert_eq!(build_stairs(5, [1, 2, 3, 4, 5].to_vec()), "Yes");
    assert_eq!(build_stairs(4, [5, 4, 3, 2].to_vec()), "No");
    assert_eq!(build_stairs(1, [1000000000].to_vec()), "Yes");
    assert_eq!(build_stairs(3, [1, 3, 5].to_vec()), "Yes");
}

fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
            .bytes()
            .map(|c| c.expect("failed to read char") as char)
            .skip_while(|c| c.is_whitespace())
            .take_while(|c| !c.is_whitespace())
            .collect();
    token.parse().ok().expect("failed to parse token")
}

fn build_stairs(n: usize, mut v: Vec<i32>) -> String {
    v.reverse();
    for i in 0..n {
        if (i + 1) >= n {
            break;
        }
        if v[i + 1] - v[i] == 1 {
            if v[i + 1] <= 0 {
                return "No".to_string();
            }
            v[i + 1] = v[i + 1] - 1;
        } else if v[i + 1] - v[i] > 1 {
            return "No".to_string();
        }
    }
    "Yes".to_string()
}
fn main() {
    let n: usize = read();
    let v: Vec<i32> = (0..n).map(|_| read()).collect();
    println!("{}", build_stairs(n, v));

}

