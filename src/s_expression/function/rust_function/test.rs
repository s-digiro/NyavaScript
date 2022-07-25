use super::*;
use crate::evaluate::{
    Environment as Env,
    Result as EvalResult,
};

fn dummy_fn(mut args: Vec<SXRef>, _env: &mut Env) -> EvalResult {
    Ok(args.remove(0))
}

#[test]
pub fn execute_works() {
    let subject = RustFunction::new(dummy_fn);

    let args = vec![
        SXRef::number(1),
    ];

    let expected = SXRef::number(1);

    let actual = subject.execute(args, &mut Env::new()).unwrap();

    assert_eq!(expected, actual);
}
