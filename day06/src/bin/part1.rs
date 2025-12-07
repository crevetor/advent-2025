use std::env;
use std::fs;
use anyhow::{Result, bail, Context};
use matrix::Matrix;

fn read_input(filename: &str) -> Result<(Matrix<usize>, Vec<char>)> {
    let content = fs::read_to_string(filename)?;
    let mut lines = content.lines();
    let mut operators = Vec::new();
    let mut numbers = Vec::new();

    for line in lines {
        let elts = line.split_whitespace().collect::<Vec<_>>();
        if elts.len() > 0 && elts[0].contains(&['*', '/', '+', '-']) {
            operators = elts.iter().map(|el| el.chars()).flatten().collect::<Vec<_>>();
        } else {
            numbers.push(elts.iter().map(|el| el.parse::<usize>().unwrap()).collect::<Vec<_>>());
        }
    }


    Ok((Matrix::from_iter(numbers), operators))
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        bail!("Wrong number of args");
    }

    let (numbers, operations) = read_input(&args[1])?;

    println!("Numbers: {:?}", numbers);
    println!("Operations: {:?}", operations);

    let mut total = 0;
    for (op, nums) in operations.iter().zip(numbers.cols()) {
        println!("{:?} {:?}", op, nums);
        total += match op {
            '*' => nums.iter().product::<usize>(),
            '+' => nums.iter().sum::<usize>(),
            _ => 0,
        }
    }

    println!("Total: {}", total);

    Ok(())
}
