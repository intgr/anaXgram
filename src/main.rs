use std::cmp::max;
use std::env;
use std::fs::File;
use std::io::Result;
use std::num::Wrapping;
use std::process::exit;
use std::sync::Arc;
use std::thread;
use std::time::Instant;

use memchr::memchr;
use memcmp::Memcmp;
use memmap::MmapOptions;
use num_cpus;

/*
https://stackoverflow.com/questions/28169745/

Strings in Rust are unicode (UTF-8), and unicode codepoints are a superset of iso-8859-1
characters. This specific conversion is actually trivial.
*/
fn latin1_to_string(s: &[u8]) -> String {
    s.iter().map(|&c| c as char).collect()
}
fn string_to_latin1(s: &String) -> Vec<u8> {
    s.chars().map(|c| c as u8).collect()
}

fn hash(s: &[u8]) -> u64 {
    let mut res: u64 = 0;
    for chr in s.iter() {
        // Okay, this can probably be cleaned up but who cares :)
        res |= (Wrapping(1 as u64) << (Wrapping(*chr as i64) - Wrapping(64)).0 as usize).0;
    }

    return res;
}

type Gram = [u8; 256];

fn gramify(s: &[u8]) -> Gram {
    let mut ret: Gram = [0; 256];
    for chr in s.iter() {
        ret[*chr as usize] += 1;
    }
    return ret;
}

struct Needle {
    len: usize,
    hash: u64,
    gram: Gram
}

impl Needle {
    fn new(s: &String) -> Needle {
        let tmp = string_to_latin1(s);
        let bytes = tmp.as_slice();
        Needle {
            len: bytes.len(),
            hash: hash(bytes),
            gram: gramify(bytes)
        }
    }

    fn test(&self, s: &[u8]) -> bool {
        if self.len != s.len() {
            // println!("LENGTH exclude: {}", latin1_to_string(line));
            return false;
        }
        // D'oh, this hash checking only gains 15-20 milliseconds :)
        if self.hash != hash(s) {
            // println!("HASH exclude: {}", latin1_to_string(line));
            return false;
        }
        if !self.gram.memcmp(&gramify(s)) {
            // println!("GRAM exclude: {}", latin1_to_string(line));
            return false;
        }
        return true;
    }
}

fn handle(ndl: &Needle, data: &[u8]) -> Vec<String> {
    let mut startpos = 0;
    let mut ret = Vec::new();

    while let Some(offset) = memchr(b'\n', &data[startpos..]) {
        let chrpos = startpos + offset;
        let mut endpos = chrpos - 1;

        // Example file has \r\n line endings. If we find otherwise, fix it up.
        if data[endpos] != b'\r' {
            endpos += 1;
        }

        let line = &data[startpos..endpos];
        startpos = chrpos + 1;

        // OK, process this line
//        if print_all {
//            println!("{:16x} {}", hash(line), latin1_to_string(line));
//        }
//        else
        if ndl.test(line) {
            ret.push(latin1_to_string(line));
        }
    }
    return ret;
}

fn main() -> Result<()> {
    // Time tracking must be the first executed line in code
    let start_time = Instant::now();
    let threads = num_cpus::get();
    let args: Vec<String> = env::args().collect();
//    let mut print_all = true;
    let mut search_string = &"".to_string();

    if args.len() > 2 {
        search_string = &args[2];
//        print_all = false;
    }

    let filename = if args.len() > 1 { args[1].clone() } else { "lemmad.txt".to_string() };
    let file = File::open(filename)?;
    // Shared data between threads
    let mapping = unsafe { MmapOptions::new().map(&file)? };
    let data = Arc::new(mapping);
    let ndl = Arc::new(Needle::new(&search_string));

    let mut children = vec![];

    let mut startpos = 0;
    for i in 0..threads-1 {
        // Find a linebreak
        let tmp_pos = max(startpos, (data.len() * (i+1))/threads);
        match memchr(b'\n', &data[tmp_pos..]) {
            Some(offset) => {
                let endpos = tmp_pos + offset + 1;
                let thread_data = data.clone();
                let thread_ndl = ndl.clone();
                children.push(thread::spawn( move || {
                    handle(&thread_ndl, &thread_data[startpos..endpos])
                }));
                startpos = endpos;
            }
            None => {},     // OK, file may not be long enough
        }
    }
    // Last thread gets everything else
    let thread_ndl = ndl.clone();
    let thread_data = data.clone();

    children.push(thread::spawn(move || {
        handle(&thread_ndl, &thread_data[startpos..])
    }));

    let mut result = Vec::new();
    for child in children {
        result.extend(child.join().unwrap());
    }

    // The stopper must stop just before writing the results to console.
    println!("{},{}", start_time.elapsed().as_micros(), result.join(","));
    exit(0);
}
