mod network_inf_ex;
use crate::network_inf_ex::Memory;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::string::String;
use std::time:: Instant;

static mut LENGTH: usize = 0;
static mut VAL1: Vec<i64> = Vec::new();
static mut VAL2: Vec<i64> = Vec::new();
static mut VAL3: Vec<bool> = Vec::new();
static mut VAL4: Vec<bool> = Vec::new();
static mut VAL5: Vec<bool> = Vec::new();
static mut VAL6: Vec<i64> = Vec::new();

fn main() {
    let input = "network_data.txt";
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

    let lines: Vec<String> = prog.split('\n').map(|l| format!("{}", l)).collect();
    let mut v1: Vec<i64> = Vec::new();
    let mut v2: Vec<i64> = Vec::new();
    let mut v3: Vec<bool> = Vec::new();
    let mut v4: Vec<bool> = Vec::new();
    let mut v5: Vec<bool> = Vec::new();
    let mut v6: Vec<i64> = Vec::new();
    for l in lines.into_iter() {
        if l.is_empty() {
            continue;
        } else {
            let entries: Vec<&str> = l
                .split(',')
                .filter(|v| !v.is_empty())
                .map(|v| v.trim())
                .collect();
            v1.push(entries[0].parse().unwrap());
            v2.push(entries[1].parse().unwrap());
            v6.push(entries[5].parse().unwrap());
            let x3: i64 = entries[2].parse().unwrap();
            let x4: i64 = entries[3].parse().unwrap();
            let x5: i64 = entries[4].parse().unwrap();
            v3.push(if x3 == 1 { true } else { false });
            v4.push(if x4 == 1 { true } else { false });
            v5.push(if x5 == 1 { true } else { false });
        }
    }
    let l = v1.len();
    unsafe {
        LENGTH = v1.len();
        VAL1 = v1;
        VAL2 = v2;
        VAL3 = v3;
        VAL4 = v4;
        VAL5 = v5;
        VAL6 = v6;
    }

    let mut mem = Memory {
        count_0: 0,
        rec_var_0: 0,
        workload_0: 0,
        opened_0: 0,
        closed_0: 0,
    };
    let before = Instant::now();
    network_inf_ex::monitor(&mut mem);
    let after = Instant::now();
    println!("{:?}",after.duration_since(before));
    let msg = format!("End of Data, {} inputs received", l);
    print!("{}", &msg);
    std::io::stdout().write(b"\n");
}
