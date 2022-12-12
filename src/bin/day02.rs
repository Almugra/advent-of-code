use itertools::Itertools;
use std::{fs, io::Result};

// Rock = 0 ; Paper = 1 ; Scissors = 2
// lost = 0 ; draw = 3 ; win = 6

pub fn res(input: &str) -> (u64, u64) {
    let mut res = 0;
    let mut res2 = 0;
    for li in input.lines() {
        let (abc, xyz) = li.split(' ').collect_tuple().unwrap();
        let [abc, xyz] = [abc.as_bytes()[0] - b'A', xyz.as_bytes()[0] - b'X'];
        let outcome = (4 + xyz - abc) % 3;
        let outcome2 = (2 + xyz + abc) % 3;
        res += (xyz + 1 + (3 * outcome)) as u64;
        res2 += (xyz * 3 + (1 + outcome2)) as u64;
    }
    (res, res2)
}

fn main() -> Result<()> {
    let file = fs::read_to_string("./inputs/i2.txt").unwrap();
    let start = std::time::Instant::now();
    let (one, two) = res(&file);
    println!("Part 1 = {}", one);
    println!("Part 2 = {}", two);
    println!("time: {:?}", start.elapsed());
    Ok(())
}
