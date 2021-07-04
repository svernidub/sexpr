use super::sexpr_parser::{Expr, Expr::{BoolExpr, FuncExpr, NumberExpr, StringExpr}};
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

pub fn parse_expr<'a>(sexpr: String) -> Result<Expr, &'a str> {
    lazy_static! {
        static ref STRING: Regex = Regex::new(r#"^".*"$"#).unwrap();
        static ref INTEGER: Regex = Regex::new(r#"^\d+$"#).unwrap();
        static ref FLOAT: Regex = Regex::new(r#"^\d+\.\d+$"#).unwrap();
        static ref FUNC: Regex = Regex::new(r#"^.*$"#).unwrap();
        static ref TRUE: Regex = Regex::new("^true$").unwrap();
        static ref FALSE: Regex = Regex::new("^false$").unwrap();
    };

    let sexpr_ptr = sexpr.as_str();
    println!("Full expr: {:#?}", sexpr_ptr);

    if STRING.is_match(sexpr_ptr) {
        return Ok(StringExpr(sexpr[1..sexpr.len()-1].parse().unwrap()))
    }

    if INTEGER.is_match(sexpr_ptr) || FLOAT.is_match(sexpr_ptr) {
        return Ok(NumberExpr(f32::from_str(sexpr_ptr).unwrap()));
    }

    if TRUE.is_match(sexpr_ptr) {
        return Ok(BoolExpr(true));
    }

    if FALSE.is_match(sexpr_ptr) {
        return Ok(BoolExpr(false))
    }

    if FUNC.is_match(sexpr_ptr) {
        return Ok(FuncExpr(sexpr.clone()))
    }

    Err("nothing parsed")
}


#[cfg(test)]
mod test {
    use crate::sexpr::expr_parser::parse_expr;
    use crate::sexpr::sexpr_parser::Expr;

    #[test]
    fn test_parse_string() {
        let sexpr = String::from(r#""hello""#);
        let expr = parse_expr(sexpr).unwrap();
        let value = String::from("hello");

        if let Expr::StringExpr(parsed_value) = expr {
            assert_eq!(parsed_value, value)
        } else {
            panic!("Unexpected expression:\n{:#?}", expr)
        }
    }

    #[test]
    fn test_parse_string_with_escaped_quote() {
        let sexpr = String::from(r#"" abc "efg" ""#);
        let expr = parse_expr(sexpr).unwrap();
        let value = String::from(" abc \"efg\" ");

        if let Expr::StringExpr(parsed_value) = expr {
            assert_eq!(parsed_value, value)
        } else {
            panic!("Unexpected expression:\n{:#?}", expr)
        }
    }

    #[test]
    fn test_parse_int() {
        let sexpr = String::from("5");
        let expr = parse_expr(sexpr).unwrap();
        let value = 5.0;

        if let Expr::NumberExpr(parsed_value) = expr {
            assert_eq!(parsed_value, value)
        } else {
            panic!("Unexpected expression:\n{:#?}", expr)
        }
    }

    #[test]
    fn test_parse_float() {
        let sexpr = String::from("5.55");
        let expr = parse_expr(sexpr).unwrap();
        let value = 5.55;

        if let Expr::NumberExpr(parsed_value) = expr {
            assert_eq!(parsed_value, value)
        } else {
            panic!("Unexpected expression:\n{:#?}", expr)
        }
    }

    #[test]
    fn test_parse_false() {
        let sexpr = String::from("false");
        let expr = parse_expr(sexpr).unwrap();
        let value = false;

        if let Expr::BoolExpr(parsed_value) = expr {
            assert_eq!(parsed_value, value)
        } else {
            panic!("Unexpected expression:\n{:#?}", expr)
        }
    }


    #[test]
    fn test_parse_true() {
        let sexpr = String::from("true");
        let expr = parse_expr(sexpr).unwrap();
        let value = true;

        if let Expr::BoolExpr(parsed_value) = expr {
            assert_eq!(parsed_value, value)
        } else {
            panic!("Unexpected expression:\n{:#?}", expr)
        }
    }

    #[test]
    fn test_parse_func() {
        let sexpr = String::from("println");
        let expr = parse_expr(sexpr).unwrap();
        let value = String::from("println");

        if let Expr::FuncExpr(parsed_value) = expr {
            assert_eq!(parsed_value, value)
        } else {
            panic!("Unexpected expression:\n{:#?}", expr)
        }
    }
}
