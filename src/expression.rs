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
            "+" => Operation::Addition,
            "-" => Operation::Subtraction,
            "*" => Operation::Multiplication,
            "/" => Operation::Division,
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

        (string, Self { lhs, rhs, op })
    }
    pub(crate) fn evaluate(&self) -> Value {
        let Number(lhs) = self.lhs;
        let Number(rhs) = self.rhs;

        let result = match self.op {
            Operation::Addition => lhs + rhs,
            Operation::Subtraction => lhs - rhs,
            Operation::Multiplication => lhs * rhs,
            Operation::Division => lhs / rhs,
        };
        Value::Number(result)
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
        assert_eq!(Operation::new("+"), ("", Operation::Addition));
    }
    #[test]
    fn parse_sub_operator() {
        assert_eq!(Operation::new("-"), ("", Operation::Subtraction));
    }
    #[test]
    fn parse_mul_operator() {
        assert_eq!(Operation::new("*"), ("", Operation::Multiplication));
    }
    #[test]
    fn parse_div_operator() {
        assert_eq!(Operation::new("/"), ("", Operation::Division));
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
                    op: Operation::Addition
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
                    op: Operation::Multiplication
                }
            )
        );
    }
    #[test]
    fn evaluate_add() {
        assert_eq!(
            Expression {
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
            Expression {
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
            Expression {
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
            Expression {
                lhs: Number(200),
                rhs: Number(20),
                op: Operation::Division
            }
            .evaluate(),
            Value::Number(10)
        );
    }
}
