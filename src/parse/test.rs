use super::*;

use crate::expression::{ Atom, ConsCell, ExRef };

#[test]
pub fn full_parse() {
    let subject = "((lambda (x) (cons x (\"foo\" 2))) (car (3 \"bar\")))";

    let expected =
        ExRef::cons_cell(ConsCell::new( // First actual parenthesis
            ExRef::cons_cell(ConsCell::new(
                ExRef::atom(Atom::Symbol("lambda".to_owned())),
                ExRef::cons_cell(ConsCell::new(
                    ExRef::cons_cell(ConsCell::new(
                        ExRef::atom(Atom::Symbol("x".to_owned())),
                        ExRef::nil(),
                    )),
                    ExRef::cons_cell(ConsCell::new(
                        ExRef::cons_cell(ConsCell::new(
                            ExRef::atom(Atom::Symbol("cons".to_owned())),
                            ExRef::cons_cell(ConsCell::new(
                                ExRef::atom(Atom::Symbol("x".to_owned())),
                                ExRef::cons_cell(ConsCell::new(
                                    ExRef::cons_cell(ConsCell::new(
                                        ExRef::atom(Atom::String("foo".to_owned())),
                                        ExRef::cons_cell(ConsCell::new(
                                            ExRef::atom(Atom::Number(2)),
                                            ExRef::nil(),
                                        )),
                                    )),
                                    ExRef::nil(),
                                )),
                            )),
                        )),
                        ExRef::nil(),
                    )),
                )),
            )),
            ExRef::cons_cell(ConsCell::new(
                ExRef::cons_cell(ConsCell::new(
                    ExRef::atom(Atom::Symbol("car".to_owned())),
                    ExRef::cons_cell(ConsCell::new(
                        ExRef::cons_cell(ConsCell::new(
                            ExRef::atom(Atom::Number(3)),
                            ExRef::cons_cell(ConsCell::new(
                                ExRef::atom(Atom::String("bar".to_owned())),
                                ExRef::nil(),
                            )),
                        )),
                        ExRef::nil(),
                    )),
                )),
                ExRef::nil(),
            )),
        ));

    println!("{}", expected);
    assert_eq!(expected, parse(subject).unwrap());
}
