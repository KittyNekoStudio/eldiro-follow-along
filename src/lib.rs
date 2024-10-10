mod utils;

#[derive(Debug, PartialEq)]
pub struct Number(pub i32);

#[derive(Debug, PartialEq)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq)]
pub struct Expression {
    pub lhs: Number,
    pub rhs: Number,
    pub op: Operation,
}

impl Number {
    pub fn new(string: &str) -> (&str, Self) {
        let (string, number) = utils::extract_digits(string);
        (string, Self(number.parse().unwrap()))
    }
}

impl Operation {
    pub fn new(string: &str) -> (&str, Self) {
        let (string, op) = utils::extract_ops(string);

        let op = match op {
            "+" => Operation::Add,
            "-" => Operation::Sub,
            "*" => Operation::Mul,
            "/" => Operation::Div,
            _ => unreachable!(),
        };

        (string, op)
    }
}

impl Expression {
    pub fn new(string: &str) -> (&str, Self) {
        let (string, lhs) = Number::new(string);
        let (string, _) = utils::extract_whitespace(string);

        let (string, op) = Operation::new(string);
        let (string, _) = utils::extract_whitespace(string);

        let (string, rhs) = Number::new(string);

        return (string, Self { lhs, rhs, op });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number() {
        let line = line!();
        println!("{line}");

        assert_eq!(Number::new("123"), ("", Number(123)));
    }
    #[test]
    fn parse_add_operator() {
        assert_eq!(Operation::new("+"), ("", Operation::Add));
    }
    #[test]
    fn parse_sub_operator() {
        assert_eq!(Operation::new("-"), ("", Operation::Sub));
    }
    #[test]
    fn parse_mul_operator() {
        assert_eq!(Operation::new("*"), ("", Operation::Mul));
    }
    #[test]
    fn parse_div_operator() {
        assert_eq!(Operation::new("/"), ("", Operation::Div));
    }
    #[test]
    fn parse_one_plus_two() {
        assert_eq!(
            Expression::new("1+2"),
            (
                "",
                Expression {
                    lhs: Number(1),
                    rhs: Number(2),
                    op: Operation::Add
                }
            )
        );
    }
    #[test]
    fn parse_exprestion_with_whitespace() {
        assert_eq!(
            Expression::new("2 * 2"),
            (
                "",
                Expression {
                    lhs: Number(2),
                    rhs: Number(2),
                    op: Operation::Mul
                }
            )
        );
    }
}
