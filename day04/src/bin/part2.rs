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

    let mut contents = read_input(&args[1])?;
    let mut removed = 0;

    loop {
        let mut removable_coords = Vec::new();
        for (x, y) in contents.coord_iter() {
            if contents.get(x, y)? == '@' && contents.get_neighbors_diag(x, y).iter().filter(|elt| elt.1 == '@').count() < 4 {
                println!("{x} {y} is accessible");
                removable_coords.push((x, y));
                removed += 1;

            }
        }

        if removable_coords.len() == 0 {
            break;
        }
        for coord in removable_coords {
            contents.set(coord.0, coord.1, '.');
        }
    }

    println!("{}", removed);
    Ok(())
}
