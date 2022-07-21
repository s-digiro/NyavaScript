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
