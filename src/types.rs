use std::collections::HashMap;
use std::ops::{Add, Div, Mul, Sub};
use uom::si::Dimension;

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum BinaryOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Unit {
    Meter(i64),
    Kilometer(i64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Float(f64),
    Vec(Vec<f64>),
}

fn scalar_vector_addition(scalar: f64, vector: Vec<f64>) -> Vec<f64> {
    vector.iter().map(|left_x| left_x + scalar).collect()
}

fn scalar_vector_subtraction(scalar: f64, vector: Vec<f64>) -> Vec<f64> {
    vector.iter().map(|left_x| left_x - scalar).collect()
}

fn scalar_vector_multiplication(scalar: f64, vector: Vec<f64>) -> Vec<f64> {
    vector.iter().map(|left_x| left_x * scalar).collect()
}

fn scalar_vector_division(scalar: f64, vector: Vec<f64>) -> Vec<f64> {
    vector.iter().map(|left_x| left_x / scalar).collect()
}

fn elementwise_vector_addition(lhs_value: Vec<f64>, rhs_value: Vec<f64>) -> Vec<f64> {
    lhs_value
        .iter()
        .zip(rhs_value)
        .map(|(left_x, right_x)| left_x + right_x)
        .collect()
}

fn elementwise_vector_subtraction(lhs_value: Vec<f64>, rhs_value: Vec<f64>) -> Vec<f64> {
    lhs_value
        .iter()
        .zip(rhs_value)
        .map(|(left_x, right_x)| left_x - right_x)
        .collect()
}

fn elementwise_vector_multiplication(lhs_value: Vec<f64>, rhs_value: Vec<f64>) -> Vec<f64> {
    lhs_value
        .iter()
        .zip(rhs_value)
        .map(|(left_x, right_x)| left_x * right_x)
        .collect()
}

fn elementwise_vector_division(lhs_value: Vec<f64>, rhs_value: Vec<f64>) -> Vec<f64> {
    lhs_value
        .iter()
        .zip(rhs_value)
        .map(|(left_x, right_x)| left_x / right_x)
        .collect()
}

impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self {
        match self {
            // we are float
            Value::Float(lhs_value) => match rhs {
                // they are float
                Value::Float(rhs_value) => Value::Float(lhs_value + rhs_value),
                // they are vec
                Value::Vec(rhs_value) => Value::Vec(scalar_vector_addition(lhs_value, rhs_value)),
            },
            // we are vec
            Value::Vec(lhs_value) => match rhs {
                // they are float
                Value::Float(rhs_value) => Value::Vec(scalar_vector_addition(rhs_value, lhs_value)),
                // they are vec
                Value::Vec(rhs_value) => {
                    Value::Vec(elementwise_vector_addition(lhs_value, rhs_value))
                }
            },
        }
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Self) -> Self {
        match self {
            // we are float
            Value::Float(lhs_value) => match rhs {
                // they are float
                Value::Float(rhs_value) => Value::Float(lhs_value - rhs_value),
                // they are vec
                Value::Vec(rhs_value) => {
                    Value::Vec(scalar_vector_subtraction(lhs_value, rhs_value))
                }
            },
            // we are vec
            Value::Vec(lhs_value) => match rhs {
                // they are float
                Value::Float(rhs_value) => {
                    Value::Vec(scalar_vector_subtraction(rhs_value, lhs_value))
                }
                // they are vec
                Value::Vec(rhs_value) => {
                    Value::Vec(elementwise_vector_subtraction(lhs_value, rhs_value))
                }
            },
        }
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Self) -> Self {
        match self {
            // we are float
            Value::Float(lhs_value) => match rhs {
                // they are float
                Value::Float(rhs_value) => Value::Float(lhs_value * rhs_value),
                // they are vec
                Value::Vec(rhs_value) => {
                    Value::Vec(scalar_vector_multiplication(lhs_value, rhs_value))
                }
            },
            // we are vec
            Value::Vec(lhs_value) => match rhs {
                // they are float
                Value::Float(rhs_value) => {
                    Value::Vec(scalar_vector_multiplication(rhs_value, lhs_value))
                }
                // they are vec
                Value::Vec(rhs_value) => {
                    Value::Vec(elementwise_vector_multiplication(lhs_value, rhs_value))
                }
            },
        }
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, rhs: Self) -> Self {
        match self {
            // we are float
            Value::Float(lhs_value) => match rhs {
                // they are float
                Value::Float(rhs_value) => Value::Float(lhs_value / rhs_value),
                // they are vec
                Value::Vec(rhs_value) => Value::Vec(scalar_vector_division(lhs_value, rhs_value)),
            },
            // we are vec
            Value::Vec(lhs_value) => match rhs {
                // they are float
                Value::Float(rhs_value) => Value::Vec(scalar_vector_division(rhs_value, lhs_value)),
                // they are vec
                Value::Vec(rhs_value) => {
                    Value::Vec(elementwise_vector_division(lhs_value, rhs_value))
                }
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct DimensionedValue {
    pub value: Value,
}

impl Add for DimensionedValue {
    type Output = DimensionedValue;

    fn add(self, rhs: Self) -> Self {
        println!("\n\nAdding {:#?} to {:#?}", self, rhs);

        let value = self.value + rhs.value;

        println!("\nResult = {:#?}", value);

        DimensionedValue { value: value }
    }
}

impl Sub for DimensionedValue {
    type Output = DimensionedValue;

    fn sub(self, rhs: Self) -> Self {
        println!("\n\nSubtracting {:#?} from {:#?}", rhs, self);

        let value = self.value - rhs.value;

        println!("\nResult = {:#?}", value);

        DimensionedValue { value: value }
    }
}

impl Mul for DimensionedValue {
    type Output = DimensionedValue;

    fn mul(self, rhs: Self) -> Self {
        println!("\n\nMultiplying {:#?} with {:#?}", self, rhs);

        let value = self.value * rhs.value;

        println!("\nResult = {:#?}", value);

        DimensionedValue { value: value }
    }
}

impl Div for DimensionedValue {
    type Output = DimensionedValue;

    fn div(self, rhs: Self) -> Self {
        println!("\n\nDividing {:#?} into {:#?}", self, rhs);

        let value = self.value / rhs.value;

        println!("\nResult = {:#?}", value);

        DimensionedValue { value: value }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum AstNode {
    Print(Box<AstNode>),
    Double {
        value: Value,
        unit: Unit,
    },
    Vector {
        value: Value,
        unit: Unit,
    },
    Name(String),
    Expression {
        operation: BinaryOperation,
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
    },
    Variable {
        name: Box<AstNode>,
        expr: Box<AstNode>,
    },
}
