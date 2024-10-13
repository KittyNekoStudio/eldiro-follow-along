use crate::environment::Enviroment;
use crate::utils;
use crate::value::Value;

#[derive(Debug, PartialEq)]
pub struct BindingUsage {
    name: String,
}

impl BindingUsage {
    pub fn new(string: &str) -> Result<(&str, Self), String> {
        let (string, name) = utils::extract_identifier(string)?;

        Ok((
            string,
            BindingUsage {
                name: name.to_string(),
            },
        ))
    }
    pub(crate) fn eval(&self, enviroment: &Enviroment) -> Result<Value, String> {
        enviroment.get_binding_value(&self.name)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_binding_usage() {
        assert_eq!(
            BindingUsage::new("abc"),
            Ok((
                "",
                BindingUsage {
                    name: "abc".to_string()
                }
            ))
        );
    }
    #[test]
    fn evaluate_existing_binding_usage() {
        let mut enviroment = Enviroment::default();
        enviroment.store_binding("foo".to_string(), Value::Number(10));

        assert_eq!(
            BindingUsage {
                name: "foo".to_string()
            }
            .eval(&enviroment),
            Ok(Value::Number(10))
        );
    }
    #[test]
    fn evaluate_non_existing_binding_usage() {
        let empty_enviroment = Enviroment::default();

        assert_eq!(
            BindingUsage {
                name: "i_dont_exist".to_string()
            }
            .eval(&empty_enviroment),
            Err("binding with name 'i_dont_exist' does not exist".to_string())
        );
    }
}
