use crate::expression::{ExpressionSide, ExpressionType};
use std::borrow::Borrow;

#[derive(Debug, Clone)]
pub struct EquationSolver {
    pub left_side: ExpressionType,
    pub right_side: ExpressionType,
}

impl EquationSolver {
    pub fn from(left_side: ExpressionType, right_side: ExpressionType) -> EquationSolver {
        EquationSolver {
            left_side,
            right_side,
        }
    }

    pub fn solve_step(&mut self) -> Result<EquationSolver, &'static str> {
        let mut variable_path = self.get_variable_path()?;

        let side = &variable_path[0];

        let mut right_side = self.right_side.clone();
        let mut flip = false;

        match &self.left_side {
            ExpressionType::Addition(box_tuple) => match side {
                ExpressionSide::Right => {
                    right_side =
                        ExpressionType::Subtraction(Box::new((right_side, (*box_tuple.clone()).0)))
                }
                ExpressionSide::Left => {
                    right_side =
                        ExpressionType::Subtraction(Box::new((right_side, (*box_tuple.clone()).1)))
                }
            },
            ExpressionType::Multiplication(box_tuple) => match side {
                ExpressionSide::Right => {
                    right_side =
                        ExpressionType::Division(Box::new((right_side, (*box_tuple.clone()).0)))
                }
                ExpressionSide::Left => {
                    right_side =
                        ExpressionType::Division(Box::new((right_side, (*box_tuple.clone()).1)))
                }
            },
            ExpressionType::Subtraction(box_tuple) => match side {
                ExpressionSide::Right => {
                    right_side = ExpressionType::Multiplication(Box::new((
                        ExpressionType::Constant(-1.0),
                        ExpressionType::Subtraction(Box::new((right_side, (*box_tuple.clone()).0))),
                    )))
                }
                ExpressionSide::Left => {
                    right_side =
                        ExpressionType::Addition(Box::new((right_side, (*box_tuple.clone()).1)))
                }
            },
            ExpressionType::Division(box_tuple) => match side {
                ExpressionSide::Left => {
                    right_side = ExpressionType::Multiplication(Box::new((
                        (*box_tuple.clone()).1,
                        right_side,
                    )));
                }
                ExpressionSide::Right => {
                    right_side = ExpressionType::Multiplication(Box::new((
                        (*box_tuple.clone()).1,
                        right_side,
                    )));
                    flip = true;
                }
            },
            _ => {}
        }

        let new_left_side = if (!flip) {
            match &self.left_side {
                ExpressionType::Addition(box_tuple)
                | ExpressionType::Subtraction(box_tuple)
                | ExpressionType::Multiplication(box_tuple)
                | ExpressionType::Division(box_tuple) => match side {
                    ExpressionSide::Right => Some((*box_tuple.clone()).1),
                    ExpressionSide::Left => Some((*box_tuple.clone()).0),
                },
                _ => None,
            }
        } else {
            match &self.left_side {
                ExpressionType::Addition(box_tuple)
                | ExpressionType::Subtraction(box_tuple)
                | ExpressionType::Multiplication(box_tuple)
                | ExpressionType::Division(box_tuple) => match side {
                    ExpressionSide::Right => Some((*box_tuple.clone()).0),
                    ExpressionSide::Left => Some((*box_tuple.clone()).1),
                },
                _ => None,
            }
        };

        return if (!flip) {
            Ok(EquationSolver::from(new_left_side.unwrap(), right_side))
        } else {
            Ok(EquationSolver::from(right_side, new_left_side.unwrap()))
        };
    }

    fn get_variable_path(&self) -> Result<Vec<ExpressionSide>, &'static str> {
        if let Some(variable_path) = self.left_side.find_variable() {
            return Ok(variable_path);
        }

        Err("Couldn't find path")
    }
}
