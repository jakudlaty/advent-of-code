use advent_of_code_2023::utils::strings::StrExt;

fn main() {
    let str = include_str!("input1.txt");
    let result = calculate(str.to_string());
    println!("Puzzle two result is: {}", result)
}

fn calculate(input: String) -> u32 {
    return input
        .lines()
        .map(|l| {
            let string = l.replace_word_digits();
            let letters: Vec<u32> = string.digits();
            let x = letters.first().unwrap_or(&0);
            let y = letters.last().unwrap_or(&0);
            let res = x * 10 + y;
            res

        }).sum()
}



#[cfg(test)]
mod tests {
    use crate::calculate;

    #[test]
    fn it_works() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        let result = calculate(input.to_owned());
        assert_eq!(result, 281);
    }
}