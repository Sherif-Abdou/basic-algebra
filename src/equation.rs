use crate::expression::*;

pub struct Equation {
    tokens: Vec<String>,
    pub right_side: Option<ExpressionType>,
    pub left_side: Option<ExpressionType>,
}

impl Equation {
    pub fn from_tokens(tokens: Vec<String>) -> Equation {
        Equation {
            tokens,
            left_side: None,
            right_side: None,
        }
    }

    pub fn parse(&self) -> Result<(Vec<String>, Vec<String>), &'static str> {
        let cloned_tokens = self.tokens.clone();
        for (i, token) in cloned_tokens.iter().enumerate() {
            if token == "=" {
                let (left_side, right_side) = self.split_tokens(i);

                return Ok((left_side, right_side));
            }
        }
        Err("No equal sign")
    }

    fn split_tokens(&self, i: usize) -> (Vec<String>, Vec<String>) {
        let mut right_side = self.tokens.to_vec();
        let left_side: Vec<String> = right_side.drain(..i).collect();
        right_side.drain(0..1);
        (left_side, right_side)
    }
}
