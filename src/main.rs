use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::env;

/*
https://stackoverflow.com/questions/28169745/

Strings in Rust are unicode (UTF-8), and unicode codepoints are a superset of iso-8859-1
characters. This specific conversion is actually trivial.
*/
fn latin1_to_string(s: &[u8]) -> String {
    s.iter().map(|&c| c as char).collect()
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

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut buf = vec![];
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
        println!("{} = {:x}", latin1_to_string(&buf[0..len]), hash(&buf[0..len]));
        buf = vec![];
    }
    Ok(())
}
