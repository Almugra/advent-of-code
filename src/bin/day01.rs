use itertools::Itertools;
use std::{fs, io::Result};

pub fn parse(input: &str) -> impl Iterator<Item = u32> + '_ {
    input
        .split("\n\n")
        .map(|money_bag| {
            money_bag
                .lines()
                .map(|v| v.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .sorted_by(|a, b| b.cmp(a))
}

fn main() -> Result<()> {
    let file = fs::read_to_string("./inputs/i.txt").unwrap();
    println!("Part 1 = {}", parse(&file).max().unwrap());
    println!("Part 2 = {}", parse(&file).take(3).sum::<u32>());
    Ok(())
}
