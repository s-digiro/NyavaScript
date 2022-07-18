use super::*;

use crate::s_expression::{ ConsCell, SExpression };

#[test]
pub fn full_parse() {
    let subject = "((lambda (x) (cons x (\"foo\" 2))) (car (3 \"bar\")))";

    let expected =
        SExpression::cons_cell(ConsCell::new( // First actual parenthesis
            SExpression::cons_cell(ConsCell::new(
                SExpression::symbol("lambda".to_owned()),
                SExpression::cons_cell(ConsCell::new(
                    SExpression::cons_cell(ConsCell::new(
                        SExpression::symbol("x".to_owned()),
                        SExpression::nil(),
                    )),
                    SExpression::cons_cell(ConsCell::new(
                        SExpression::cons_cell(ConsCell::new(
                            SExpression::symbol("cons".to_owned()),
                            SExpression::cons_cell(ConsCell::new(
                                SExpression::symbol("x".to_owned()),
                                SExpression::cons_cell(ConsCell::new(
                                    SExpression::cons_cell(ConsCell::new(
                                        SExpression::string("foo".to_owned()),
                                        SExpression::cons_cell(ConsCell::new(
                                            SExpression::number(2),
                                            SExpression::nil(),
                                        )),
                                    )),
                                    SExpression::nil(),
                                )),
                            )),
                        )),
                        SExpression::nil(),
                    )),
                )),
            )),
            SExpression::cons_cell(ConsCell::new(
                SExpression::cons_cell(ConsCell::new(
                    SExpression::symbol("car".to_owned()),
                    SExpression::cons_cell(ConsCell::new(
                        SExpression::cons_cell(ConsCell::new(
                            SExpression::number(3),
                            SExpression::cons_cell(ConsCell::new(
                                SExpression::string("bar".to_owned()),
                                SExpression::nil(),
                            )),
                        )),
                        SExpression::nil(),
                    )),
                )),
                SExpression::nil(),
            )),
        ));

    println!("{}", expected);
    assert_eq!(expected, parse(subject).unwrap());
}
