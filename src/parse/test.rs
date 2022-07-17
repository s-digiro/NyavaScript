use super::*;

use crate::expression::{ ConsCell, Value };

#[test]
pub fn full_parse() {
    let subject = "((lambda (x) (cons x (\"foo\" 2))) (car (3 \"bar\")))";

    let expected =
        Value::cons_cell(ConsCell::new( // First actual parenthesis
            Value::cons_cell(ConsCell::new(
                Value::symbol("lambda".to_owned()),
                Value::cons_cell(ConsCell::new(
                    Value::cons_cell(ConsCell::new(
                        Value::symbol("x".to_owned()),
                        Value::nil(),
                    )),
                    Value::cons_cell(ConsCell::new(
                        Value::cons_cell(ConsCell::new(
                            Value::symbol("cons".to_owned()),
                            Value::cons_cell(ConsCell::new(
                                Value::symbol("x".to_owned()),
                                Value::cons_cell(ConsCell::new(
                                    Value::cons_cell(ConsCell::new(
                                        Value::string("foo".to_owned()),
                                        Value::cons_cell(ConsCell::new(
                                            Value::number(2),
                                            Value::nil(),
                                        )),
                                    )),
                                    Value::nil(),
                                )),
                            )),
                        )),
                        Value::nil(),
                    )),
                )),
            )),
            Value::cons_cell(ConsCell::new(
                Value::cons_cell(ConsCell::new(
                    Value::symbol("car".to_owned()),
                    Value::cons_cell(ConsCell::new(
                        Value::cons_cell(ConsCell::new(
                            Value::number(3),
                            Value::cons_cell(ConsCell::new(
                                Value::string("bar".to_owned()),
                                Value::nil(),
                            )),
                        )),
                        Value::nil(),
                    )),
                )),
                Value::nil(),
            )),
        ));

    println!("{}", expected);
    assert_eq!(expected, parse(subject).unwrap());
}
