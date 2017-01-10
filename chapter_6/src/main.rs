#[macro_use]
extern crate nom;

use nom::{IResult, digit};
use std::str::{FromStr, from_utf8};

#[derive (Debug, Clone, PartialEq)]
enum Operator { Add, Mul, Sub, Div }

#[derive (Debug, PartialEq)]
enum Expression {
    Number(usize),
    Expr(Operator, Vec<Expression>)
}

impl Expression {
    fn from_usize(u: usize) -> Result<Self, ()> {
        Ok(Expression::Number(u))
    }

    fn from_tuples(op: Operator, exprs: Vec<Self>) -> Result<Self, ()> {
        if exprs.is_empty() {
            Err(())
        } else {
            Ok(Expression::Expr(op, exprs))
        }
    }
}

impl Operator {
    fn from_char(c: char) -> Result<Self, ()> {
        match c {
            '+' => Ok(Operator::Add),
            '-' => Ok(Operator::Sub),
            '*' => Ok(Operator::Mul),
            '/' => Ok(Operator::Div),
            _ => Err(())
        }
    }
}

impl FromStr for Operator {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "+" => Ok(Operator::Add),
            "-" => Ok(Operator::Sub),
            "*" => Ok(Operator::Mul),
            "/" => Ok(Operator::Div),
            _ => Err(())
        }
    }
}

named!(
    operator_parser<Operator>,
    map_res!(
        one_of!("*+/-"),
        Operator::from_char)); 

named!(
    number_parser<Expression>,
    map_res!(
        map_res!(
            map_res!(
                ws!(digit),
                from_utf8),
            FromStr::from_str),
        Expression::from_usize));
    
named!(
    expression_parser<Expression>,
        alt!(
            number_parser | 
            map_res!(
                do_parse!(
                    operator: operator_parser >>
                    tag!(" ") >>
                    expression_vec: ws!(delimited!(tag!("(", many1!(expression_parser), tag!(")")))) >> 
                    (operator, expression_vec))
                Expression::from_tuple)));

named!(
    program_start_parser<Expression>,
    map_res!(
        do_parse!(
            operator: operator_parser >>
            tag!(" ") >>
            expression_vec: ws!(many1!(expression_parser)) >>
            (operator, expression_vec)),
        Expression::from_tuple));
            

fn main() {




}

fn output<I, O>(iresult: nom::IResult<I, O>) -> O {
    match iresult {
        IResult::Done(_, output) => output,
        _ => panic!("Not a Done")
    }
}

#[test]
fn operator_parser_test() {
    assert_eq!(output(operator_parser("+".as_bytes())), Operator::Add);
    assert_eq!(output(operator_parser("*".as_bytes())), Operator::Mul);
    assert_eq!(output(operator_parser("-".as_bytes())), Operator::Sub);
    assert_eq!(output(operator_parser("/".as_bytes())), Operator::Div);
}

#[test]
fn number_parser_test() {
    assert_eq!(output(number_parser("  1".as_bytes())), Expression::Number(1));
    assert_eq!(output(number_parser(" 10   ".as_bytes())), Expression::Number(10));
}

#[test]
fn expression_parser_test() {
   assert_eq!(output(expression_parser(" 1".as_bytes())), Expression::Number(1));
   assert_eq!(output(program_start_parser("+ 1 2 3".as_bytes())), Expression::Expr(Operator:: Plus, vec![Expression::Number(1), Expression::Number(2), Expression::Number(3)]));
   assert_eq!(output(program_start_parser("+ (* (- 7 1) 2 3) 2 (/ 3 3)".as_bytes())), 
              Expression::Expr(
                  Operator:: Plus,
                  vec![
                  Expression::Expr(
                      Operator::Mul, 
                      vec![
                      Expression::Expr(
                          Operator::Sub, 
                          vec![
                          Expression::Number(7), 
                          Expression::Number(1)]), 
                      Expression::Number(2),
                      Expression::Number(3)]),
                  Expression::Number(2),
                  Expression::Expr(
                      Operator::Div,
                      vec![
                      Expression::Num(3),
                      Expression::Num(3)])]));

                 
}
