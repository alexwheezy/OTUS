use rand::{thread_rng, Rng};
use std::{thread, time::Duration};

fn main() {
    loop {
        println!("{}", lipsum::lipsum_words_with_rng(thread_rng(), 20));
        thread::sleep(Duration::from_secs(2));
    }
}
