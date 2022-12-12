use itertools::Itertools;

pub fn parse(input: &str) -> impl Iterator<Item = u32> + '_ {
    input
        .split("\n\n")
        .map(|elf_bag| {
            elf_bag
                .lines()
                .map(|v| v.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .sorted_by(|a, b| b.cmp(a))
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn it_works() {
        let file = fs::read_to_string("./src/bin/i.txt").unwrap();
        let res = parse(&file).max().unwrap();
        assert_eq!(res, 69912);
    }

    #[test]
    fn part_two_works() {
        let file = fs::read_to_string("./src/bin/i.txt").unwrap();
        let res = parse(&file).take(3).sum::<u32>();
        assert_eq!(res, 208180);
    }
}
