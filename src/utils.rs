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
        .unwrap_or(string.len());

    let extracted = &string[..extracted_end];
    let remainder = &string[extracted_end..];
    (remainder, extracted)
}

pub(crate) fn take_while1(
    accept: impl Fn(char) -> bool,
    string: &str,
    error_msg: String,
) -> Result<(&str, &str), String> {
    let (remainder, extracted) = take_while(accept, string);

    if extracted.is_empty() {
        Err(error_msg)
    } else {
        Ok((remainder, extracted))
    }
}
pub(crate) fn tag<'a>(starting_text: &str, string: &'a str) -> Result<&'a str, String> {
    if string.starts_with(starting_text) {
        Ok(&string[starting_text.len()..])
    } else {
        Err(format!("expected {}", starting_text))
    }
}

pub(crate) fn extract_digits(string: &str) -> Result<(&str, &str), String> {
    take_while1(
        |char| char.is_ascii_digit(),
        string,
        "expected digits".to_string(),
    )
}

pub(crate) fn extract_whitespace(string: &str) -> (&str, &str) {
    take_while(|char| char == ' ', string)
}

pub(crate) fn extract_whitespace1(string: &str) -> Result<(&str, &str), String> {
    take_while1(|char| char == ' ', string, "expected a space".to_string())
}
pub(crate) fn extract_identifier(string: &str) -> Result<(&str, &str), String> {
    let input_starts_with_alphabetic = string
        .chars()
        .next()
        .map(|char| char.is_ascii_alphabetic())
        .unwrap_or(false);
    if input_starts_with_alphabetic {
        Ok(take_while(|char| char.is_ascii_alphanumeric(), string))
    } else {
        Err("expected identifier".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_one_digit() {
        assert_eq!(extract_digits("1+2"), Ok(("+2", "1")));
    }
    #[test]
    fn extract_multiple_digits() {
        assert_eq!(extract_digits("10-20"), Ok(("-20", "10")));
    }
    #[test]
    fn do_not_extract_digits_when_input_is_invalid() {
        assert_eq!(extract_digits("abcd"), Err("expected digits".to_string()));
    }
    #[test]
    fn extract_digits_with_no_remainder() {
        assert_eq!(extract_digits("100"), Ok(("", "100")));
    }
    #[test]
    fn extract_spaces() {
        assert_eq!(extract_whitespace("    1"), ("1", "    "));
    }
    #[test]
    fn extract_alphbetic_identifier() {
        assert_eq!(extract_identifier("abcdEFG stop"), Ok((" stop", "abcdEFG")));
    }
    #[test]
    fn extract_alphanumaric_identifier() {
        assert_eq!(extract_identifier("foobar1()"), Ok(("()", "foobar1")));
    }
    #[test]
    fn cannot_extract_identifier_beginning_with_number() {
        assert_eq!(
            extract_identifier("123abc"),
            Err("expected identifier".to_string())
        );
    }
    #[test]
    fn tag_word() {
        assert_eq!(tag("let", "let a"), Ok(" a"));
    }
    #[test]
    fn do_not_extract_spaces1_when_input_does_not_start_with_them() {
        assert_eq!(
            extract_whitespace1("blah"),
            Err("expected a space".to_string())
        );
    }
}
