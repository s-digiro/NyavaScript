use super::*;

use crate::expression::{ Atom, ConsCell, ValRef };

#[test]
pub fn full_parse() {
    let subject = "((lambda (x) (cons x (\"foo\" 2))) (car (3 \"bar\")))";

    let expected =
        ValRef::cons_cell(ConsCell::new( // First actual parenthesis
            ValRef::cons_cell(ConsCell::new(
                ValRef::atom(Atom::Symbol("lambda".to_owned())),
                ValRef::cons_cell(ConsCell::new(
                    ValRef::cons_cell(ConsCell::new(
                        ValRef::atom(Atom::Symbol("x".to_owned())),
                        ValRef::nil(),
                    )),
                    ValRef::cons_cell(ConsCell::new(
                        ValRef::cons_cell(ConsCell::new(
                            ValRef::atom(Atom::Symbol("cons".to_owned())),
                            ValRef::cons_cell(ConsCell::new(
                                ValRef::atom(Atom::Symbol("x".to_owned())),
                                ValRef::cons_cell(ConsCell::new(
                                    ValRef::cons_cell(ConsCell::new(
                                        ValRef::atom(Atom::String("foo".to_owned())),
                                        ValRef::cons_cell(ConsCell::new(
                                            ValRef::atom(Atom::Number(2)),
                                            ValRef::nil(),
                                        )),
                                    )),
                                    ValRef::nil(),
                                )),
                            )),
                        )),
                        ValRef::nil(),
                    )),
                )),
            )),
            ValRef::cons_cell(ConsCell::new(
                ValRef::cons_cell(ConsCell::new(
                    ValRef::atom(Atom::Symbol("car".to_owned())),
                    ValRef::cons_cell(ConsCell::new(
                        ValRef::cons_cell(ConsCell::new(
                            ValRef::atom(Atom::Number(3)),
                            ValRef::cons_cell(ConsCell::new(
                                ValRef::atom(Atom::String("bar".to_owned())),
                                ValRef::nil(),
                            )),
                        )),
                        ValRef::nil(),
                    )),
                )),
                ValRef::nil(),
            )),
        ));

    println!("{}", expected);
    assert_eq!(expected, parse(subject).unwrap());
}
