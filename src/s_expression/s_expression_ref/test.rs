use super::SExpressionRef as SXRef;

#[test]
pub fn iter_works() {
    let sx = SXRef::from(vec![
        SXRef::number(1),
        SXRef::number(2),
        SXRef::number(3),
    ]);

    let mut it = sx.iter();

    assert_eq!(Some(SXRef::number(1)), it.next());
    assert_eq!(Some(SXRef::number(2)), it.next());
    assert_eq!(Some(SXRef::number(3)), it.next());
    assert_eq!(None, it.next());
}

#[test]
pub fn iter_on_non_list() {
    let sx = SXRef::number(3);

    let mut it = sx.iter();

    assert_eq!(None, it.next());
}

#[test]
pub fn len_works_on_list() {
    let sx = SXRef::from(vec![
        SXRef::number(1),
        SXRef::number(2),
        SXRef::number(3),
    ]);

    assert_eq!(sx.len(), 3);
}

#[test]
pub fn len_works_on_non_list() {
    let sx = SXRef::number(3);

    assert_eq!(sx.len(), 0);
}

#[test]
pub fn sxref_from_empty_list_is_nil() {
    let subject: Vec<SXRef> = Vec::new();

    let expected = SXRef::nil();

    let actual = SXRef::from(subject);

    assert_eq!(expected, actual);
}

#[test]
fn from_iter_returns_args_in_correct_order() {
    let subject = SXRef::from(vec![
        SXRef::number(1),
        SXRef::number(2),
        SXRef::number(3),
    ]);

    let expected = SXRef::from(vec![
        SXRef::number(1),
        SXRef::number(2),
        SXRef::number(3),
    ]);

    let actual = subject.iter().collect();

    assert_eq!(expected, actual);
}
