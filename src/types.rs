extern crate dimensioned as dim;
extern crate uom;

use std::collections::HashMap;
use std::ops::{Add, Div, Mul, Sub};

use uom::marker;
use uom::si::area::square_meter;
use uom::si::f64::{Area, Length, Mass, Time};
use uom::si::length::{kilometer, meter};
use uom::si::mass::kilogram;
use uom::si::time::second;
use uom::si::{Dimension, Quantity, SI};
use uom::typenum::{N1, P1};

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum BinaryOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

type Q<D, U> = Quantity<D, U, f64>;

fn scalar_vector_addition<D, U>(scalar: Q<D, U>, vector: Vec<Q<D, U>>) -> Vec<Q<D, U>>
where
    D: uom::si::Dimension + ?Sized,
    U: uom::si::Units<f64> + ?Sized,
    <D as Dimension>::Kind: marker::Add,
{
    vector.iter().map(|left_x| *left_x + scalar).collect()
}

fn scalar_vector_subtraction<D, U>(scalar: Q<D, U>, vector: Vec<Q<D, U>>) -> Vec<Q<D, U>>
where
    D: uom::si::Dimension + ?Sized,
    U: uom::si::Units<f64> + ?Sized,
    <D as Dimension>::Kind: marker::Sub,
{
    vector.iter().map(|left_x| *left_x - scalar).collect()
}

fn scalar_vector_multiplication<D1, D2, U>(
    scalar: Q<D1, U>,
    vector: Vec<Q<D2, U>>,
) -> Vec<Q<<D1 as Mul<D2>>::Output, U>>
where
    D1: Dimension + Mul<D2>,
    D2: Dimension,
    U: uom::si::Units<f64> + ?Sized,
    <D1 as Mul<D2>>::Output: Dimension,
    <D2 as Dimension>::L: std::ops::Add<<D1 as Dimension>::L>,
    <D2 as Dimension>::M: std::ops::Add<<D1 as Dimension>::M>,
    <D2 as Dimension>::T: std::ops::Add<<D1 as Dimension>::T>,
    <D2 as Dimension>::I: std::ops::Add<<D1 as Dimension>::I>,
    <D2 as Dimension>::Th: std::ops::Add<<D1 as Dimension>::Th>,
    <D2 as Dimension>::N: std::ops::Add<<D1 as Dimension>::N>,
    <D2 as Dimension>::J: std::ops::Add<<D1 as Dimension>::J>,
    <D2 as Dimension>::Kind: marker::Mul,
    <D1 as Dimension>::Kind: marker::Mul,
{
    vector.iter().map(|left_x| *left_x * scalar).collect()
}

fn scalar_vector_division<D1, D2, U>(
    scalar: Q<D1, U>,
    vector: Vec<Q<D2, U>>,
) -> Vec<Q<<D1 as Div<D2>>::Output, U>>
where
    D1: Dimension + Div<D2>,
    D2: Dimension,
    U: uom::si::Units<f64> + ?Sized,
    <D1 as Div<D2>>::Output: Dimension,
{
    vector.iter().map(|left_x| *left_x / scalar).collect()
}

fn elementwise_vector_addition<D, U>(vector1: Vec<Q<D, U>>, vector2: Vec<Q<D, U>>) -> Vec<Q<D, U>>
where
    D: uom::si::Dimension + ?Sized,
    U: uom::si::Units<f64> + ?Sized,
{
    vector1
        .iter()
        .zip(vector2)
        .map(|(left_x, right_x)| *left_x + right_x)
        .collect()
}

fn elementwise_vector_subtraction<D, U>(
    vector1: Vec<Q<D, U>>,
    vector2: Vec<Q<D, U>>,
) -> Vec<Q<D, U>>
where
    D: uom::si::Dimension + ?Sized,
    U: uom::si::Units<f64> + ?Sized,
{
    vector1
        .iter()
        .zip(vector2)
        .map(|(left_x, right_x)| *left_x - right_x)
        .collect()
}

fn elementwise_vector_multiplication<D1, D2, U>(
    vector1: Vec<Q<D1, U>>,
    vector2: Vec<Q<D2, U>>,
) -> Vec<Q<<D1 as Mul<D2>>::Output, U>>
where
    D1: Dimension + Mul<D2>,
    D2: Dimension,
    U: uom::si::Units<f64> + ?Sized,
    <D1 as Mul<D2>>::Output: Dimension,
{
    vector1
        .iter()
        .zip(vector2)
        .map(|(left_x, right_x)| *left_x * right_x)
        .collect()
}

fn elementwise_vector_division<D1, D2, U>(
    vector1: Vec<Q<D1, U>>,
    vector2: Vec<Q<D2, U>>,
) -> Vec<Q<<D1 as Div<D2>>::Output, U>>
where
    D1: Dimension + Div<D2>,
    D2: Dimension,
    U: uom::si::Units<f64> + ?Sized,
    <D1 as Div<D2>>::Output: Dimension,
{
    vector1
        .iter()
        .zip(vector2)
        .map(|(left_x, right_x)| *left_x / right_x)
        .collect()
}

pub enum Unit {
    Meter,
    Kilometer,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Length(Length),
    // Mass(Mass),
    // Time(Time),
    // Area(Area),
    VectorLength(Vec<Length>),
    // VectorMass(Vec<Mass>),
    // VectorTime(Vec<Time>),
    // VectorArea(Vec<Area>),
}

impl Value {
    pub fn new(value: f64, unit: Unit) -> Self {
        match unit {
            Unit::Meter => Self::Length(Length::new::<meter>(value)),
            Unit::Kilometer => Self::Length(Length::new::<kilometer>(value)),
            _ => panic!("Unsupported unit"),
        }
    }

    pub fn new_vec(values: Vec<f64>, unit: Unit) -> Self {
        match unit {
            Unit::Meter => Self::VectorLength(
                values
                    .into_iter()
                    .map(|v| Length::new::<meter>(v))
                    .collect(),
            ),
            Unit::Kilometer => Self::VectorLength(
                values
                    .into_iter()
                    .map(|v| Length::new::<kilometer>(v))
                    .collect(),
            ),
            _ => panic!("Unsupported unit"),
        }
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self {
        match self {
            // we are float
            Value::Length(lhs_value) => match rhs {
                // they are float
                Value::Length(rhs_value) => Value::Length(lhs_value + rhs_value),
                // they are vec
                Value::VectorLength(rhs_value) => {
                    Value::VectorLength(scalar_vector_addition(lhs_value, rhs_value))
                }
            },
            // we are vec
            Value::VectorLength(lhs_value) => match rhs {
                // they are float
                Value::Length(rhs_value) => {
                    Value::VectorLength(scalar_vector_addition(rhs_value, lhs_value))
                }
                // they are vec
                Value::VectorLength(rhs_value) => {
                    Value::VectorLength(elementwise_vector_addition(lhs_value, rhs_value))
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
            Value::Length(lhs_value) => match rhs {
                // they are float
                Value::Length(rhs_value) => Value::Length(lhs_value - rhs_value),
                // they are vec
                Value::VectorLength(rhs_value) => {
                    Value::VectorLength(scalar_vector_subtraction(lhs_value, rhs_value))
                }
            },
            // we are vec
            Value::VectorLength(lhs_value) => match rhs {
                // they are float
                Value::Length(rhs_value) => {
                    Value::VectorLength(scalar_vector_subtraction(rhs_value, lhs_value))
                }
                // they are vec
                Value::VectorLength(rhs_value) => {
                    Value::VectorLength(elementwise_vector_subtraction(lhs_value, rhs_value))
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
            Value::Length(lhs_value) => match rhs {
                // they are float
                Value::Length(rhs_value) => Value::Length(lhs_value * rhs_value),
                // they are vec
                Value::VectorLength(rhs_value) => {
                    Value::VectorLength(scalar_vector_multiplication(lhs_value, rhs_value))
                }
            },
            // we are vec
            Value::VectorLength(lhs_value) => match rhs {
                // they are float
                Value::Length(rhs_value) => {
                    Value::VectorLength(scalar_vector_multiplication(rhs_value, lhs_value))
                }
                // they are vec
                Value::VectorLength(rhs_value) => {
                    Value::VectorLength(elementwise_vector_multiplication(lhs_value, rhs_value))
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
            Value::Length(lhs_value) => match rhs {
                // they are float
                Value::Length(rhs_value) => Value::Length(lhs_value / rhs_value),
                // they are vec
                Value::VectorLength(rhs_value) => {
                    Value::VectorLength(scalar_vector_division(lhs_value, rhs_value))
                }
            },
            // we are vec
            Value::VectorLength(lhs_value) => match rhs {
                // they are float
                Value::Length(rhs_value) => {
                    Value::VectorLength(scalar_vector_division(rhs_value, lhs_value))
                }
                // they are vec
                Value::VectorLength(rhs_value) => {
                    Value::VectorLength(elementwise_vector_division(lhs_value, rhs_value))
                }
            },
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum AstNode {
    Print(Box<AstNode>),
    Double {
        value: f64,
        unit: Unit,
    },
    Vector {
        value: Vec<f64>,
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
