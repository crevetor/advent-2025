use std::env;
use std::fs;
use anyhow::{Result, bail, Context};
use matrix::Matrix;

fn read_input(filename: &str) -> Result<Matrix<char>> {
    let content = fs::read_to_string(filename)?;
    Ok(content.lines().map(|l| l.trim().chars().collect()).collect())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        bail!("Wrong number of args");
    }

    let contents = read_input(&args[1])?;
    let mut num_accessible = 0;

    for (x, y) in contents.coord_iter() {
        if contents.get(x, y)? == '@' && contents.get_neighbors_diag(x, y).iter().filter(|elt| elt.1 == '@').count() < 4 {
            println!("{x} {y} is accessible");
            num_accessible += 1;
        }
    }

    println!("{}", num_accessible);
    Ok(())
}
