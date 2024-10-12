use crate::utils;
use crate::value::Value;

#[derive(Debug, PartialEq)]
pub struct Number(pub i32);

#[derive(Debug, PartialEq)]
pub enum Operation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Number(Number),
    Operation {
        lhs: Number,
        rhs: Number,
        op: Operation,
    },
}

impl Number {
    pub fn new(string: &str) -> Result<(&str, Self), String> {
        let (string, number) = utils::extract_digits(string)?;
        Ok((string, Self(number.parse().unwrap())))
    }
}

impl Operation {
    pub fn new(string: &str) -> Result<(&str, Self), String> {
        utils::tag("+", string)
            .map(|string| (string, Self::Addition))
            .or_else(|_| utils::tag("-", string).map(|string| (string, Self::Subtraction)))
            .or_else(|_| utils::tag("*", string).map(|string| (string, Self::Multiplication)))
            .or_else(|_| utils::tag("/", string).map(|string| (string, Self::Division)))
    }
}

impl Expression {
    pub fn new(string: &str) -> Result<(&str, Self), String> {
        Self::new_operation(string).or_else(|_| Self::new_number(string))
    }
    fn new_operation(string: &str) -> Result<(&str, Self), String> {
        let (string, lhs) = Number::new(string)?;
        let (string, _) = utils::extract_whitespace(string);

        let (string, op) = Operation::new(string)?;
        let (string, _) = utils::extract_whitespace(string);

        let (string, rhs) = Number::new(string)?;

        Ok((string, Self::Operation { lhs, rhs, op }))
    }
    pub fn new_number(string: &str) -> Result<(&str, Self), String> {
        Number::new(string).map(|(string, number)| (string, Self::Number(number)))
    }
    pub(crate) fn evaluate(&self) -> Value {
        match self {
            Self::Number(Number(n)) => Value::Number(*n),
            Self::Operation { lhs, rhs, op } => {
                let Number(lhs) = lhs;
                let Number(rhs) = rhs;

                let result = match op {
                    Operation::Addition => lhs + rhs,
                    Operation::Subtraction => lhs - rhs,
                    Operation::Multiplication => lhs * rhs,
                    Operation::Division => lhs / rhs,
                };
                Value::Number(result)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number() {
        let line = line!();
        println!("{line}");

        assert_eq!(Number::new("123"), Ok(("", Number(123))));
    }
    #[test]
    fn parse_add_operator() {
        assert_eq!(Operation::new("+"), Ok(("", Operation::Addition)));
    }
    #[test]
    fn parse_sub_operator() {
        assert_eq!(Operation::new("-"), Ok(("", Operation::Subtraction)));
    }
    #[test]
    fn parse_mul_operator() {
        assert_eq!(Operation::new("*"), Ok(("", Operation::Multiplication)));
    }
    #[test]
    fn parse_div_operator() {
        assert_eq!(Operation::new("/"), Ok(("", Operation::Division)));
    }
    #[test]
    fn parse_one_plus_two() {
        assert_eq!(
            Expression::new("1+2"),
            Ok((
                "",
                Expression::Operation {
                    lhs: Number(1),
                    rhs: Number(2),
                    op: Operation::Addition
                }
            ))
        );
    }
    #[test]
    fn parse_exprestion_with_whitespace() {
        assert_eq!(
            Expression::new("2 * 2"),
            Ok((
                "",
                Expression::Operation {
                    lhs: Number(2),
                    rhs: Number(2),
                    op: Operation::Multiplication
                }
            ))
        );
    }
    #[test]
    fn evaluate_add() {
        assert_eq!(
            Expression::Operation {
                lhs: Number(10),
                rhs: Number(10),
                op: Operation::Addition
            }
            .evaluate(),
            Value::Number(20)
        );
    }
    #[test]
    fn evaluate_sub() {
        assert_eq!(
            Expression::Operation {
                lhs: Number(1),
                rhs: Number(5),
                op: Operation::Subtraction
            }
            .evaluate(),
            Value::Number(-4)
        );
    }
    #[test]
    fn evaluate_multiply() {
        assert_eq!(
            Expression::Operation {
                lhs: Number(5),
                rhs: Number(6),
                op: Operation::Multiplication
            }
            .evaluate(),
            Value::Number(30)
        );
    }
    #[test]
    fn evaluate_divide() {
        assert_eq!(
            Expression::Operation {
                lhs: Number(200),
                rhs: Number(20),
                op: Operation::Division
            }
            .evaluate(),
            Value::Number(10)
        );
    }
    #[test]
    fn parse_number_as_expression() {
        assert_eq!(
            Expression::new("456"),
            Ok(("", Expression::Number(Number(456))))
        );
    }
}
