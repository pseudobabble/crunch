use std::mem::discriminant;
use std::ops::{Add, Div, Mul, Sub};

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum BinaryOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

/// The f64 type is the conversion factor to base units
#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum UnitIdentity {
    CompoundUnit {
        operation: BinaryOperation,
        lhs: Box<UnitIdentity>,
        rhs: Box<UnitIdentity>,
    },
    None(f64),
    Second(f64),
    Minute(f64), // Minute(60): x[m] * 60 == x[s]
    Hour(f64),
    Day(f64),
    Meter(f64),
    Kilometer(f64),
    SquareMeter(f64),
    SquareKilometer(f64),
    CubicMeter(f64),
    CubicKilometer(f64),
    USD(f64),
    GBP(f64),
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Quantity {
    CompoundQuantity {
        operation: BinaryOperation,
        lhs: Box<Quantity>,
        rhs: Box<Quantity>,
    },
    None(i64),
    Length(i64),
    //    Mass(i64),
    Time(i64),
    Currency(i64),
}

impl Quantity {
    fn get_base_unit(&self) -> UnitIdentity {
        match self {
            Quantity::None(power) => UnitIdentity::None(1.0),
            Quantity::Time(power) => UnitIdentity::Second(1.0),
            Quantity::Length(power) => match power {
                1 => UnitIdentity::Meter(1.0),
                2 => UnitIdentity::SquareMeter(1.0),
                3 => UnitIdentity::CubicMeter(1.0),
                _ => todo!("generic power"),
            },
            Quantity::Currency(power) => UnitIdentity::USD(1.0),
            Quantity::CompoundQuantity {
                operation,
                lhs,
                rhs,
            } => UnitIdentity::CompoundUnit {
                operation: operation.clone(),
                lhs: Box::new(lhs.get_base_unit()),
                rhs: Box::new(rhs.get_base_unit()),
            },
        }
    }

    fn power(self) -> i64 {
        match self {
            Quantity::None(power) => power,
            Quantity::Time(power) => power,
            Quantity::Length(power) => power,
            Quantity::Currency(power) => power,
            Quantity::CompoundQuantity { .. } => panic!("CompoundQuantity has no power"),
        }
    }
}

impl Add for Quantity {
    type Output = Quantity;

    /// we can only add when the units are identical
    /// and the addition returns the same unit:
    /// 1[m^1] + 1[m^1] == 2[m^1]
    /// 1[m^1] + 1[m^2] -> invalid!
    fn add(self, rhs: Self) -> Self {
        // if simple, use, otherwise simplify
        if discriminant(&self) == discriminant(&rhs) {
            // compounds cant be simplified
            if let Quantity::CompoundQuantity { .. } = self {
                Quantity::CompoundQuantity {
                    operation: BinaryOperation::Add,
                    lhs: Box::new(self),
                    rhs: Box::new(rhs),
                }
            } else {
                if self.clone().power() == rhs.clone().power() {
                    self
                } else {
                    panic!(
                        "Cannot add {:#?} with dimension {} from {:#?} with dimension {}.",
                        rhs.clone(),
                        rhs.power(),
                        self.clone(),
                        self.power()
                    )
                }
            }
        } else {
            panic!("Cannot add {:#?} from {:#?}.", rhs, self)
        }
    }
}

impl Sub for Quantity {
    type Output = Quantity;

    /// we can only subtract when the units are identical
    /// and the subtraction returns the same unit:
    /// 1[m^1] - 1[m^1] == 0[m^1]
    /// 1[m^1] - 1[m^2] -> invalid!
    fn sub(self, rhs: Self) -> Self {
        // if they are the same variant
        if discriminant(&self) == discriminant(&rhs) {
            // compounds cant be simplified
            if let Quantity::CompoundQuantity { .. } = self {
                Quantity::CompoundQuantity {
                    operation: BinaryOperation::Subtract,
                    lhs: Box::new(self),
                    rhs: Box::new(rhs),
                }
            } else {
                if self.clone().power() == rhs.clone().power() {
                    self
                } else {
                    panic!(
                        "Cannot subtract {:#?} with dimension {} from {:#?} with dimension {}.",
                        rhs.clone(),
                        rhs.power(),
                        self.clone(),
                        self.power()
                    )
                }
            }
        } else {
            panic!("Cannot subtract {:#?} from {:#?}.", rhs, self)
        }
    }
}

impl Mul for Quantity {
    type Output = Quantity;

    /// 1[m^1] * 1[m^2] = 1[m^3]
    /// a^1 * a^2 = a^3
    fn mul(self, rhs: Self) -> Self {
        println!("Multiplying {:#?} with {:#?}", self.clone(), rhs.clone());

        // if simple and type match, add powers
        // otherwise compound
        match self {
            Quantity::None(power) => rhs,
            Quantity::Time(lhs_power) => match rhs {
                // if self == rhs
                // note using lhs power in type comparison to check type and power at same time
                Quantity::Time(rhs_power) => Quantity::Time(lhs_power + rhs_power),
                Quantity::None(power) => self,
                _ => Quantity::CompoundQuantity {
                    operation: BinaryOperation::Multiply,
                    lhs: Box::new(self),
                    rhs: Box::new(rhs),
                },
            },
            Quantity::Length(lhs_power) => match rhs {
                Quantity::Length(rhs_power) => Quantity::Length(lhs_power + rhs_power),
                Quantity::None(power) => self,
                _ => Quantity::CompoundQuantity {
                    operation: BinaryOperation::Multiply,
                    lhs: Box::new(self),
                    rhs: Box::new(rhs),
                },
            },
            Quantity::Currency(lhs_power) => match rhs {
                Quantity::Currency(rhs_power) => Quantity::Currency(lhs_power + rhs_power),
                Quantity::None(power) => self,
                _ => Quantity::CompoundQuantity {
                    operation: BinaryOperation::Multiply,
                    lhs: Box::new(self),
                    rhs: Box::new(rhs),
                },
            },
            Quantity::CompoundQuantity { .. } => Quantity::CompoundQuantity {
                operation: BinaryOperation::Multiply,
                lhs: Box::new(self),
                rhs: Box::new(rhs),
            },
        }
    }
}

impl Div for Quantity {
    type Output = Quantity;

    /// 1[m^1] * 1[m^2] = 1[m^3]
    /// a^1 * a^2 = a^3
    fn div(self, rhs: Self) -> Self {
        println!("Multiplying {:#?} with {:#?}", self.clone(), rhs.clone());
        // if simple, use, otherwise simplify
        match self {
            Quantity::None(power) => rhs,
            Quantity::Time(lhs_power) => match rhs {
                // if self == rhs
                // note using lhs power in type comparison to check type and power at same time
                Quantity::Time(rhs_power) => Quantity::Time(lhs_power - rhs_power),
                Quantity::None(power) => self,
                _ => Quantity::CompoundQuantity {
                    operation: BinaryOperation::Divide,
                    lhs: Box::new(self),
                    rhs: Box::new(rhs),
                },
            },
            Quantity::Length(lhs_power) => match rhs {
                Quantity::Length(rhs_power) => Quantity::Length(lhs_power - rhs_power),
                Quantity::None(power) => self,
                _ => Quantity::CompoundQuantity {
                    operation: BinaryOperation::Divide,
                    lhs: Box::new(self),
                    rhs: Box::new(rhs),
                },
            },
            Quantity::Currency(lhs_power) => match rhs {
                Quantity::Currency(rhs_power) => Quantity::Currency(lhs_power - rhs_power),
                Quantity::None(power) => self,
                _ => Quantity::CompoundQuantity {
                    operation: BinaryOperation::Divide,
                    lhs: Box::new(self),
                    rhs: Box::new(rhs),
                },
            },
            Quantity::CompoundQuantity { .. } => Quantity::CompoundQuantity {
                operation: BinaryOperation::Divide,
                lhs: Box::new(self),
                rhs: Box::new(rhs),
            },
        }
    }
}

/// unit contains conversion factor
/// quantity contains power
#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct Unit {
    pub unit: UnitIdentity,
    pub quantity: Quantity,
}

impl Unit {
    fn get_conversion_factor(self) -> f64 {
        match self.clone().unit {
            UnitIdentity::None(factor) => factor,
            UnitIdentity::Second(factor) => factor,
            UnitIdentity::Minute(factor) => factor,
            UnitIdentity::Hour(factor) => factor,
            UnitIdentity::Day(factor) => factor,
            UnitIdentity::Meter(factor) => factor,
            UnitIdentity::Kilometer(factor) => factor,
            UnitIdentity::SquareMeter(factor) => factor,
            UnitIdentity::SquareKilometer(factor) => factor,
            UnitIdentity::CubicMeter(factor) => factor,
            UnitIdentity::CubicKilometer(factor) => factor,
            UnitIdentity::USD(factor) => factor,
            UnitIdentity::GBP(factor) => factor,
            UnitIdentity::CompoundUnit { .. } => 1.0,
        }
    }
}

impl Add for Unit {
    type Output = Unit;

    fn add(self, rhs: Self) -> Self {
        let derived_quantity = self.quantity + rhs.quantity;
        let unit = derived_quantity.get_base_unit();
        Unit {
            unit: unit,
            quantity: derived_quantity,
        }
    }
}

impl Sub for Unit {
    type Output = Unit;

    fn sub(self, rhs: Self) -> Self {
        let derived_quantity = self.quantity - rhs.quantity;
        let unit = derived_quantity.get_base_unit();
        Unit {
            unit: unit,
            quantity: derived_quantity,
        }
    }
}

impl Mul for Unit {
    type Output = Unit;

    /// 1[m^1] * 1[m^2] = 1[m^3]
    /// a^1 * a^2 = a^3
    fn mul(self, rhs: Self) -> Self {
        let derived_quantity = self.quantity * rhs.quantity;
        let unit = derived_quantity.get_base_unit();
        Unit {
            unit: unit,
            quantity: derived_quantity,
        }
    }
}

impl Div for Unit {
    type Output = Unit;

    fn div(self, rhs: Self) -> Self {
        let derived_quantity = self.quantity / rhs.quantity;
        println!("Div for Unit: derived_quantity = {:#?}", derived_quantity);
        let unit = derived_quantity.get_base_unit();
        Unit {
            unit: unit,
            quantity: derived_quantity,
        }
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

        // the conversion to base units MUST happen here
        // at the outmost scope, so we dont have to write match arms for every
        // downstream Mul, Add, etc
        let lhs_value_in_base_units =
            self.value * Value::Float(self.unit.clone().get_conversion_factor());
        let rhs_value_in_base_units =
            rhs.value * Value::Float(rhs.unit.clone().get_conversion_factor());

        let unit = self.unit + rhs.unit;
        let value = lhs_value_in_base_units + rhs_value_in_base_units;

        println!("\nResult = {:#?}", value);

        DimensionedValue {
            value: value,
            unit: unit,
        }
    }
}

impl Sub for DimensionedValue {
    type Output = DimensionedValue;

    fn sub(self, rhs: Self) -> Self {
        println!("\n\nSubtracting {:#?} from {:#?}", rhs, self);

        // the conversion to base units MUST happen here
        // at the outmost scope, so we dont have to write match arms for every
        // downstream Mul, Add, etc
        let lhs_value_in_base_units =
            self.value * Value::Float(self.unit.clone().get_conversion_factor());
        let rhs_value_in_base_units =
            rhs.value * Value::Float(rhs.unit.clone().get_conversion_factor());

        let unit = self.unit - rhs.unit;
        let value = lhs_value_in_base_units - rhs_value_in_base_units;

        println!("\nResult = {:#?}", value);

        DimensionedValue {
            value: value,
            unit: unit,
        }
    }
}

impl Mul for DimensionedValue {
    type Output = DimensionedValue;

    fn mul(self, rhs: Self) -> Self {
        println!("\n\nMultiplying {:#?} with {:#?}", self, rhs);

        // the conversion to base units MUST happen here
        // at the outmost scope, so we dont have to write match arms for every
        // downstream Mul, Add, etc
        let lhs_value_in_base_units =
            self.value * Value::Float(self.unit.clone().get_conversion_factor());
        let rhs_value_in_base_units =
            rhs.value * Value::Float(rhs.unit.clone().get_conversion_factor());

        let unit = self.unit * rhs.unit;
        let value = lhs_value_in_base_units * rhs_value_in_base_units;

        println!("\nResult = {:#?}", value);

        DimensionedValue {
            value: value,
            unit: unit,
        }
    }
}

impl Div for DimensionedValue {
    type Output = DimensionedValue;

    fn div(self, rhs: Self) -> Self {
        println!("\n\nDividing {:#?} into {:#?}", self, rhs);

        // the conversion to base units MUST happen here
        // at the outmost scope, so we dont have to write match arms for every
        // downstream Mul, Add, etc
        let lhs_value_in_base_units =
            self.value * Value::Float(self.unit.clone().get_conversion_factor());
        let rhs_value_in_base_units =
            rhs.value * Value::Float(rhs.unit.clone().get_conversion_factor());

        println!(
            "\n\nLHS in base units {:#?} / RHS in base units {:#?}",
            lhs_value_in_base_units.clone(),
            rhs_value_in_base_units.clone()
        );

        let unit = self.unit / rhs.unit;
        let value = lhs_value_in_base_units / rhs_value_in_base_units;

        println!("\nResult = {:#?}", value);

        DimensionedValue {
            value: value,
            unit: unit,
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
