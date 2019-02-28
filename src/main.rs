use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::env;
use std::time::Instant;

/*
https://stackoverflow.com/questions/28169745/

Strings in Rust are unicode (UTF-8), and unicode codepoints are a superset of iso-8859-1
characters. This specific conversion is actually trivial.
*/
fn latin1_to_string(s: &[u8]) -> String {
    s.iter().map(|&c| c as char).collect()
}
fn string_to_latin1(s: &String) -> Vec<u8> {
//fn string_to_latin1(s: String) -> &'static [u8] {
    s.chars().map(|c| c as u8).collect()
}


#[inline(never)]
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

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut buf = vec![];
    let now = Instant::now();
    let mut search_hash = 0;
    let mut print_all = true;
    let mut search_len = 0;

    if args.len() > 2 {
        print_all = false;
        //let search_string = args[2];
        search_hash = hash(string_to_latin1(&args[2]).as_slice().clone());
        search_len = args[2].len();
    }

    let filename = if args.len() > 1 { args[1].clone() } else { "lemmad.txt".to_string() };
    //let file = File::open("lemmad.txt")?;
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);

    //for len in BufReader::new(file).read_until(0x0a, &mut buf) {
    while let Ok(mut len) = reader.read_until(0x0a as u8, &mut buf) {
        if len < 2 {
            break;
        }
        len -= 2;
        // Example file has \r\n line endings. If we find otherwise, fix it up.
        if buf[len] != 0x0d {
            len += 1;
        }

        let hash = hash(&buf[0..len]);
        if print_all {
            println!("{:16x} {}", hash, latin1_to_string(&buf[0..len]));
        }
        else if hash == search_hash {
            if len != search_len {
                // println!("LENGTH exclude: {}", latin1_to_string(&buf[0..len]));
            }
            // TODO do slow comparison
            else {
                println!("{}", latin1_to_string(&buf[0..len]));
            }
        }
        buf = vec![];
    }
    println!("Time: {}", now.elapsed().as_micros());
    Ok(())
}
