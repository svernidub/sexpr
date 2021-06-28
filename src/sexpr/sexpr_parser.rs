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

    for token in sexpr.chars() {
        if token == '"' && !string_started {
            string_started = true;
            tokens_stack.push(token);
            continue
        } else if token == '"' && string_started {
            string_started = false;
            tokens_stack.push(token);
            continue
        } else if !is_end_terminator(token) {
            tokens_stack.push(token);
            continue
        } else if is_list_start_terminator(token) {
            tokens_stack.push(token);
            expr_stack.push(ElemStartTerminator);
            continue
        }

        let mut sexpr = String::from("");

        while let Some(prev_token) = tokens_stack.pop() {
            if is_start_terminator(prev_token) { break }

            sexpr = String::from(format!("{}{}", prev_token, sexpr));
        }

        expr_stack.push(expr_parser::parse_expr(sexpr)?);

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
            if let AstExpr(sexpr) = expr_stack.pop().unwrap() {
                Ok((*sexpr.clone()).clone())
            } else {
                Err("unexpected expression")
            }
        }
        _ => Err("unexpected expression")
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
    fn test_full_program() {
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