extern crate core;

use std::ops::RangeInclusive;
use std::env;
use std::fs;
use anyhow::{Result, bail, Context, anyhow};

fn read_input(filename: &str) -> Result<Vec<RangeInclusive<i128>>> {
    let content = fs::read_to_string(filename)?;
    let line = content.lines().next().ok_or(anyhow!("File doesn't contain any lines"))?.trim();
    let mut ranges = Vec::new();
    for range in line.split(',') {
        let (start, end) = range.split_once('-').context("bad range")?;
        ranges.push(RangeInclusive::new(start.parse::<i128>()?, end.parse::<i128>()?));
    }
    Ok(ranges)
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        bail!("Wrong number of args");
    }

    let contents = read_input(&args[1])?;
    let mut sum = 0;

    for range in contents {
        for i in range {
            let i_rep = i.to_string();
            if i_rep.len() == 1 {
                continue;
            }
            let min_len = if i_rep.len() % 2 == 0 { i_rep.len() / 2 } else { i_rep.len() / 3 };
            for l in 1..=min_len {
                let pat = &i_rep[..l];
                let repeats = i_rep.len()/l;
                if pat.repeat(repeats) == i_rep {
                    println!("{}", i_rep);
                    sum += i;
                    break;
                }
            }
        }
    }
    println!("{}", sum);



    Ok(())
}
