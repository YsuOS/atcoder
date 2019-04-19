fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let s: &str = &input.trim();
    
    let mut t = String::new();

    let mut tmp = String::new();
    let erase = "erase".chars().rev().collect::<String>();
    let eraser = "eraser".chars().rev().collect::<String>();
    let dream = "dream".chars().rev().collect::<String>();
    let dreamer = "dreamer".chars().rev().collect::<String>();

    for word in s.chars().rev() {
        tmp.push(word);
        if ( tmp == erase ) {
            t.push_str(erase.trim());
            tmp.clear();
        } else if ( tmp == dream ) {
            t.push_str(dream.trim());
            tmp.clear();
        } else if ( tmp == eraser ) {
            t.push_str(eraser.trim());
            tmp.clear();
        } else if ( tmp == dreamer ) {
            t.push_str(dreamer.trim());
            tmp.clear();
        } 
    }

    println!("input:{}", s.chars().rev().collect::<String>().trim());
    println!("t    :{}", t);
    if (s.chars().rev().collect::<String>().trim() == t.trim() ) {
        println!("YES");
        return;
    }
    println!("NO");
}
