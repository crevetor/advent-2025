use std::env;
use std::fs;
use std::ops::RangeInclusive;
use anyhow::{Result, bail, Context};

fn read_input(filename: &str) -> Result<(Vec<RangeInclusive<usize>>, Vec<usize>)> {
    let content = fs::read_to_string(filename)?;
    let mut lines = content.lines();
    Ok((lines.by_ref().take_while(|l| !l.is_empty()).map(|l| {
        let (start, end) = l.split_once('-').unwrap();
        RangeInclusive::new(start.parse().unwrap(), end.parse().unwrap())
    }).collect(),
    lines.map(|l| l.parse().unwrap()).collect()))
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        bail!("Wrong number of args");
    }

    let (ranges, ids) = read_input(&args[1])?;
    let mut fresh = ids.iter().filter(|id| ranges.iter().any(|r| r.contains(id))).count();

    println!("{}", fresh);

    Ok(())
}
