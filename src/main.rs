use std::fs::File;
use std::io::Result;
use std::env;
use std::time::Instant;
use memcmp::Memcmp;
use memchr::memchr_iter;
use memmap::MmapOptions;

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
    // let mut chr: u8;
    let mut res: u64 = 0;
    for chr in s.iter() {
        match chr {
            64...127 => {
                res |= (1 as u64) << (*chr as i32 - 64);
            }
            _ => ()
        }
    }

    return res;
}

fn gramify(s: &[u8]) -> [u8; 256] {
    let mut ret : [u8; 256] = [0; 256];
    for chr in s.iter() {
        ret[*chr as usize] += 1;
    }
    return ret;
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let now = Instant::now();
    // let mut print_all = true;
    let mut search_string = "".to_string();

    if args.len() > 2 {
        search_string = args[2].clone();
        // print_all = false;
    }

    let foo = string_to_latin1(&search_string);
    let search_bytes = foo.as_slice();
    let search_hash = hash(search_bytes);
    let search_len = search_bytes.len();
    let search_gram = gramify(search_bytes);

    let filename = if args.len() > 1 { args[1].clone() } else { "lemmad.txt".to_string() };
    let file = File::open(filename)?;
    let data = unsafe { MmapOptions::new().map(&file)? };

    let mut startpos = 0;

    for chrpos in memchr_iter(b'\n', &*data) {
        let mut endpos = chrpos;
        // Example file has \r\n line endings. If we find otherwise, fix it up.
        if data[endpos - 1] == b'\r' {
            endpos -= 1;
        }
        /*
        if print_all {
            println!("{:16x} {}", hash(&buf[0..len]), latin1_to_string(&buf[0..len]));
            buf = vec![];
            continue
        }
        */
        let line = &data[startpos..endpos];
        if line.len() != search_len {
            // println!("LENGTH exclude: {}", latin1_to_string(line));
            startpos = chrpos + 1;
            continue;
        }
        let hash = hash(line);
        if hash != search_hash {
            // println!("HASH exclude: {}", latin1_to_string(line));
            startpos = chrpos + 1;
            continue
        }
        if !gramify(line).memcmp(&search_gram) {
            // println!("GRAM exclude: {}", latin1_to_string(line));
            startpos = chrpos + 1;
            continue
        }

        println!("{}", latin1_to_string(line));
        startpos = chrpos + 1;
    }
    println!("Time: {}", now.elapsed().as_micros());
    Ok(())
}
