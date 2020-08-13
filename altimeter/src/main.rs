mod motivating_example_inf;
use crate::motivating_example_inf::Memory;
use std::fs::File;
use std::path::Path;
use std::io::{Read,Write};
use std::time::{Duration, Instant};
use std::thread::sleep;
use std::string::String;

static mut LENGTH: usize = 0;
static mut VAL: Vec<i64> = Vec::new();

fn main() {
    let input = "altidute_data.txt";
    let path = Path::new(&input);
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(f) => f,
    };
    let mut prog = String::new();
    match file.read_to_string(&mut prog) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => (),
    }

    //let file_string = fs::read_to_string("altitude_data.txt").expect("File should be readable");
    let numbers: Vec<i64> = prog
        .split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();
    let l = numbers.len();
    unsafe {
        LENGTH = numbers.len();
        VAL = numbers;
    }

    let mut mem = Memory {
        altitude_0: 0,
        altitude_1: 0,
    };
    let before = Instant::now();
    motivating_example_inf::monitor(&mut mem);
    let after = Instant::now();
    println!("{:?}",after.duration_since(before));
    let msg = format!("End of Data, {} inputs recived",l);
    print!("{}",&msg);
    std::io::stdout().write(b"\n");
}
