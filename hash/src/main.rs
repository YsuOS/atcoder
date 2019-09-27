use std::collections::HashMap;
type Table = HashMap<String, Vec<String>>;

fn main() {
    let mut table = Table::new();
    table.insert("Konnichiwa".to_string(),
            vec!["hoge".to_string(), "fuga".to_string()]);
    table.insert("Hello".to_string(),
            vec!["foo".to_string(), "bar".to_string()]);

    show(&table);
    sort_works(&mut table);
    assert_eq!(table["Hello"][0], "bar");
}

fn show(table: &Table) {
    for (artist, works) in table {
        println!("works by {}", artist);
        for work in works {
            println!("  {}", work);
        }
    }
}

fn sort_works(table: &mut Table) {
    for (_artist, works) in table {
        works.sort();
    }
}
