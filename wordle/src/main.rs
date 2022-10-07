fn main() {
    let wordle = wordle::Wordle::new(String::from("alert"), String::from("alarm"));
    println!("{}", wordle.check_guess());
}

pub mod wordle {
    pub struct Wordle {
        target: String,
        guess: String,
    }

    impl Wordle {
        pub fn new(target: String, guess: String) -> Self {
            Self { target, guess }
        }

        pub fn check_guess(&self) -> String {
            let mut result = String::new();

            for (i, char) in self.guess.chars().enumerate() {
                if char == self.target.chars().nth(i).unwrap() {
                    result.push('2');
                } else {
                    let guess_count = self.guess[0..=i].matches(char).count();
                    let target_count = self.target.matches(char).count();

                    if target_count >= guess_count {
                        result.push('1');
                    } else {
                        result.push('0');
                    }
                }
            }
            result
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::wordle::Wordle;

    #[test]
    fn no_matching_characters() {
        let target = String::from("ropes");
        let guess = String::from("child");
        let wordle = Wordle::new(target, guess);
        let result = wordle.check_guess();

        assert_eq!(result, "00000");
    }

    #[test]
    fn characters_matching_in_correct_positions() {
        let target = String::from("alert");
        let guess = String::from("alarm");
        let wordle = Wordle::new(target, guess);
        let result = wordle.check_guess();

        assert_eq!(result, "22020");
    }

    #[test]
    fn character_in_wrong_position() {
        let target = String::from("stair");
        let guess = String::from("chore");
        let wordle = Wordle::new(target, guess);
        let result = wordle.check_guess();

        assert_eq!(result, "00010");
    }

    #[test]
    fn mix_of_characters_matching_and_wrong_positions() {
        let target = String::from("hairy");
        let guess = String::from("charm");
        let wordle = Wordle::new(target, guess);
        let result = wordle.check_guess();

        assert_eq!(result, "01120");
    }

    #[test]
    fn multiple_characters_in_wrong_positions() {
        let target = String::from("reads");
        let guess = String::from("elect");
        let wordle = Wordle::new(target, guess);
        let result = wordle.check_guess();

        assert_eq!(result, "10000");
    }
}
