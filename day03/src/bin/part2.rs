use std::env;
use std::fs;
use anyhow::{Result, bail, Context};

fn read_input(filename: &str) -> Result<Vec<String>> {
    let content = fs::read_to_string(filename)?;
    Ok(content.lines().map(|l| l.trim().to_string()).collect())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        bail!("Wrong number of args");
    }

    let contents = read_input(&args[1])?;

    Ok(())
}
