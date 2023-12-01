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
            let letters: Vec<u32> = l.digits();
            let x = letters.first().unwrap_or(&0);
            let y = letters.last().unwrap_or(&0);
            x * 10 + y
        }).sum()
}



#[cfg(test)]
mod tests {
    use crate::calculate;

    #[test]
    fn it_works() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        let result = calculate(input.to_owned());
        assert_eq!(result, 142);
    }
}