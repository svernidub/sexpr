use std::rc::Rc;

use crate::sexpr::{expr_parser, parse_to_ast};
use crate::sexpr::expr::SExpr;
use crate::sexpr::sexpr_parser::Expr::*;

#[derive(Debug, Clone)]
pub enum Expr {
    ElemStartTerminator,
    NumberExpr(f32),
    StringExpr(String),
    FuncExpr(String),
    BoolExpr(bool),
    AstExpr(Rc<SExpr>)
}

pub fn parse<'a>(sexpr: String) -> Result<SExpr, &'a str> {
    let mut tokens_stack: Vec<char> = vec!();
    let mut expr_stack: Vec<Expr> = vec!();

    let mut string_started = false;
    let mut prev_string_token = None;

    const ESCAPE_TOKEN: char = '\\';

    for token in sexpr.chars() {
        if token == '"' {
            if let Some(prev) = prev_string_token {
                if prev == ESCAPE_TOKEN {
                    tokens_stack.push(token);
                    prev_string_token = Some(token);
                    continue
                }
            }

            string_started = !string_started;

            if string_started {
                prev_string_token = Some(token);
            } else {
                prev_string_token = None;
            }
        }

        if string_started {
            tokens_stack.push(token);
            prev_string_token = Some(token);
            continue
        }

        if !is_end_terminator(token) {
            tokens_stack.push(token);
            continue
        } else if is_list_start_terminator(token) {
            tokens_stack.push(token);
            expr_stack.push(ElemStartTerminator);
            continue
        }

        let mut sexpr = String::from("");

        while let Some(prev_token) = tokens_stack.pop() {
            if prev_token == '"' {
                string_started = !string_started;
            }

            if !string_started && is_start_terminator(prev_token) { break }

            sexpr = String::from(format!("{}{}", prev_token, sexpr));
        }

        let parsed_expr = expr_parser::parse_expr(sexpr)?;
        println!("{:#?}", parsed_expr);
        expr_stack.push(parsed_expr);

        if !is_list_end_terminator(token) { continue }

        let mut exprs = vec!();
        while let Some(expr) = expr_stack.pop() {
            if let ElemStartTerminator = expr { break }

            exprs.push(expr);
        }
        exprs.reverse();

        expr_stack.push(AstExpr(Rc::new(parse_to_ast::parse(exprs)?)));
    }

    match expr_stack.len() {
        0 => Err("input is empty?"),
        1 => {
            let rest = expr_stack.pop().unwrap();
            if let AstExpr(sexpr) = rest {
                Ok((*sexpr.clone()).clone())
            } else {
                Err("Expected to have only one expression which will be AST")
            }
        }
        _ => Err("Closing bracket missing?")
    }
}

fn is_end_terminator(token: char) -> bool {
    token == ' ' || token == ')' || token == '\n'
}

fn is_start_terminator(token: char) -> bool {
    token == ' ' || token == '(' || token == '\n'
}

fn is_list_end_terminator(token: char) -> bool {
    token == ')'
}

fn is_list_start_terminator(token: char) -> bool {
    token == '('
}

#[cfg(test)]
mod test {
    use std::rc::Rc;

    use super::parse;
    use super::super::expr::SExpr::*;

    #[test]
    fn test_empty_list() {
        let sexpr = String::from("()");
        let expected_expr = format!("{:#?}", SList(vec!()));
        let fact_expr = format!("{:#?}", parse(sexpr).unwrap());

        assert_eq!(fact_expr, expected_expr)
    }

    #[test]
    fn test_boolean_func() {
        let sexpr = String::from(r#"(and true false)"#);

        let expected_expr = format!("{:#?}", SFunc(
            String::from("and"),
            vec![Rc::new(SBool(true)), Rc::new(SBool(false))]
        ));

        let fact_expr = format!("{:#?}", parse(sexpr).unwrap());
        assert_eq!(fact_expr, expected_expr)
    }

    #[test]
    fn test_numeric_func_with_ints() {
        let sexpr = String::from(r#"(add 5 4)"#);

        let expected_expr = format!("{:#?}", SFunc(
            String::from("add"),
            vec![Rc::new(SNumber(5.0)), Rc::new(SNumber(4.0))]
        ));

        let fact_expr = format!("{:#?}", parse(sexpr).unwrap());
        assert_eq!(fact_expr, expected_expr)
    }

    #[test]
    fn test_numeric_func_with_floats() {
        let sexpr = String::from(r#"(add 5.5 4.5)"#);

        let expected_expr = format!("{:#?}", SFunc(
            String::from("add"),
            vec![Rc::new(SNumber(5.5)), Rc::new(SNumber(4.5))]
        ));

        let fact_expr = format!("{:#?}", parse(sexpr).unwrap());
        assert_eq!(fact_expr, expected_expr)
    }

    #[test]
    fn test_string_func() {
        let sexpr = String::from(r#"(concat "hello" "_" "world")"#);

        let expected_expr = format!("{:#?}", SFunc(
            String::from("concat"),
            vec![
                Rc::new(SString(String::from("hello"))),
                Rc::new(SString(String::from("_"))),
                Rc::new(SString(String::from("world")))
            ]
        ));

        let fact_expr = format!("{:#?}", parse(sexpr).unwrap());
        assert_eq!(fact_expr, expected_expr)
    }

    #[test]
    fn test_func_with_whitespace_arg() {
        let sexpr = String::from(r#"(concat "hello" " " "world")"#);

        let expected_expr = format!("{:#?}", SFunc(
            String::from("concat"),
            vec![
                Rc::new(SString(String::from("hello"))),
                Rc::new(SString(String::from(" "))),
                Rc::new(SString(String::from("world")))
            ]
        ));

        let fact_expr = format!("{:#?}", parse(sexpr).unwrap());
        assert_eq!(fact_expr, expected_expr)
    }

    #[test]
    fn test_func_with_escaped_quote_arg() {
        let sexpr = String::from(r#"(concat "hello" "\"" "world")"#);

        let expected_expr = format!("{:#?}", SFunc(
            String::from("concat"),
            vec![
                Rc::new(SString(String::from("hello"))),
                Rc::new(SString(String::from("\""))),
                Rc::new(SString(String::from("world")))
            ]
        ));

        let fact_expr = format!("{:#?}", parse(sexpr).unwrap());
        assert_eq!(fact_expr, expected_expr)
    }

    #[test]
    fn test_complex_program() {
        let sexpr = String::from(r#"(
            (if (> 7 3)
                (concat "hello" " " "world")
                (add 5 3)
            )

            (div 6.3 2.7)
        )"#);

        let expected_expr = format!("{:#?}",
            SList(vec!(
                Rc::new(SFunc(String::from("if"), vec!(
                    Rc::new(SFunc(String::from("concat"), vec!(
                        Rc::new(SString(String::from("hello"))),
                        Rc::new(SString(String::from(" "))),
                        Rc::new(SString(String::from("world"))),
                    ))),
                    Rc::new(SFunc(String::from("add"), vec!(
                        Rc::new(SNumber(5.0)),
                        Rc::new(SNumber(3.0))
                    )))
                ))),

                Rc::new(SFunc(String::from("div"), vec!(
                    Rc::new(SNumber(6.3)),
                    Rc::new(SNumber(2.7))
                )))
            ))
        );

        let fact_expr = format!("{:#?}", parse(sexpr).unwrap());
        assert_eq!(fact_expr, expected_expr)
    }
}