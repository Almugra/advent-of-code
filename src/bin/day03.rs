use std::fs;

pub fn parse(compartment: &str) -> u64 {
    compartment
        .bytes()
        .map(|item| match item {
            b'a'..=b'z' => 1 << (item - 96),
            b'A'..=b'Z' => 1 << (item - 38),
            _ => panic!("cant parse"),
        })
        .fold(0, |acc, item| acc | item)
}

fn main() {
    let file = fs::read_to_string("./inputs/i3.txt").unwrap();
    let start = std::time::Instant::now();
    let part_one = file.lines().fold(0, |acc, line| {
        let (a, b) = line.split_at(line.len() / 2);
        acc + (parse(a) & parse(b)).trailing_zeros()
    });
    println!("Part 1 = {}", part_one);

    let part_two: u32 = file
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|a| (parse(a[0]) & parse(a[1]) & parse(a[2])).trailing_zeros())
        .sum();
    println!("Part 2 = {}", part_two);

    println!("time: {:?}", start.elapsed());
}
