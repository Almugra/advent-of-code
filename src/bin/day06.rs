use std::{collections::HashMap, io};

fn main() -> io::Result<()> {
    let file = include_str!("../../inputs/i6.txt");
    let start = std::time::Instant::now();
    let mut map: HashMap<char, usize> = HashMap::new();
    let input_bytes = file.as_bytes();
    let mut right = 0;
    let mut left = 0;
    while right - left < 14 {
        if let Some(val) = map.get_mut(&(input_bytes[right] as char)) {
            if *val >= left {
                left = *val + 1;
            }
        }
        map.insert(input_bytes[right] as char, right);
        right += 1;
    }
    // Part 1: 'right - left < 4' ; Part 2: 'right - left < 14'
    println!("{}", right);
    println!("Time = {:?}", start.elapsed());
    Ok(())
}
