use std::{collections::HashMap, io::stdin};

fn main() {
    loop {
        let mut str = String::new();
        stdin().read_line(&mut str).expect("can't read from line");

        if str.is_empty() {
            return;
        }

        let chars = get_chars(&str);
        println!("{}", str.trim());
        for (char, cnt) in chars {
            println!("{}: {} ", char, cnt);
        }
        println!();
    }
}

fn get_chars(str: &str) -> HashMap<char, u32> {
    let mut chars = HashMap::new();
    for c in str.trim().chars() {
        *chars.entry(c).or_insert(0) += 1;
    }
    chars
}
