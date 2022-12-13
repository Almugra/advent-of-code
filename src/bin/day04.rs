use std::{fs, io};

fn main() -> io::Result<()> {
    let file = fs::read_to_string("./inputs/i4.txt").unwrap();
    let start = std::time::Instant::now();
    let val = file.lines().map(|pair| {
        pair.split_terminator(&['-', ','][..])
            .map(|v| v.parse::<u32>().unwrap())
            .collect()
    });

    println!(
        "Part 1 = {}",
        val.clone()
            .map(|a: Vec<_>| (a[0] <= a[2] && a[1] >= a[3] || a[0] >= a[2] && a[1] <= a[3]) as u32)
            .sum::<u32>()
    );
    println!(
        "Part 2 = {}",
        val.map(|a| (a[0] <= a[3] && a[2] <= a[1]) as u32)
            .sum::<u32>()
    );
    println!("time: {:?}", start.elapsed());
    Ok(())
}
