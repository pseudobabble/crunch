extern crate nom;
extern crate uom;

use uom::si::f64::*;
use uom::si::information::{bit, Information};
use uom::si::length::{kilometer, meter, Length};
use uom::si::time::{day, hour, second, Time};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::{alpha1, char, space0};
use nom::multi::{many0, many1};
use nom::number::complete::double;
use nom::sequence::{delimited, preceded, terminated};
use nom::IResult;

use super::types::*;

fn parse_unit(input: &str) -> IResult<&str, Unit> {
    println!("reached parse_unit {}", input.clone());

    // TODO: none of this is very nice, differentiate unit families better

    // https://docs.rs/nom/latest/nom/branch/fn.alt.html
    println!("  parsing unit {}", input.clone());
    let (input, _) = tag("[")(input)?;
    // println!("  parsing unit {}", input.clone());
    let (input, unit_alias) = alt((
        tag("meters"),
        tag("meter"),
        tag("m"), // longest to shortest!!
        tag("kilometers"),
        tag("kilometer"),
        tag("km"),
    ))(input)?;
    println!("  parsed unit {}", unit_alias.clone());
    let (input, _) = tag("^")(input)?;

    // TODO: add some sugar here so we can write 1[m] instead of 1[m^1]
    // println!("  parsing unit {}", input.clone());
    let (input, power_string) = digit1(input)?;
    println!("  parsed power {}", power_string.clone());

    let power = power_string.parse::<i64>().unwrap();
    // println!("  parsing unit {}", input.clone());
    let (input, _) = tag("]")(input)?;
    // println!("  parsing unit {}", input.clone());

    // TODO: We can also have a parser for each unit
    // TODO: turn these into quantities in the interpreter
    let dimension = match unit_alias {
        "meters" | "meter" | "m" => match power {
            1 => Unit::Meter(1),
            2 => Unit::Meter(2),
            3 => Unit::Meter(3),
            _ => todo!("other dimensions in meters"),
        },
        "kilometers" | "kilometer" | "km" => match power {
            1 => Unit::Kilometer(1),
            2 => Unit::Kilometer(2),
            3 => Unit::Kilometer(3),
            _ => todo!("other dimensions in kilometers"),
        },
        _ => panic!("Unsupported unit alias {}", unit_alias),
    };

    Ok((input, dimension))
}

/// Switch on dimensions
fn parse_dimension(input: &str) -> IResult<&str, Unit> {
    println!("reached parse_dimension {}", input.clone());
    let (input, dimension) = parse_unit(input)?;
    // let (input, dimension) = delimited(tag("["), alt((parse_unit, parse_volume)), tag("]"))(input)?;

    Ok((input, dimension))
}

fn parse_number(number: &str) -> IResult<&str, AstNode> {
    println!("reached parse_number {}", number.clone());
    let (input, number) = double(number)?;

    let (input, unit) = parse_dimension(input)?;

    Ok((
        input,
        AstNode::Double {
            value: Value::Float(number),
            unit: unit,
        },
    ))
}

fn parse_vector(input: &str) -> IResult<&str, AstNode> {
    println!("reached parse_vector {}", input.clone());

    println!("  reached vector bracket open {}", input.clone());
    let (input, _) = tag("[")(input)?;
    println!("  reached vector {}", input.clone());
    let (input, vector) = many1(delimited(space0, double, space0))(input)?;
    println!("  parsed vector {:#?}", vector.clone());
    println!("  reached vector bracket close {}", input.clone());
    let (input, _) = tag("]")(input)?;

    let (input, unit) = parse_dimension(input)?;

    Ok((
        input,
        AstNode::Vector {
            value: Value::Vec(vector),
            unit: unit,
        },
    ))
}

fn parse_value(input: &str) -> IResult<&str, AstNode> {
    println!("reached parse_value {}", input.clone());
    alt((parse_vector, parse_number))(input)
}

fn parse_name(name: &str) -> IResult<&str, AstNode> {
    println!("reached parse_name {}", name.clone());
    let (input, name) = alpha1(name)?;

    Ok((input, AstNode::Name(name.to_string())))
}

fn parse_operator(input: &str) -> IResult<&str, &str> {
    println!("reached parse_operator {}", input.clone());
    Ok(alt((
        terminated(preceded(space0, tag("+")), space0),
        terminated(preceded(space0, tag("-")), space0),
        terminated(preceded(space0, tag("*")), space0),
        terminated(preceded(space0, tag("/")), space0),
        terminated(preceded(space0, tag("^")), space0),
    ))(input)?)
}

fn parse_expression(input: &str) -> IResult<&str, AstNode> {
    println!("reached parse_expression {}", input.clone());

    let (input, _) = tag("(")(input)?;
    let (input, lhs) = alt((parse_value, parse_name, parse_expression))(input)?;
    let (input, operator) = parse_operator(input)?;
    let (input, rhs) = alt((parse_expression, parse_name, parse_value))(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((
        input,
        AstNode::Expression {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
            operation: match operator {
                "+" => BinaryOperation::Add,
                "-" => BinaryOperation::Subtract,
                "*" => BinaryOperation::Multiply,
                "/" => BinaryOperation::Divide,
                _ => panic!("Unsupported binary operation {}", operator),
            },
        },
    ))
}

fn parse_variable(input: &str) -> IResult<&str, AstNode> {
    println!("reached parse_variable {}", input.clone());
    let (input, name) = parse_name(input)?;
    let (input, _) = tag(" = ")(input)?;
    let (input, expr) = terminated(alt((parse_value, parse_expression)), char(';'))(input)?;

    Ok((
        input,
        AstNode::Variable {
            name: Box::new(name),
            expr: Box::new(expr),
        },
    ))
}

pub fn parse_line(input: &str) -> IResult<&str, Vec<AstNode>> {
    println!("reached parse_line {}", input.clone());
    many0(preceded(space0, parse_variable))(input)
}

#[test]
fn test_parse_number() {
    assert_eq!(
        parse_number("11e-1[m]"),
        Ok((
            "",
            AstNode::Double {
                value: Value::Float(1.1),
                dimension: Unit {
                    unit: Unit {
                        unit: UnitIdentity::Meter,
                        conversion_factor: 1.0
                    },
                    power: 1
                }
            }
        ))
    );
    assert_eq!(
        parse_number("1[meter]"),
        Ok((
            "",
            AstNode::Double {
                value: Value::Float(1.0),
                dimension: Unit {
                    unit: Unit {
                        unit: UnitIdentity::Meter,
                        conversion_factor: 1.0
                    },
                    power: 1
                },
            }
        ))
    );
    assert_eq!(
        parse_number("1.1[km]"),
        Ok((
            "",
            AstNode::Double {
                value: Value::Float(1.1),
                dimension: Unit {
                    unit: UnitIdentity::Meter,
                    conversion_factor: 1.0
                },
                power: 1
            }
        ))
    );
    assert_eq!(
        parse_number("9999999.987654[m]"),
        Ok((
            "",
            AstNode::Double {
                value: Value::Float(9999999.987654),
                dimension: Unit {
                    unit: UnitIdentity::Meter,
                    conversion_factor: 1.0
                },
                power: 1
            }
        ))
    );
}
#[test]
fn test_parse_name() {
    assert_eq!(
        parse_name("test"),
        Ok(("", AstNode::Name("test".to_string())))
    );
    assert_eq!(
        parse_name("test"),
        Ok(("", AstNode::Name("test".to_string())))
    );
}

#[test]
fn test_parse_variable() {
    assert_eq!(
        parse_variable("test = 1.2[m];"),
        Ok((
            "",
            AstNode::Variable {
                name: Box::new(AstNode::Name("test".to_string())),
                expr: Box::new(AstNode::Double {
                    value: Value::Float(1.2),
                    dimension: Unit { unit: Unit::Meter }
                })
            }
        ))
    );

    assert_eq!(
        parse_variable("var = -2[kilometers];"),
        Ok((
            "",
            AstNode::Variable {
                name: Box::new(AstNode::Name("var".to_string())),
                expr: Box::new(AstNode::Double {
                    value: Value::Float(-2.0),
                    dimension: Unit {
                        unit: Unit::Kilometer
                    }
                })
            }
        ))
    );
}

#[test]
fn test_parse_expression() {
    assert_eq!(
        parse_expression("(2[km] / 2[m])"),
        Ok((
            "",
            AstNode::Expression {
                operation: BinaryOperation::Divide,
                lhs: Box::new(AstNode::Double {
                    value: Value::Float(2.0),
                    dimension: Unit {
                        unit: Unit::Kilometer
                    }
                }),
                rhs: Box::new(AstNode::Double {
                    value: Value::Float(2.0),
                    dimension: Unit { unit: Unit::Meter }
                })
            }
        ))
    );

    assert_eq!(
        parse_expression("((2[m] / 2[km]) + (4[km] * 4[m]))"),
        Ok((
            "",
            AstNode::Expression {
                operation: BinaryOperation::Add,
                lhs: Box::new(AstNode::Expression {
                    operation: BinaryOperation::Divide,
                    lhs: Box::new(AstNode::Double {
                        value: Value::Float(2.0),
                        dimension: Unit { unit: Unit::Meter }
                    }),
                    rhs: Box::new(AstNode::Double {
                        value: Value::Float(2.0),
                        dimension: Unit {
                            unit: Unit::Kilometer
                        }
                    })
                }),
                rhs: Box::new(AstNode::Expression {
                    operation: BinaryOperation::Multiply,
                    lhs: Box::new(AstNode::Double {
                        value: Value::Float(4.0),
                        dimension: Unit {
                            unit: Unit::Kilometer
                        }
                    }),
                    rhs: Box::new(AstNode::Double {
                        value: Value::Float(4.0),
                        dimension: Unit { unit: Unit::Meter }
                    })
                })
            }
        ))
    );
}

#[test]
fn parse_variable_expression() {
    assert_eq!(
        parse_variable("var = (2[m] / 2[km]);"),
        Ok((
            "",
            AstNode::Variable {
                name: Box::new(AstNode::Name("var".to_string())),
                expr: Box::new(AstNode::Expression {
                    operation: BinaryOperation::Divide,
                    lhs: Box::new(AstNode::Double {
                        value: Value::Float(2.0),
                        dimension: Unit { unit: Unit::Meter }
                    }),
                    rhs: Box::new(AstNode::Double {
                        value: Value::Float(2.0),
                        dimension: Unit {
                            unit: Unit::Kilometer
                        }
                    })
                })
            }
        ))
    );

    assert_eq!(
        parse_variable("var = ((2[m] * 3[kilometers]) * (4[meters] + 5[km]));"),
        Ok((
            "",
            AstNode::Variable {
                name: Box::new(AstNode::Name("var".to_string())),
                expr: Box::new(AstNode::Expression {
                    operation: BinaryOperation::Multiply,
                    lhs: Box::new(AstNode::Expression {
                        operation: BinaryOperation::Multiply,
                        lhs: Box::new(AstNode::Double {
                            value: Value::Float(2.0),
                            dimension: Unit { unit: Unit::Meter }
                        }),
                        rhs: Box::new(AstNode::Double {
                            value: Value::Float(3.0),
                            dimension: Unit {
                                unit: Unit::Kilometer
                            }
                        }),
                    }),
                    rhs: Box::new(AstNode::Expression {
                        operation: BinaryOperation::Add,
                        lhs: Box::new(AstNode::Double {
                            value: Value::Float(4.0),
                            dimension: Unit { unit: Unit::Meter }
                        }),
                        rhs: Box::new(AstNode::Double {
                            value: Value::Float(5.0),
                            dimension: Unit {
                                unit: Unit::Kilometer
                            }
                        }),
                    })
                })
            }
        ))
    );
}

#[test]
fn parse_variables_and_abstract_expressions() {
    assert_eq!(
        parse_line("x = (2[m] * 2[kilometer]); y = 1[km]; z = (x + y);"),
        Ok((
            "",
            vec![
                AstNode::Variable {
                    name: Box::new(AstNode::Name("x".to_string())),
                    expr: Box::new(AstNode::Expression {
                        operation: BinaryOperation::Multiply,
                        lhs: Box::new(AstNode::Double {
                            value: Value::Float(2.0),
                            dimension: Unit { unit: Unit::Meter }
                        }),
                        rhs: Box::new(AstNode::Double {
                            value: Value::Float(2.0),
                            dimension: Unit {
                                unit: Unit::Kilometer
                            }
                        })
                    })
                },
                AstNode::Variable {
                    name: Box::new(AstNode::Name("y".to_string())),
                    expr: Box::new(AstNode::Double {
                        value: Value::Float(1.0),
                        dimension: Unit {
                            unit: Unit::Kilometer
                        }
                    })
                },
                AstNode::Variable {
                    name: Box::new(AstNode::Name("z".to_string())),
                    expr: Box::new(AstNode::Expression {
                        operation: BinaryOperation::Add,
                        lhs: Box::new(AstNode::Name("x".to_string())),
                        rhs: Box::new(AstNode::Name("y".to_string()))
                    })
                },
            ]
        ))
    );
}
