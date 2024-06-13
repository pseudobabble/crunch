use super::types::*;

use std::clone::Clone;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Memory {
    instructions: Vec<Vec<AstNode>>,
    pub memory: HashMap<String, Value>,
}

pub trait Interpreter {
    fn new(instructions: Vec<Vec<AstNode>>) -> Self;
    fn run(&mut self) -> ();
    fn evaluate(&self, expression: AstNode) -> Value;
    fn evaluate_expression(
        &self,
        operation: BinaryOperation,
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
    ) -> Value;
}

impl Interpreter for Memory {
    fn new(instructions: Vec<Vec<AstNode>>) -> Self {
        Memory {
            instructions: instructions,
            memory: HashMap::new(),
        }
    }

    fn run(&mut self) -> () {
        for line in &self.instructions {
            for variable in line {
                println!("\nCalculating {:#?}", variable.clone());
                let name = match variable {
                    AstNode::Variable { name, expr: _expr } => match *name.clone() {
                        AstNode::Name(name) => name,
                        _ => panic!("Variable name should be of type AstNode::Name"),
                    },
                    _ => panic!("Variable should be of type AstNode::Variable"),
                };

                let unitted_value = match variable {
                    AstNode::Variable {
                        name: _name,
                        expr: expression,
                    } => self.evaluate(*expression.clone()),
                    _ => panic!("Variable should be of type AstNode::Variable"),
                };

                println!(
                    "\nStoring result {:#?} = {:#?}",
                    name.clone(),
                    unitted_value.clone()
                );
                self.memory.insert(name, unitted_value);
                println!("=================================\n\n");
            }
        }
    }

    fn evaluate(&self, expression: AstNode) -> Value {
        match expression {
            AstNode::Name(name) => self.memory[&name].clone(),
            AstNode::Double { value, unit } => Value::new(value, unit),
            AstNode::Vector { value, unit } => Value::new_vec(value, unit),
            AstNode::Expression {
                operation,
                lhs,
                rhs,
            } => self.evaluate_expression(operation, lhs, rhs),
            _ => panic!("1 Expression should be of type AstNode::Expression, AstNode::Double, AstNode::Vector, or AstNode::Name, found: {:#?}", expression),
        }
    }

    fn evaluate_expression(
        &self,
        operation: BinaryOperation,
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
    ) -> Value {
        let lhs_value = match *lhs {
            AstNode::Name(name) => self.memory[&name].clone(),
            AstNode::Double { value, unit } => Value::new(value, unit),
            AstNode::Vector { value, unit } => Value::new_vec(value, unit),
            AstNode::Expression {
                operation,
                lhs,
                rhs,
            } => self.evaluate_expression(operation, lhs, rhs),
            _ => panic!("2 Expression should be of type AstNode::Expression, AstNode::Double, AstNode::Vector, or AstNode::Name, found: {:#?}", lhs),
        };

        let rhs_value = match *rhs {
            AstNode::Name(name) => self.memory[&name].clone(),
            AstNode::Double { value, unit } => Value::new(value, unit),
            AstNode::Vector { value, unit } => Value::new_vec(value, unit),
            AstNode::Expression {
                operation,
                lhs,
                rhs,
            } => self.evaluate_expression(operation, lhs, rhs),
            _ => panic!("3 Expression should be of type AstNode::Expression, AstNode::Double, AstNode::Vector, or AstNode::Name, found: {:#?}", rhs),
        };

        match operation {
            BinaryOperation::Add => lhs_value + rhs_value,
            BinaryOperation::Subtract => lhs_value - rhs_value,
            BinaryOperation::Multiply => lhs_value * rhs_value,
            BinaryOperation::Divide => lhs_value / rhs_value,
        }
    }
}

#[test]
fn test_interpreter() {
    let i: Memory = Interpreter::new(vec![vec![AstNode::Variable {
        name: Box::new(AstNode::Name("var".to_string())),
        expr: Box::new(AstNode::Expression {
            operation: BinaryOperation::Divide,
            lhs: Box::new(AstNode::Double {
                value: Value::Float(2.0),
                unit: Unit::Length {
                    unit: Unit {
                        unit: UnitIdentity::Meter,
                        conversion_factor: 1.0,
                    },
                    power: 1,
                },
            }),
            rhs: Box::new(AstNode::Double {
                value: Value::Float(2.0),
                unit: Unit::Length {
                    unit: Unit {
                        unit: UnitIdentity::Kilometer,
                        conversion_factor: 1000.0,
                    },
                    power: 1,
                },
            }),
        }),
    }]]);

    println!("{:#?}", i.memory);
}
