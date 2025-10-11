fn length_of_last_word(s: String) -> i32 {
    s.split_whitespace()
        .collect::<Vec<&str>>()
        .last()
        .unwrap()
        .len() as i32
}

#[cfg(test)]
mod length_of_last_word {
    use super::*;

    #[test]
    fn length_of_last_word_1() {
        assert_eq!(length_of_last_word("Hello World".to_string()), 5);
    }

    #[test]
    fn length_of_last_word_2() {
        assert_eq!(
            length_of_last_word("   fly me   to   the moon  ".to_string()),
            4
        );
    }

    #[test]
    fn length_of_last_word_3() {
        assert_eq!(length_of_last_word("luffy is still joyboy".to_string()), 6);
    }

    #[test]
    fn length_of_last_word_4() {
        assert_eq!(length_of_last_word("a ".to_string()), 1);
    }
}
