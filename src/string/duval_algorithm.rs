pub fn duval_algorithm(s: &str) -> Vec<String> {
    factorize_duval(&s.chars().collect::<Vec<char>>())
}

fn factorize_duval(s: &[char]) -> Vec<String> {
    let mut start = 0;
    let mut factors: Vec<String> = Vec::new();

    while start < s.len() {
        let mut end = start + 1;
        let mut repeat = start;

        while end < s.len() && s[repeat] <= s[end] {
            println!("start: {} end: {} repeat:{}, compared character: {}-{}", start, end, repeat, s[repeat], s[end]);
            if s[repeat] < s[end] {
                repeat = start;
            } else {
                repeat += 1;
            }
            end += 1;
        }

        while start <= repeat {
            factors.push(s[start..start + end - repeat].iter().collect::<String>());
            start += end - repeat;
        }
    }

    factors
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! test_duval_algorithm {
        ($($name:ident: $inputs:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (text, expected) = $inputs;
                    assert_eq!(duval_algorithm(text), expected);
                }
            )*
        }
    }

    test_duval_algorithm! {
        repeating_with_suffix: ("abcdabcdababc", vec!["abcd".to_string(), "abcd".to_string(), "ababc".to_string()]),
        single_repeating_char: ("aaa", vec!["a".to_string(), "a".to_string(), "a".to_string()]),
        single: ("ababb", vec!["ababb".to_string()]),
        unicode: ("അഅഅ", vec!["അ".to_string(), "അ".to_string(), "അ".to_string()]),
        empty_string: ("", Vec::<String>::new()),
        single_char: ("x", vec!["x".to_string()]),
        palindrome: ("racecar", vec!["r".to_string(), "acecar".to_string()]),
        long_repeating: ("aaaaaa", vec!["a".to_string(); 6]),
        mixed_repeating: ("ababcbabc", vec!["ababcbabc".to_string()]),
        non_repeating_sorted: ("abcdefg", vec!["abcdefg".to_string()]),
        alternating_increasing: ("abababab", vec!["ab".to_string(); 4]),
        long_repeating_lyndon: ("abcabcabcabc", vec!["abc".to_string(); 4]),
        decreasing_order: (
            "zyxwvutsrqponm",
            vec![
                "z".to_string(),
                "y".to_string(),
                "x".to_string(),
                "w".to_string(),
                "v".to_string(),
                "u".to_string(),
                "t".to_string(),
                "s".to_string(),
                "r".to_string(),
                "q".to_string(),
                "p".to_string(),
                "o".to_string(),
                "n".to_string(),
                "m".to_string()
            ]
        ),
        alphanumeric_mixed: ("a1b2c3a1", vec!["a".to_string(), "1b2c3a".to_string(), "1".to_string()]),
        special_characters: ("a@b#c$d", vec!["a".to_string(), "@b".to_string(), "#c$d".to_string()]),
        unicode_complex: ("αβγδ", vec!["αβγδ".to_string()]),
        long_string_performance: (&"a".repeat(1_000_000), vec!["a".to_string(); 1_000_000]),
        palindrome_repeating_prefix: ("abccba", vec!["abccb".to_string(), "a".to_string()].into_iter().map(|s| s.to_owned()).collect::<Vec<_>>()),
        interrupted_lyndon: ("abcxabc", vec!["abcxabc"]),
    }
}
