use std::io;

fn main() -> io::Result<()> {
    let file = include_str!("../../inputs/i5.txt");
    let start = std::time::Instant::now();
    let mut vec: Vec<Vec<char>> = vec![vec![]; 9];
    let instructions: Vec<&str> = file.split("\n\n").collect();
    instructions[0].lines().for_each(|line| {
        line.chars().enumerate().for_each(|(i, char)| {
            if (i + 3) % 4 == 0 && char.is_alphabetic() {
                vec[(i - 1) >> 2].insert(0, char);
            }
        })
    });
    instructions[1].lines().for_each(|line| {
        let commands: Vec<_> = line
            .split_whitespace()
            .filter_map(|char| char.parse::<usize>().ok())
            .collect();

        //(1..=moves[0]).for_each(|_| {
        //    let pop = vec[moves[1] - 1].pop().unwrap();
        //    vec[moves[2] - 1].push(pop);
        //});
        (0..commands[0]).for_each(|index| {
            let pop = vec[commands[1] - 1].pop().unwrap();
            let len = vec[commands[2] - 1].len();
            vec[commands[2] - 1].insert(len - index, pop);
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
