fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    use std::collections::HashMap;

    let mut map: HashMap<Vec<char>, Vec<String>> = HashMap::new();
    for i in strs {
        let mut tmp: Vec<char> = i.chars().collect();
        tmp.sort();

        let entry = map.entry(tmp).or_insert(vec![]);
        entry.push(i);
        // match map.get_mut(&tmp) {
        //     Some(arr) => {
        //         arr.push(strs[i].clone());
        //     }
        //     None => {
        //         map.insert(tmp, vec![strs[i].clone()]);
        //     }
        // };
    }

    map.into_values().collect()
    // let mut res = Vec::new();
    // for (k, v) in map.iter() {
    //     res.push(v.clone());
    // }
    // res
}

#[cfg(test)]
mod group_anagrams_test {
    use super::*;

    #[test]
    fn group_anagrams_test_1() {
        let mut res = group_anagrams(vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ]);

        let mut expected = vec![
            vec!["nat".to_string(), "tan".to_string()],
            vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
            vec!["bat".to_string()],
        ];

        assert_two_vec(&mut res, &mut expected);
    }

    #[test]
    fn group_anagrams_test_2() {
        let mut res = group_anagrams(vec!["".to_string()]);

        let mut expected = vec![vec!["".to_string()]];

        assert_two_vec(&mut res, &mut expected);
    }

    #[test]
    fn group_anagrams_test_3() {
        let mut res = group_anagrams(vec!["a".to_string()]);

        let mut expected = vec![vec!["a".to_string()]];

        assert_two_vec(&mut res, &mut expected);
    }

    #[test]
    fn group_anagrams_test_4() {
        let mut res = group_anagrams(vec!["".to_string(), "".to_string()]);

        let mut expected = vec![vec!["".to_string(), "".to_string()]];

        assert_two_vec(&mut res, &mut expected);
    }

    fn assert_two_vec(vec1: &mut Vec<Vec<String>>, vec2: &mut Vec<Vec<String>>) {
        for i in vec1.iter_mut() {
            i.sort();
        }
        vec1.sort();

        for i in vec2.iter_mut() {
            i.sort();
        }
        vec2.sort();

        assert_eq!(vec1, vec2);
    }
}
