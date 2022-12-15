use std::io;

const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

fn main() -> io::Result<()> {
    let file = include_str!("../../inputs/i5.txt");
    let start = std::time::Instant::now();
    let mut vec: Vec<Vec<char>> = vec![vec![]; 9];
    let s: Vec<&str> = file.split("\n\n").collect();
    for line in s[0].lines() {
        for (i, char) in line.as_bytes().iter().enumerate() {
            if (i + 3) % 4 == 0 && ' ' != (*char as char) {
                vec[(i - 1) >> 2].insert(0, *char as char);
            }
        }
    }
    s[1].lines().for_each(|line| {
        let moves: Vec<usize> = line
            .split_whitespace()
            .filter_map(|char| char.parse::<usize>().ok())
            .collect();

        (1..=moves[0]).for_each(|_| {
            let pop = vec[moves[1] - 1].pop().unwrap();
            vec[moves[2] - 1].push(pop);
        });
    });
    for v in vec.iter() {
        if let Some(last) = v.last() {
            print!("{}", last)
        }
    }
    println!("\nTime = {:?}", start.elapsed());
    Ok(())
}
