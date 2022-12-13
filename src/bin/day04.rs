use std::{fs, io};

fn main() -> io::Result<()> {
    let file = fs::read_to_string("./inputs/i4.txt").unwrap();
    let start = std::time::Instant::now();
    let val: u32 = file
        .lines()
        .map(|pair| {
            let a: Vec<_> = pair
                .split_terminator(&['-', ','][..])
                .map(|v| v.parse::<u32>().unwrap())
                .collect();
            if a[0] <= a[2] && a[1] >= a[3] || a[0] >= a[2] && a[1] <= a[3] {
                return 1;
            }
            0
        })
        .sum();
    println!("Part 1 = {:?}", val);
    println!("time: {:?}", start.elapsed());
    Ok(())
}
