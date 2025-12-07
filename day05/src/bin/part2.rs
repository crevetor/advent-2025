use std::cmp::{max, min};
use std::env;
use std::fs;
use std::ops::RangeInclusive;
use anyhow::{Result, bail, Context};
use std::collections::HashSet;

trait Overlap<Other=Self> {
    fn overlap(&self, other: &Other) -> bool;
}

impl<T: PartialOrd> Overlap<RangeInclusive<T>> for RangeInclusive<T> {
    fn overlap(&self, other: &RangeInclusive<T>) -> bool {
        (self.start() <= other.end() && self.end() >= other.start())
    }
}

trait Merge<Other=Self> {
    fn merge(&self, other: &Other) -> Self;
}

impl<T: Ord + Copy> Merge<RangeInclusive<T>> for RangeInclusive<T> {
    fn merge(&self, other: &RangeInclusive<T>) -> Self {
        RangeInclusive::new(min(*self.start(), *other.start()), max(*self.end(), *other.end()))
    }
}


fn read_input(filename: &str) -> Result<(Vec<RangeInclusive<usize>>, Vec<usize>)> {
    let content = fs::read_to_string(filename)?;
    let mut lines = content.lines();
    Ok((lines.by_ref().take_while(|l| !l.is_empty()).map(|l| {
        let (start, end) = l.split_once('-').unwrap();
        RangeInclusive::new(start.parse().unwrap(), end.parse().unwrap())
    }).collect(),
        lines.map(|l| l.parse().unwrap()).collect()))
}

fn merge_ranges(ranges:&mut Vec<RangeInclusive<usize>>) {
    let mut range = ranges.pop().unwrap();

    while ranges.iter().any(|r| r.overlap(&range)) {
        for i in 0..ranges.len() {
            if range.overlap(&ranges[i]) {
                range = range.merge(&ranges.swap_remove(i));
                break;
            }
        }
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        bail!("Wrong number of args");
    }

    let (ranges, ids) = read_input(&args[1])?;

    println!("{all_ids:?}");
    println!("{}", all_ids.len());

    Ok(())
}
