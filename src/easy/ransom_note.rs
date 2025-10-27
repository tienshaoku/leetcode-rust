fn ransom_note(ransom_note: String, magazine: String) -> bool {
    let mut counts = [0; 26];

    for i in magazine.chars() {
        counts[(i as u8 - b'a') as usize] += 1;
    }
    for i in ransom_note.chars() {
        let index = (i as u8 - b'a') as usize;
        if counts[index] == 0 {
            return false;
        }
        counts[index] -= 1;
    }
    true
}

#[cfg(test)]
mod ransom_note_test {
    use super::*;

    #[test]
    fn ransom_note_test_1() {
        assert_eq!(ransom_note(String::from("a"), String::from("b")), false);
    }

    #[test]
    fn ransom_note_test_2() {
        assert_eq!(ransom_note(String::from("aa"), String::from("ab")), false);
    }

    #[test]
    fn ransom_note_test_3() {
        assert_eq!(ransom_note(String::from("aa"), String::from("aab")), true);
    }

    #[test]
    fn ransom_note_test_4() {
        assert_eq!(ransom_note(String::from("ac"), String::from("bc")), false);
    }

    #[test]
    fn ransom_note_test_5() {
        assert_eq!(
            ransom_note(String::from("aaccb"), String::from("aabbcc")),
            true
        );
    }
}
