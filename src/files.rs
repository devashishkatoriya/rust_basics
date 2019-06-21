
// Currently not working - under development

use std::fs::File;
use std::io::{BufRead, BufReader, Error, Write};

pub fn run() -> Result<(), Error> {
    let path = "./input/lines.txt";


    let mut output = File::create(path)?;
    write!(output, "Rust\n💖\nFun")?;

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("{}", line?);
    }


    let mut data = read_file(path);
    println!("Data: {}", data?);

    Ok(())
}

fn read_file(path: &str) -> Result<String, Error> {
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut temp: String;

    for line in buffered.lines() {
        println!("{}", line?);
        temp.push_str(&line?);
    }

    Ok("xx".to_string())
}