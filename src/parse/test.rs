use super::*;

use crate::s_expression::ConsCell;

#[test]
pub fn full_parse() {
    let subject = "((lambda (x) (cons x (\"foo\" 2))) (car (3 \"bar\")))";

    let expected = vec![
        SExpressionRef::cons_cell(ConsCell::new( // First actual parenthesis
            SExpressionRef::cons_cell(ConsCell::new(
                SExpressionRef::symbol("lambda".to_owned()),
                SExpressionRef::cons_cell(ConsCell::new(
                    SExpressionRef::cons_cell(ConsCell::new(
                        SExpressionRef::symbol("x".to_owned()),
                        SExpressionRef::nil(),
                    )),
                    SExpressionRef::cons_cell(ConsCell::new(
                        SExpressionRef::cons_cell(ConsCell::new(
                            SExpressionRef::symbol("cons".to_owned()),
                            SExpressionRef::cons_cell(ConsCell::new(
                                SExpressionRef::symbol("x".to_owned()),
                                SExpressionRef::cons_cell(ConsCell::new(
                                    SExpressionRef::cons_cell(ConsCell::new(
                                        SExpressionRef::string("foo".to_owned()),
                                        SExpressionRef::cons_cell(ConsCell::new(
                                            SExpressionRef::number(2),
                                            SExpressionRef::nil(),
                                        )),
                                    )),
                                    SExpressionRef::nil(),
                                )),
                            )),
                        )),
                        SExpressionRef::nil(),
                    )),
                )),
            )),
            SExpressionRef::cons_cell(ConsCell::new(
                SExpressionRef::cons_cell(ConsCell::new(
                    SExpressionRef::symbol("car".to_owned()),
                    SExpressionRef::cons_cell(ConsCell::new(
                        SExpressionRef::cons_cell(ConsCell::new(
                            SExpressionRef::number(3),
                            SExpressionRef::cons_cell(ConsCell::new(
                                SExpressionRef::string("bar".to_owned()),
                                SExpressionRef::nil(),
                            )),
                        )),
                        SExpressionRef::nil(),
                    )),
                )),
                SExpressionRef::nil(),
            )),
        ))
    ];

    assert_eq!(expected, parse(subject).unwrap());
}
