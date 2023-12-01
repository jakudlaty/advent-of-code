pub trait StrExt {
    fn digits(&self) -> Vec<u32>;
    fn replace_word_digits(&self) -> String;
}

const DIGITS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

impl StrExt for str {
    fn digits(&self) -> Vec<u32> {
        self.chars().filter_map(|c| c.to_digit(10)).collect()
    }

    fn replace_word_digits(&self) -> String {
        let mut output = String::from(self);

        let mut val = 0;
        for digit in DIGITS {
            val += 1;
            let indices: Vec<_> = self.match_indices(digit).collect();
            for (idx, _) in indices {
                output.replace_range(idx..idx + 1, val.to_string().as_str());
            }
        }

        output
    }
}


#[cfg(test)]
mod tests {
    use crate::utils::strings::StrExt;

    #[test]
    fn it_works() {
        let input = "eightwo";

        let result = input.replace_word_digits();
        assert_eq!(result, "8igh2wo");
    }
}