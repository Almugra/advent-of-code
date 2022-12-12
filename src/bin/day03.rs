use std::{fs, io};

fn main() -> io::Result<()> {
    let file = fs::read_to_string("./inputs/i3.txt").unwrap();
    let start = std::time::Instant::now();
    let mut res: usize = 0;
    for line in file.lines() {
        let bys = line.as_bytes();
        let (a, b) = bys.split_at(bys.len() / 2);
        let mut val = true;
        for x in a {
            if b.contains(x) && val {
                val = false;
                if x >= &97 {
                    res += (x - 96) as usize;
                } else {
                    res += (x - 38) as usize;
                }
            }
        }
    }
    println!("Part 1 = {}", res);
    println!("time: {:?}", start.elapsed());
    Ok(())
}

