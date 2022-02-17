use std::{thread::sleep, time::Duration};

mod functions;

fn main() {
    loop {
        for _ in 0..50 {
            print!("\r{}\r", functions::gen_rand(4).unwrap());
            functions::flush();
            sleep(Duration::from_millis(20))
        }

        println!("\r{}", functions::gen_rand(4).unwrap());
    }
}
