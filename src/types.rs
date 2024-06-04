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

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum UnitIdentity {
    None,
    Second,
    Minute,
    Hour,
    Day,
    Meter,
    Kilometer,
    USD,
    GBP,
}

impl UnitIdentity {
    fn get_dimension(self) {
        match self {
            UnitIdentity::None => DimensionIdentity::None,
            UnitIdentity::Second => DimensionIdentity::Time,
            UnitIdentity::Minute => DimensionIdentity::Time,
            UnitIdentity::Hour => DimensionIdentity::Time,
            UnitIdentity::Day => DimensionIdentity::Time,
            UnitIdentity::Meter => DimensionIdentity::Length,
            UnitIdentity::Kilometer => DimensionIdentity::Time,
            UnitIdentity::USD => DimensionIdentity::Time,
            UnitIdentity::GBP => DimensionIdentity::Time,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Unit {
    unit: HashMap<UnitIdentity, i64>,
    dimension: Dimension,
}

// TODO: this wont work, discriminant is for enum
impl Add for Unit {
    type Output = Unit;

    fn add(self, rhs: Self) -> Self::Output {
        // Check all keys match between self and rhs, and they have the same length
        if self.unit.keys().all(|k| rhs.unit.contains_key(k)) && self.unit.len() == rhs.unit.len() {
            self
        } else {
            panic!("Can't subtract rhs: {:#?} from lhs: {:#?}", rhs, self)
        }
    }
}

impl Sub for Unit {
    type Output = Unit;

    fn sub(self, rhs: Self) -> Self::Output {
        // Check all keys match between self and rhs, and they have the same length
        if self.unit.keys().all(|k| rhs.unit.contains_key(k)) && self.unit.len() == rhs.unit.len() {
            self
        } else {
            panic!("Can't subtract rhs: {:#?} from lhs: {:#?}", rhs, self)
        }
    }
}

impl Mul for Unit {
    type Output = Unit;

    fn mul(self, rhs: Self) -> Self {
        let mut result = self.unit.clone();
        for (key, value) in rhs.unit {
            *result.entry(key.clone()).or_insert(0) += value;
        }

        Unit { unit: result }
    }
}

impl Div for Unit {
    type Output = Unit;

    fn div(self, rhs: Self) -> Self {
        let mut result = self.unit.clone();
        for (key, value) in rhs.unit {
            *result.entry(key.clone()).or_insert(0) -= value;
        }

        Unit { unit: result }
    }
}

impl Unit {
    pub fn conversion_rate(self) -> f64 {
        match self.identity {
            UnitIdentity::None => 1.0,
            UnitIdentity::Second => 0.000011574074074074073499,
            UnitIdentity::Minute => 0.0006944444444444445,
            UnitIdentity::Hour => 0.041666666666666664,
            UnitIdentity::Day => 1.0,
            UnitIdentity::Meter => 1.0,
            UnitIdentity::Kilometer => 1000.0,
            UnitIdentity::USD => 1.0,
            UnitIdentity::GBP => 0.8,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum DimensionIdentity {
    None,
    Time,
    Length,
    Currency,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Dimension {
    dimension: HashMap<DimensionIdentity, i64>,
}

impl Add for Dimension {
    type Output = Dimension;

    fn add(self, rhs: Self) -> Self::Output {
        // Check all keys match between self and rhs, and they have the same length
        if self.dimension.keys().all(|k| rhs.dimension.contains_key(k))
            && self.dimension.len() == rhs.dimension.len()
        {
            self
        } else {
            panic!("Can't subtract rhs: {:#?} from lhs: {:#?}", rhs, self)
        }
    }
}

impl Sub for Dimension {
    type Output = Dimension;

    fn sub(self, rhs: Self) -> Self {
        // Check all keys match between self and rhs, and they have the same length
        if self.dimension.keys().all(|k| rhs.dimension.contains_key(k))
            && self.dimension.len() == rhs.dimension.len()
        {
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

        let lhs_in_base_units = self.value * Value::Float(self.unit.conversion_rate());
        let rhs_in_base_units = rhs.value * Value::Float(rhs.unit.conversion_rate());

        let value = lhs_in_base_units + rhs_in_base_units;

        println!("\nResult = {:#?}", value);

        DimensionedValue {
            value: value,
            unit: Unit {
                dimension: self.unit.dimension + rhs.unit.dimension,
                exponent: self.unit.exponent,
                unit: self.unit.unit + rhs.unit.unit,
            },
        }
    }
}

impl Sub for DimensionedValue {
    type Output = DimensionedValue;

    fn sub(self, rhs: Self) -> Self {
        println!("\n\nSubtracting {:#?} from {:#?}", rhs, self);

        let lhs_in_base_units = self.value * Value::Float(self.unit.conversion_rate());
        let rhs_in_base_units = rhs.value * Value::Float(rhs.unit.conversion_rate());

        let value = lhs_in_base_units - rhs_in_base_units;

        println!("\nResult = {:#?}", value);

        // TODO: validate exponent?
        DimensionedValue {
            value: value,
            unit: Unit {
                dimension: self.unit.dimension - rhs.unit.dimension,
                exponent: self.unit.exponent,
                unit: self.unit.unit - rhs.unit.unit,
            },
        }
    }
}

impl Mul for DimensionedValue {
    type Output = DimensionedValue;

    fn mul(self, rhs: Self) -> Self {
        println!("\n\nMultiplying {:#?} with {:#?}", self, rhs);

        let lhs_in_base_units = self.value * Value::Float(self.unit.conversion_rate());
        let rhs_in_base_units = rhs.value * Value::Float(rhs.unit.conversion_rate());

        let value = lhs_in_base_units * rhs_in_base_units;

        println!("\nResult = {:#?}", value);

        DimensionedValue {
            value: value,
            unit: Unit {
                dimension: self.unit.dimension * rhs.unit.dimension,
                exponent: self.unit.exponent,
                unit: self.unit.unit * rhs.unit.unit,
            },
        }
    }
}

impl Div for DimensionedValue {
    type Output = DimensionedValue;

    fn div(self, rhs: Self) -> Self {
        println!("\n\nDividing {:#?} into {:#?}", self, rhs);

        let lhs_in_base_units = self.value * Value::Float(self.unit.conversion_rate());
        let rhs_in_base_units = rhs.value * Value::Float(rhs.unit.conversion_rate());

        let value = lhs_in_base_units / rhs_in_base_units;

        println!("\nResult = {:#?}", value);

        DimensionedValue {
            value: value,
            unit: Unit {
                dimension: self.unit.dimension / rhs.unit.dimension,
                exponent: self.unit.exponent,
                unit: self.unit.unit / rhs.unit.unit,
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
