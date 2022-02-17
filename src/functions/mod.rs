use rand::Rng;
use std::io::{stdout, Write};

const LETTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn rand_in_range(range: usize) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..range)
}

pub fn gen_rand(range: usize) -> Option<String> {
    let mut letters = Vec::<String>::new();

    for _ in 0..range {
        letters.push(
            LETTERS
                .chars()
                .nth(rand_in_range(LETTERS.len()))?
                .to_string(),
        );
    }

    Some(letters.join(" "))
}

pub fn flush() {
    stdout().flush().expect("Failed to flush stdout");
}
