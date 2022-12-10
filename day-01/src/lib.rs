pub fn process_part1(input: &str) -> String {
    let calories_by_elf = get_sorted_calories_by_elf(input);

    calories_by_elf.iter().max().unwrap().to_string()
}

pub fn process_part2(input: &str) -> String {
    let calories_by_elf = get_sorted_calories_by_elf(input);

    calories_by_elf.iter().take(3).sum::<u32>().to_string()
}

fn get_sorted_calories_by_elf(input: &str) -> Vec<u32> {
    let mut calories_by_elf = input
        .split("\n\n")
        .map(|elf_load| {
            elf_load
                .lines()
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();

    calories_by_elf.sort_by(|a, b| b.cmp(a));
    return calories_by_elf;
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "24000");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "45000");
    }
}
