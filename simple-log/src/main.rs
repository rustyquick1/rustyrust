extern crate chrono;

use std::io::prelude::*;
use std::fs::File;
use std::io;
use chrono::*;

fn log_something(filename: &'static str, string: &'static [u8; 12]) -> io::Result<()> {
    let mut f = File::create(filename)?;
    f.write_all(string)?;
    Ok(())
}

fn log_time(filename: &'static str) -> io::Result<()> {
    let local: DateTime<Local> = Local::now();
    let formatted = local.format("%a, %b %d %Y %I:%M:%S %p\n").to_string();
    let bytes = formatted.as_bytes();
    let mut f = File::create(filename)?;
    f.write_all(bytes)?;
    Ok(())
}

fn main() {
    // let local: DateTime<Local> = Local::now();
    // println!("{}", local);
    match log_time("newlog.txt") {
        Ok(..) => println!("File created!"),
        Err(..) => println!("Error: could not create file.")
    }
}