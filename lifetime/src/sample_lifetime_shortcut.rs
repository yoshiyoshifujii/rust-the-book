fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

#[cfg(test)]
mod tests {
    use crate::sample_lifetime_shortcut::first_word;

    #[test]
    fn test_1() {
        let w = String::from("word");
        let result = first_word(w.as_str());
        assert_eq!("word", result);
    }
}