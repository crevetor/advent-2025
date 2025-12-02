use anyhow::{anyhow, bail, Context, Result};
use std::env;
use std::fs;

fn read_input(filename: &str) -> Result<Vec<(char, i128)>> {
    let content = fs::read_to_string(filename)?;
    Ok(content
        .lines()
        .map(|l| {
            (
                l.chars().nth(0).unwrap(),
                l.chars()
                    .skip(1)
                    .collect::<String>()
                    .parse::<i128>()
                    .unwrap(),
            )
        })
        .collect())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        bail!("Wrong number of args");
    }

    let contents = read_input(&args[1])?;
    let mut pwd = 0;

    let mut dial = 50;
    for (dir, steps) in contents.iter() {
        match dir {
            'L' => dial -= steps,
            'R' => dial += steps,
            _ => return Err(anyhow!("Unknow direction {dir}")),
        }
        dial = dial % 100;
        if dial < 0 {
            dial = 100 + (dial % 100)
        }
        println!("{dial:?}");
        if dial == 0 {
            pwd += 1;
        }
    }
    println!("{pwd}");

    Ok(())
}
