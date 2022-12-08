use std::fs;

fn main() {
    let input = fs::read_to_string("src/input/1.txt").unwrap();

    let elves_calories_strs = input.split("\n").collect::<Vec<&str>>();

    let mut elves_calories_totals: Vec<u32> = vec![];

    let mut current = 0;
    for calories in elves_calories_strs {
        let result = calories.parse::<u32>();
        match result {
            Ok(calories) => current += calories,
            Err(_) => {
                elves_calories_totals.push(current);
                current = 0;
            }
        }
    }

    elves_calories_totals.sort_by(|a, b| b.cmp(a));

    let highest = elves_calories_totals.first().unwrap();

    let top_three_sum =
        elves_calories_totals[0] + elves_calories_totals[1] + elves_calories_totals[2];

    println!("highest: {:?}", highest);
    println!("top three sum: {:?}", top_three_sum);
}
