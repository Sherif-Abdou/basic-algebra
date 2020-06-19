extern crate regex;

mod equation;
mod equation_solver;
mod expression;

use crate::equation::*;
use crate::equation_solver::*;
use crate::expression::*;
use regex::Regex;
use std::env;

fn tokenize(argument: &str) -> Vec<String> {
    let re = Regex::new(r"[0-9]+|\+|-|\*|/|=|[A-Za-z]").unwrap();
    let mut values: Vec<String> = re
        .find_iter(argument)
        .map(|val| String::from(val.as_str()))
        .collect();
    let mut insert_indexes: Vec<usize> = vec![];
    for (i, value) in values.iter().enumerate() {
        if i + 1 < values.len() {
            if let Ok(_) = value.parse::<f64>() {
                if values[i + 1].chars().all(char::is_alphanumeric) {
                    insert_indexes.push(i + 1);
                }
            }
        }
    }

    for index in insert_indexes.iter() {
        values.insert(*index, String::from("*"));
    }
    values
}

fn recursive_solve(mut solver: EquationSolver) -> EquationSolver {
    let mut new_eq_result = solver.solve_step();
    let new_eq = new_eq_result.unwrap();

    return if let ExpressionType::Variable(_) = new_eq.left_side {
        new_eq
    } else {
        recursive_solve(new_eq)
    };
}

fn simplify_side(expression: ExpressionType) -> ExpressionType {
    match &expression {
        ExpressionType::Addition(box_tuple) => {
            let values = (*box_tuple.clone());
            if let (ExpressionType::Constant(a), ExpressionType::Constant(b)) =
                (simplify_side(values.0), simplify_side(values.1))
            {
                return ExpressionType::Constant(a + b);
            }
        }
        ExpressionType::Subtraction(box_tuple) => {
            let values = (*box_tuple.clone());
            if let (ExpressionType::Constant(a), ExpressionType::Constant(b)) =
                (simplify_side(values.0), simplify_side(values.1))
            {
                return ExpressionType::Constant(a - b);
            }
        }
        ExpressionType::Multiplication(box_tuple) => {
            let values = (*box_tuple.clone());
            if let (ExpressionType::Constant(a), ExpressionType::Constant(b)) =
                (simplify_side(values.0), simplify_side(values.1))
            {
                return ExpressionType::Constant(a * b);
            }
        }
        ExpressionType::Division(box_tuple) => {
            let values = (*box_tuple.clone());
            if let (ExpressionType::Constant(a), ExpressionType::Constant(b)) =
                (simplify_side(values.0), simplify_side(values.1))
            {
                return ExpressionType::Constant(a / b);
            }
        }
        _ => {
            return expression.clone();
        }
    }

    return expression.clone();
}

fn eqsolver_to_string(solver: EquationSolver) -> String {
    if let (ExpressionType::Variable(letter), ExpressionType::Constant(value)) =
        (&solver.left_side, &solver.right_side)
    {
        return format!("{} = {}", letter, value);
    }
    return String::from(format!("Unable to create a string from {:?}", solver));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(raw_tokens) = args.get(1) {
        let tokens = tokenize(raw_tokens);
        let mut equation = equation::Equation::from_tokens(tokens);
        let parsed_values = equation.parse().expect("Couldn't find equal sign");
        let expressions = (
            expression::ExpressionType::parse_tokens(parsed_values.0),
            expression::ExpressionType::parse_tokens(parsed_values.1),
        );
        let mut solver =
            equation_solver::EquationSolver::from(expressions.0.unwrap(), expressions.1.unwrap());
        let solved = recursive_solve(solver);
        let simplfied_side =
            EquationSolver::from(solved.left_side, simplify_side(solved.right_side));
        println!("{}", eqsolver_to_string(simplfied_side));
    } else {
        println!("Missing input");
    }
}
