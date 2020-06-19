use std::ops::Deref;

#[derive(Debug, Clone)]
pub enum ExpressionType {
    Addition(Box<(ExpressionType, ExpressionType)>),
    Subtraction(Box<(ExpressionType, ExpressionType)>),
    Multiplication(Box<(ExpressionType, ExpressionType)>),
    Division(Box<(ExpressionType, ExpressionType)>),
    Variable(String),
    Constant(f64),
}

#[derive(Debug, Clone)]
pub enum ExpressionSide {
    Left,
    Right,
}

impl ExpressionType {
    fn parse_operation(
        tokens: &Vec<String>,
        operation: &str,
    ) -> Option<(ExpressionType, ExpressionType)> {
        for (i, token) in tokens.iter().enumerate() {
            if token == operation {
                // println!("Found operator using tokens {:?}", operation);
                let split_tokens = ExpressionType::split_tokens(tokens, i);
                let parsed_split_tokens = (
                    ExpressionType::parse_tokens(split_tokens.0),
                    ExpressionType::parse_tokens(split_tokens.1),
                );
                if let (Ok(left_side), Ok(right_side)) = parsed_split_tokens {
                    return Some((left_side, right_side));
                }
            }
        }
        None
    }

    fn split_tokens(tokens: &Vec<String>, index: usize) -> (Vec<String>, Vec<String>) {
        let mut right_side = tokens.to_vec();
        let left_side: Vec<String> = right_side.drain(..index).collect();
        right_side.drain(0..1);
        (left_side, right_side)
    }

    pub fn parse_tokens(tokens: Vec<String>) -> Result<ExpressionType, &'static str> {
        if tokens.len() != 1 {
            let operations = vec!["+", "-", "*", "/"];
            for operation in operations {
                for token in tokens.iter() {
                    if token == operation {
                        match token.as_str() {
                            "*" => {
                                return Ok(ExpressionType::Multiplication(Box::new(
                                    ExpressionType::parse_operation(&tokens, "*").unwrap(),
                                )))
                            }
                            "/" => {
                                return Ok(ExpressionType::Division(Box::new(
                                    ExpressionType::parse_operation(&tokens, "/").unwrap(),
                                )))
                            }
                            "+" => {
                                return Ok(ExpressionType::Addition(Box::new(
                                    ExpressionType::parse_operation(&tokens, "+").unwrap(),
                                )))
                            }
                            "-" => {
                                return Ok(ExpressionType::Subtraction(Box::new(
                                    ExpressionType::parse_operation(&tokens, "-").unwrap(),
                                )))
                            }
                            _ => {}
                        }
                    }
                }
            }
        } else {
            let token = &tokens[0];
            if let Ok(number) = token.parse::<f64>() {
                return Ok(ExpressionType::Constant(number));
            } else {
                return Ok(ExpressionType::Variable(token.clone()));
            }
        }

        Err("Failed to parse")
    }

    pub fn find_variable(&self) -> Option<Vec<ExpressionSide>> {
        let mut output_vec: Vec<ExpressionSide> = vec![];
        match self {
            ExpressionType::Addition(box_values)
            | ExpressionType::Subtraction(box_values)
            | ExpressionType::Multiplication(box_values)
            | ExpressionType::Division(box_values) => {
                if let Some(mut vec) = box_values.deref().0.find_variable() {
                    output_vec.push(ExpressionSide::Left);
                    output_vec.append(&mut vec);
                }
                if let Some(mut vec) = box_values.deref().1.find_variable() {
                    output_vec.push(ExpressionSide::Right);
                    output_vec.append(&mut vec);
                }
                return Some(output_vec);
            }
            ExpressionType::Constant(_) => {}
            ExpressionType::Variable(_) => return Some(output_vec),
        };

        None
    }
}
