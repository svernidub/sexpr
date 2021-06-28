use std::rc::Rc;
use std::fmt::{Debug, Formatter, Result};

#[derive(Clone)]
pub enum SExpr {
    SNumber(f32),
    SString(String),
    SFunc(String, Vec<Rc<SExpr>>),
    SList(Vec<Rc<SExpr>>),
    SBool(bool)
}

impl Debug for SExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use SExpr::*;

        match self {
            SNumber(n) => write!(f, "{}", n),
            SString(s) => write!(f, "\"{}\"", s),
            SFunc(name, args) => {
                write!(f, "(")?;
                write!(f, "{}", name)?;
                for arg in args {
                    write!(f, " {:?}", arg.as_ref())?;
                }
                write!(f, ")")
            },
            SList(elems) => {
                write!(f, "(")?;
                for (i, elem) in elems.into_iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }

                    write!(f, "{:?}", elem.as_ref())?;
                }
                write!(f, ")")
            },
            SBool(val) => write!(f, "{}", val)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::sexpr::expr::SExpr::{SFunc, SNumber, SString, SList, SBool};
    use std::rc::Rc;

    #[test]
    fn test_debug() {
        let expr = SFunc(String::from("+"), vec![
            Rc::new(SFunc(String::from("*"), vec![Rc::new(SNumber(2.0)), Rc::new(SNumber(3.0))])),
            Rc::new(SFunc(String::from("/"), vec![Rc::new(SNumber(10.0)), Rc::new(SNumber(5.0))])),
        ]);


        let sexpr = format!("{:?}", expr);
        assert_eq!(sexpr, "(+ (* 2 3) (/ 10 5))")
    }

    #[test]
    fn test_debug_with_floats() {
        let expr = SFunc(String::from("+"), vec![
            Rc::new(
                SFunc(
                    String::from("*"),
                    vec![Rc::new(SNumber(2.5)), Rc::new(SNumber(3.5))]
                )
            ),
            Rc::new(
                SFunc(
                    String::from("/"),
                    vec![Rc::new(SNumber(7.5)), Rc::new(SNumber(2.5))]
                )
            ),
        ]);


        let sexpr = format!("{:?}", expr);
        assert_eq!(sexpr, "(+ (* 2.5 3.5) (/ 7.5 2.5))")
    }

    #[test]
    fn test_debug_with_strings() {
        let expr = SFunc(String::from("println"), vec![
            Rc::new(
                SFunc(
                    String::from("concat"),
                    vec![
                        Rc::new(SString(String::from("hello "))),
                        Rc::new(SString(String::from("world")))
                    ]
                )
            )
        ]);

        let sexpr = format!("{:?}", expr);
        assert_eq!(sexpr, "(println (concat \"hello \" \"world\"))");
    }

    #[test]
    fn test_debug_empty() {
        let expr = SList(vec![]);
        let sexpr = format!("{:?}", expr);
        assert_eq!(sexpr, "()")
    }

    #[test]
    fn test_list() {
        let expr = SList(
          vec!(
              Rc::new(
                  SFunc(
                      String::from("if"),
                      vec!(
                          Rc::new(SFunc(
                              String::from(">"),
                              vec!(
                                  Rc::new(SNumber(5.0)), Rc::new(SNumber(3.0)))
                          )),
                          Rc::new(SFunc(
                              String::from("println"),
                              vec!(Rc::new(SString(String::from("greater"))))
                          )),
                          Rc::new(SFunc(
                              String::from("println"),
                              vec!(Rc::new(SString(String::from("less"))))
                          ))
                      )

                  )
              )
          )
        );

        let sexpr = format!("{:?}", expr);
        assert_eq!(sexpr, r#"((if (> 5 3) (println "greater") (println "less")))"#)
    }


    #[test]
    fn test_boolean() {
        let expr = SFunc(
            String::from("and"),
            vec!(
                Rc::new(SBool(true)),
                Rc::new(SBool(false))
            )
        );

        let sexpr = format!("{:?}", expr);
        assert_eq!(sexpr, r#"(and true false)"#)
    }
}