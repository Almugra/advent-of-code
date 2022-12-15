use std::io;

fn main() -> io::Result<()> {
    let file = include_str!("../../inputs/i4.txt");
    let start = std::time::Instant::now();
    let pair_range = file.lines().map(|pair| {
        pair.split_terminator(&['-', ','][..])
            .map(|v| v.parse::<u32>().unwrap())
            .collect()
    });

    let res: (u32, u32) = pair_range.fold((0, 0), |(acc1, acc2), a: Vec<u32>| {
        let x = (a[0] <= a[2] && a[1] >= a[3] || a[0] >= a[2] && a[1] <= a[3]) as u32;
        let y = (a[0] <= a[3] && a[2] <= a[1]) as u32;
        (acc1 + x, acc2 + y)
    });

    println!("Part 1 = {}", res.0);
    println!("Part 2 = {}", res.1);

    println!("time: {:?}", start.elapsed());
    Ok(())
}
