use regex::{Captures, Regex};

fn main() {
    let str = include_str!("input1.txt");
    let result = calculate(str.to_string());
    println!("Puzzle one result is: {}", result)
}

fn calculate(input: String) -> u64 {
    return input
        .lines()
        .map(|l| {
            let regex = Regex::new("Game (\\d+):\\s(.*)").unwrap();
            let captures = regex.captures(l).unwrap();
            let game_num: u32 = captures[1].parse().unwrap();
            let game_power = get_game_power(&captures[2]);

            println!("{} {:?}, {:?}", game_num, &captures[2], game_power);
            game_power
        }).sum();
}

struct Mins {
    red: u64,
    green: u64,
    blue: u64,
}

impl Mins {
    pub fn power(&self) -> u64 {
        return self.red * self.green * self.blue
    }
}

fn get_game_power(input: &str) -> u64 {
    let regex = Regex::new(r"(\d+)\s(red|green|blue)").unwrap();
    let grabs:Vec<Captures> = regex.captures_iter(input).collect();
    let mut mins = Mins {
        red: 0,
        green: 0,
        blue: 0,
    };

    for grab in grabs {
        let boxes:u64 = grab[1].parse().unwrap();
        match &grab[2] {
            "red" => {
                if boxes > mins.red { mins.red = boxes }
            },
            "green" => {
                if boxes > mins.green { mins.green = boxes }

            },
            "blue" => {
                if boxes > mins.blue { mins.blue = boxes }
            }
            _ => { }
        };
    }
    return mins.power()
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
        assert_eq!(result, 2286);
    }
}