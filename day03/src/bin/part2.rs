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
        let mut digits: [u32; 12] = [0; 12];
        let mut next_idx = 0;

        for digit_idx in 0..12 {
            let mut bank_iter = bank.iter();
            let offset = next_idx;
            for (idx, digit) in bank[next_idx..bank.len()-11+digit_idx].iter().enumerate() {
                if digit  > &digits[digit_idx] {
                    digits[digit_idx] = *digit;
                    next_idx = offset + idx + 1;
                }
            }
        }

        println!("{digits:?}");
        let mut power = String::new();
        for digit in digits {
            power.push_str(digit.to_string().as_str());
        }
        println!("{power}");
        powersum += power.parse::<u128>()?;
    }

    println!("Total power : {powersum}");

    Ok(())
}
