use crate::environment::Enviroment;
use crate::expression::Expression;
use crate::utils;

#[derive(Debug, PartialEq)]
pub struct BindingDef {
    name: String,
    val: Expression,
}

impl BindingDef {
    pub fn new(string: &str) -> Result<(&str, Self), String> {
        let string = utils::tag("let", string)?;
        let (string, _) = utils::extract_whitespace1(string)?;

        let (string, name) = utils::extract_identifier(string)?;
        let (string, _) = utils::extract_whitespace(string);

        let string = utils::tag("=", string)?;
        let (string, _) = utils::extract_whitespace(string);

        let (string, value) = Expression::new(string)?;

        Ok((
            string,
            Self {
                name: name.to_string(),
                val: value,
            },
        ))
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
            Ok((
                "",
                BindingDef {
                    name: "a".to_string(),
                    val: Expression::Operation {
                        lhs: Number(10),
                        rhs: Number(2),
                        op: Operation::Division
                    }
                }
            ))
        );
    }
    #[test]
    fn cannot_parse_binding_def_without_space_after_let() {
        assert_eq!(
            BindingDef::new("letaaa=1+2"),
            Err("expected a space".to_string())
        );
    }
}
