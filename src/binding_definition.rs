use crate::environment::Enviroment;
use crate::expression::Expression;
use crate::utils;

#[derive(Debug, PartialEq)]
pub struct BindingDef {
    name: String,
    val: Expression,
}

impl BindingDef {
    pub fn new(string: &str) -> (&str, Self) {
        let string = utils::tag("let", string);
        let (string, _) = utils::extract_whitespace(string);

        let (string, name) = utils::extract_identifier(string);
        let (string, _) = utils::extract_whitespace(string);

        let string = utils::tag("=", string);
        let (string, _) = utils::extract_whitespace(string);

        let (string, value) = Expression::new(string);

        (
            string,
            Self {
                name: name.to_string(),
                val: value,
            },
        )
    }
    pub fn evaluate(&self, environment: &mut Enviroment) {
        environment.store_binding(self.name.clone(), self.val.evaluate());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expression::{Number, Operation};

    #[test]
    fn parse_binding_def() {
        assert_eq!(
            BindingDef::new("let a = 10 / 2"),
            (
                "",
                BindingDef {
                    name: "a".to_string(),
                    val: Expression {
                        lhs: Number(10),
                        rhs: Number(2),
                        op: Operation::Division
                    }
                }
            )
        );
    }
}
