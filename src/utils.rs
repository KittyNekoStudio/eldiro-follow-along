pub(crate) fn take_while(accept: impl Fn(char) -> bool, string: &str) -> (&str, &str) {
    let extracted_end = string
        .char_indices()
        .find_map(
            |(index, char)| {
                if accept(char) {
                    None
                } else {
                    Some(index)
                }
            },
        )
        .unwrap_or_else(|| string.len());

    let extracted = &string[..extracted_end];
    let remainder = &string[extracted_end..];
    (remainder, extracted)
}
pub(crate) fn extract_digits(string: &str) -> (&str, &str) {
    take_while(|char| char.is_ascii_digit(), string)
}

pub(crate) fn extract_whitespace(string: &str) -> (&str, &str) {
    take_while(|char| char == ' ', string)
}
pub(crate) fn extract_ops(string: &str) -> (&str, &str) {
    match &string[0..1] {
        "+" | "-" | "*" | "/" => {}
        _ => panic!("bad operator"),
    }

    (&string[1..], &string[0..1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_one_digit() {
        assert_eq!(extract_digits("1+2"), ("+2", "1"));
    }
    #[test]
    fn extract_multiple_digits() {
        assert_eq!(extract_digits("10-20"), ("-20", "10"));
    }
    #[test]
    fn do_not_extract_anything_from_empty_input() {
        assert_eq!(extract_digits(""), ("", ""));
    }
    #[test]
    fn extract_digits_with_no_remainder() {
        assert_eq!(extract_digits("100"), ("", "100"));
    }
    #[test]
    fn extract_plus() {
        assert_eq!(extract_ops("+2"), ("2", "+"));
    }
    #[test]
    fn extract_minus() {
        assert_eq!(extract_ops("-10"), ("10", "-"));
    }
    #[test]
    fn extract_multiply() {
        assert_eq!(extract_ops("*3"), ("3", "*"));
    }
    #[test]
    fn extract_divide() {
        assert_eq!(extract_ops("/4"), ("4", "/"));
    }
    #[test]
    fn extract_spaces() {
        assert_eq!(extract_whitespace("    1"), ("1", "    "));
    }
}
