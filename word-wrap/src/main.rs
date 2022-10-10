fn main() {
    let result = wrapper::Wrapper::wrap(String::from("Hello, world!"), 5);
    println!("{}", result);
}

pub mod wrapper {
    pub struct Wrapper {}

    impl Wrapper {
        pub fn wrap(string: String, col_num: usize) -> String {
            let chars = string.chars();
            if col_num == 0 || chars.count() <= col_num {
                string
            } else {
                let wrap_index = Self::wrap_index(&string[0..col_num], &col_num);
                let line = &string[0..wrap_index].trim();
                let unwrapped = &string[wrap_index..].trim();
                let remaining = Self::wrap(unwrapped.to_string(), col_num);

                [line, remaining.as_str()].join("\n")
            }
        }

        fn wrap_index(string: &str, col_num: &usize) -> usize {
            let mut spaces: Vec<_> = string.match_indices(" ").collect();
            if spaces.len() > 0 {
                spaces.pop().unwrap().0
            } else {
                *col_num
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::wrapper::Wrapper;

    #[test]
    fn word_that_fits() {
        let result = Wrapper::wrap(String::from("Hello"), 5);
        assert_eq!(result, "Hello");
    }

    #[test]
    fn sentence_that_fits() {
        let result = Wrapper::wrap(String::from("Hello, world!"), 13);
        assert_eq!(result, "Hello, world!");
    }

    #[test]
    fn a_sentence_that_does_not_fit() {
        let result = Wrapper::wrap(String::from("Hello, world!"), 7);
        assert_eq!(result, "Hello,\nworld!");
    }

    #[test]
    fn a_word_in_a_sentence_that_does_not_fit() {
        let result = Wrapper::wrap(String::from("Hello, world! Procrastinating"), 7);
        assert_eq!(result, "Hello,\nworld!\nProcras\ntinatin\ng");
    }

    #[test]
    fn double_spaces() {
        let result = Wrapper::wrap(String::from("Hello,  world!"), 7);
        assert_eq!(result, "Hello,\nworld!");
    }

    #[test]
    fn zero() {
        let result = Wrapper::wrap(String::from("Hello, world!"), 0);
        assert_eq!(result, "Hello, world!");
    }
}
