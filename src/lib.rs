#[derive(Debug, PartialEq)]
pub struct Number(pub i32);

#[derive(Debug, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Number {
    pub fn new(string: &str) -> Self {
        Self(string.parse().unwrap())
    }
}

impl Op {
    pub fn new(string: &str) -> Self {
        match string {
            "+" => Op::Add,
            "-" => Op::Sub,
            "*" => Op::Mul,
            "/" => Op::Div,
            _ => panic!("bad operator"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number() {
        assert_eq!(Number::new("123"), Number(123));
    }
    #[test]
    fn parse_add_operator() {
        assert_eq!(Op::new("+"), Op::Add);
    }
    #[test]
    fn parse_sub_operator() {
        assert_eq!(Op::new("-"), Op::Sub);
    }
    #[test]
    fn parse_mul_operator() {
        assert_eq!(Op::new("*"), Op::Mul);
    }
    #[test]
    fn parse_div_operator() {
        assert_eq!(Op::new("/"), Op::Div);
    }
}
