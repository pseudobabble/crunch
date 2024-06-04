use std::collections::HashMap;
use std::mem::discriminant;
use std::ops::{Add, Div, Mul, Sub};

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum BinaryOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

/// This is the conversion factor to base units
#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum UnitConversionRate {
    None = 1,
    Second = 0.000011574074074074073499,
    Minute = 0.0006944444444444445,
    Hour = 0.041666666666666664,
    Day = 1,
    Meter = 1,
    Kilometer = 1000,
    USD = 1,
    GBP = 0.8,
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct Unit {
    conversion_rate: UnitConversionRate,
    exponent: i64,
    dimension: Dimension,
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum DimensionIdentity {
    None(UnitConversionRate::None),
    Time(UnitConversionRate::Hour),
    Length(UnitConversionRate::Meter),
    Currency(UnitConversionRate::USD),
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct Dimension {
    dimension: HashMap<DimensionIdentity, i64>,
}

impl Add for Dimension {
    type Output = Dimension;

    fn add(self, rhs: Self) -> Self {
        if discriminant(self) == discriminant(rhs) {
            self
        } else {
            panic!("Can't add lhs: {:#?} to rhs: {:#?}", self, rhs)
        }
    }
}

impl Sub for Dimension {
    type Output = Dimension;

    fn sub(self, rhs: Self) -> Self {
        if discriminant(self) == discriminant(rhs) {
            self
        } else {
            panic!("Can't subtract rhs: {:#?} from lhs: {:#?}", rhs, self)
        }
    }
}

impl Mul for Dimension {
    type Output = Dimension;

    fn mul(self, rhs: Self) -> Self {
        let mut result = self.dimension.clone();
        for (key, value) in rhs.dimension {
            *result.entry(key.clone()).or_insert(0) += value;
        }

        Dimension { dimension: result }
    }
}

impl Div for Dimension {
    type Output = Dimension;

    fn div(self, rhs: Self) -> Self {
        let mut result = self.dimension.clone();
        for (key, value) in rhs.dimension {
            *result.entry(key.clone()).or_insert(0) -= value;
        }

        Dimension { dimension: result }
    }
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
    pub unit: Unit,
}

impl Add for DimensionedValue {
    type Output = DimensionedValue;

    fn add(self, rhs: Self) -> Self {
        println!("\n\nAdding {:#?} to {:#?}", self, rhs);

        let lhs_in_base_units = self.value * Value::Float(self.unit.conversion_rate);
        let rhs_in_base_units = rhs.value * Value::Float(rhs.unit.conversion_rate);

        let value = lhs_in_base_units + rhs_in_base_units;

        println!("\nResult = {:#?}", value);

        DimensionedValue {
            value: value,
            unit: Unit {
                dimension: self.unit.dimension + rhs.unit.dimension,
            },
        }
    }
}

impl Sub for DimensionedValue {
    type Output = DimensionedValue;

    fn sub(self, rhs: Self) -> Self {
        println!("\n\nSubtracting {:#?} from {:#?}", rhs, self);

        let lhs_in_base_units = self.value * Value::Float(self.unit.conversion_rate);
        let rhs_in_base_units = rhs.value * Value::Float(rhs.unit.conversion_rate);

        let value = lhs_in_base_units - rhs_in_base_units;

        println!("\nResult = {:#?}", value);

        DimensionedValue {
            value: value,
            unit: Unit {
                dimension: self.unit.dimension - rhs.unit.dimension,
            },
        }
    }
}

impl Mul for DimensionedValue {
    type Output = DimensionedValue;

    fn mul(self, rhs: Self) -> Self {
        println!("\n\nMultiplying {:#?} with {:#?}", self, rhs);

        let lhs_in_base_units = self.value * Value::Float(self.unit.conversion_rate);
        let rhs_in_base_units = rhs.value * Value::Float(rhs.unit.conversion_rate);

        let value = lhs_in_base_units * rhs_in_base_units;

        println!("\nResult = {:#?}", value);

        DimensionedValue {
            value: value,
            unit: Unit {
                dimension: self.unit.dimension * rhs.unit.dimension,
            },
        }
    }
}

impl Div for DimensionedValue {
    type Output = DimensionedValue;

    fn div(self, rhs: Self) -> Self {
        println!("\n\nDividing {:#?} into {:#?}", self, rhs);

        let lhs_in_base_units = self.value * Value::Float(self.unit.conversion_rate);
        let rhs_in_base_units = rhs.value * Value::Float(rhs.unit.conversion_rate);

        let value = lhs_in_base_units / rhs_in_base_units;

        println!("\nResult = {:#?}", value);

        DimensionedValue {
            value: value,
            unit: Unit {
                dimension: self.unit.dimension / rhs.unit.dimension,
            },
        }
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
