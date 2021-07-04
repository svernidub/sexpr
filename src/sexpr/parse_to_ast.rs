use crate::sexpr::expr::SExpr;
use crate::sexpr::sexpr_parser::{Expr, Expr::*};
use std::rc::Rc;
use crate::sexpr::expr::SExpr::*;

pub fn parse<'a>(expressions: Vec<Expr>) -> Result<SExpr, &'a str> {
    match expressions[0].clone() {
        FuncExpr(function_name) =>  {
            let args = transform_expressions(expressions[1..].to_owned())?;
            Ok(SFunc(function_name.clone(), args))
        },
        AstExpr(_) => {
            Ok(SList(transform_expressions(expressions.to_owned())?))
        },
        _ => Err("Can't parse expressions")
    }
}

fn transform_expressions<'a>(expressions: Vec<Expr>) -> Result<Vec<Rc<SExpr>>, &'a str> {
    let sexprs = expressions.iter().map(|expr| {
        match expr {
            NumberExpr(n) => Rc::new(SNumber(n.clone())),
            StringExpr(s) => Rc::new(SString(s.clone())),
            FuncExpr(fname) => Rc::new(SFunc(fname.clone(), vec!())),
            BoolExpr(b) => Rc::new(SBool(b.clone())),
            AstExpr(ast) => ast.clone(),
            _ => unreachable!("transform_expressions: unexpected expression")
        }
    }).collect::<Vec<Rc<SExpr>>>();

    Ok(sexprs)
}

#[cfg(test)]
mod test {
    use crate::sexpr::sexpr_parser::Expr::*;
    use super::parse;
    use std::rc::Rc;
    use crate::sexpr::expr::SExpr::{SFunc, SNumber, SString};

    #[test]
    fn test_string_function_call() {
        let expressions = vec!(
            FuncExpr(String::from("concat")),
            StringExpr(String::from("hello")),
            StringExpr(String::from(" ")),
            StringExpr(String::from("world"))
        );

        let ast = parse(expressions).unwrap();
        let sexpr = format!("{:?}", ast);

        assert_eq!(sexpr, r#"(concat "hello" " " "world")"#)
    }

    #[test]
    fn test_int_function_call() {
        let expressions = vec!(
            FuncExpr(String::from("add")),
            NumberExpr(5.0),
            NumberExpr(4.0),
        );

        let ast = parse(expressions).unwrap();
        let sexpr = format!("{:?}", ast);

        assert_eq!(sexpr, r#"(add 5 4)"#)
    }

    #[test]
    fn test_float_function_call() {
        let expressions = vec!(
            FuncExpr(String::from("add")),
            NumberExpr(5.5),
            NumberExpr(4.5),
        );

        let ast = parse(expressions).unwrap();
        let sexpr = format!("{:?}", ast);

        assert_eq!(sexpr, r#"(add 5.5 4.5)"#)
    }

    #[test]
    fn test_list() {
        let expressions = vec!(
            AstExpr(Rc::new(
                SFunc(String::from("add"), vec!(
                    Rc::new(SNumber(5.3)),
                    Rc::new(SNumber(4.7))
                ))
            )),
            AstExpr(Rc::new(
                SFunc(String::from("println"), vec!(
                    Rc::new(SString(String::from("hello world"))),
                ))
            ))
        );

        let ast = parse(expressions).unwrap();
        let sexpr = format!("{:?}", ast);

        assert_eq!(sexpr, r#"((add 5.3 4.7) (println "hello world"))"#)
    }
}
