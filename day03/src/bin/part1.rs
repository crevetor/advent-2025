use anyhow::{bail, Context, Result};
use std::env;
use std::fs;

fn read_input(filename: &str) -> Result<Vec<Vec<u32>>> {
    let content = fs::read_to_string(filename)?;
    Ok(content
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        bail!("Wrong number of args");
    }

    let contents = read_input(&args[1])?;
    let mut powersum = 0;

    for bank in contents {
        let mut bank_iter = bank.iter();
        let mut first = (0, bank_iter.next().context("No first number")?);
        for num in bank_iter.take(bank.len() - 2).enumerate() {
            if num.1 > first.1 {
                first = (num.0 + 1, num.1);
            }
        }
        let second = bank
            .iter()
            .skip(first.0 + 1)
            .max()
            .context("Couldn't find max for second digit")?;
        let power = format!("{}{}", first.1, second).parse::<u128>()?;
        println!("{power}");
        powersum += power;
    }

    println!("Total power : {powersum}");

    Ok(())
}
