use anyhow::{Context, Result};

use count::count_lines;
use std::env;
use std::fs::File;
use std::io::BufReader;

fn main() -> Result<()> {
    for path in env::args().skip(1) {
        let file = File::open(&path)?;
        let file = BufReader::new(file);
        let lines = count_lines(file).context(&path)?;
        println!("{path}: {lines} lines");
    }
    Ok(())
}
