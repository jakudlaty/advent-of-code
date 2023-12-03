use regex::{Captures, Regex};
use advent_of_code_2023::utils::strings::StrExt;

fn main() {
    let str = include_str!("input1.txt");
    let result = calculate(str.to_string());
    println!("Puzzle one result is: {}", result)
}

fn calculate(input: String) -> u32 {
    return input
        .lines()
        .map(|l| {
            let regex = Regex::new("Game (\\d+):\\s(.*)").unwrap();
            let captures = regex.captures(l).unwrap();
            let game_num: u32 = captures[1].parse().unwrap();
            let game_possible = is_game_possible(&captures[2]);

            println!("{} {:?}, {:?}", game_num, &captures[2], game_possible);

            match game_possible {
                true => game_num,
                false => 0
            }
        }).sum();
}

fn is_game_possible(input: &str) -> bool {
    let regex = Regex::new(r"(\d+)\s(red|green|blue)").unwrap();
    let grabs:Vec<Captures> = regex.captures_iter(input).collect();
    for grab in grabs {
        let boxes:i32 = grab[1].parse().unwrap();
        let x = match &grab[2] {
            "red" => 12,
            "green" => 13,
            "blue" => 14,
            _ => 12
        };

        if boxes > x {
            return false
        }
    }

    return true
}


#[cfg(test)]
mod tests {
    use crate::calculate;

    #[test]
    fn it_works() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let result = calculate(input.to_owned());
        assert_eq!(result, 8);
    }
}