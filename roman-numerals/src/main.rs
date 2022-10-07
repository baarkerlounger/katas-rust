use std::convert::TryFrom;
use std::io;

fn main() {
    loop {
        println!("Enter an integer to convert to roman numerals");
        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Cannot read input");

        let parsed_input: u32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let output = match convert(parsed_input) {
            Ok(numeral) => numeral,
            Err(e) => e.to_string(),
        };

        println!("{:?}", output);
        break;
    }
}

fn convert(int: u32) -> Result<String, &'static str> {
    if int > 3000 {
        Err("Roman numerals only went up to 3000")
    } else {
        let numeral_mapping = [
            Vec::from(["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"]),
            Vec::from(["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"]),
            Vec::from(["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"]),
            Vec::from(["", "M", "MM", "MMM"]),
        ];

        let str = int.to_string();
        let digit_chars = str.chars();
        let num_of_digits = str.len();
        let mut chars = Vec::new();

        for (i, char) in digit_chars.enumerate() {
            let key = num_of_digits - i - 1;
            let mapping = &numeral_mapping[key];
            let idx = usize::try_from(char.to_digit(10).unwrap()).unwrap();
            chars.push(mapping[idx]);
        }

        let roman_numeral: String = chars.into_iter().collect();
        Ok(roman_numeral)
    }
}

#[cfg(test)]
mod tests {

    use crate::convert;

    #[test]
    fn it_converts_integers_to_roman_numerals() {
        let result = convert(1).unwrap();
        assert_eq!(result, "I");

        let result = convert(2).unwrap();
        assert_eq!(result, "II");

        let result = convert(5).unwrap();
        assert_eq!(result, "V");

        let result = convert(10).unwrap();
        assert_eq!(result, "X");

        let result = convert(50).unwrap();
        assert_eq!(result, "L");

        let result = convert(369).unwrap();
        assert_eq!(result, "CCCLXIX");

        let result = convert(2751).unwrap();
        assert_eq!(result, "MMDCCLI");

        let result = convert(3000).unwrap();
        assert_eq!(result, "MMM");
    }

    #[test]
    fn it_returns_an_error_for_integers_over_3000() {
        let result = convert(3001)
            .err()
            .expect("Roman numerals only went up to 3000")
            .to_string();
        assert_eq!(result, "Roman numerals only went up to 3000");
    }
}
