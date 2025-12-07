use std::env;
use std::fs;
use anyhow::{Result, bail, Context, anyhow};
use matrix::Matrix;

fn read_input(filename: &str) -> Result<Matrix<char>> {
    let content = fs::read_to_string(filename)?;
    let mut lines = content.lines();
    let mut mat: Matrix<char> = lines.map(|l| l.chars().collect()).collect();
    println!("{:?}", mat);

    let mut cur_op = ' ';
    for col_idx in 0..mat.num_cols() {
        let col = mat.col(col_idx)?;
        if col[col.len() - 1] != ' ' {
            cur_op = col[col.len() - 1];
        } else {
            mat.set(col_idx, col.len() -1, cur_op);
        }

    }

    Ok(mat)
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        bail!("Wrong number of args");
    }

    let mat = read_input(&args[1])?;

    let mut total = 0;
    let mut cur_op = ' ';
    let mut sum_acc = 0;
    let mut mul_acc = 1;
    for colidx in (0..mat.num_cols()).rev() {
        let col = mat.col(colidx)?;
        let col_str = col[..col.len() - 1].iter().collect::<String>();
        if col_str.chars().all(|c| c == ' ') {
            match cur_op {
                '+' => total += sum_acc,
                '*' => total += mul_acc,
                _ => bail!("Unrecognized operator"),
            }
            sum_acc = 0;
            mul_acc = 1;
            continue;
        }
        cur_op = col[col.len() - 1];
        match cur_op {
            '*' => mul_acc *= col_str.trim().parse::<u128>()?,
            '+' => sum_acc += col_str.trim().parse::<u128>()?,
            _ => bail!("Unrecognized operator"),
        }
        println!("{} {} {} {}", cur_op, col_str, sum_acc, mul_acc);
    }
    match cur_op {
        '+' => total += sum_acc,
        '*' => total += mul_acc,
        _ => bail!("Unrecognized operator"),
    }
    
    println!("{}", total);

    Ok(())
}
