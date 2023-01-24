use super::*;
use crate::evaluate::{
    Environment as Env,
    Result as EvalResult,
};
use crate::s_expression::{
    RustFunction,
    SExpressionRef as SXRef,
};

pub struct DebugScope;

impl DebugScope {
    pub fn new() -> HashScope {
        let mut ret = HashScope::new();

        ret.insert(
            "debug_env".into(),
            RustFunction::new(Self::debug_env).into(),
        );

        ret
    }

    pub fn debug_env(_args: Vec<SXRef>, env: &mut Env) -> EvalResult {
        let env_str = format!("{:?}", env);
        let ret = SXRef::string(env_str);
        Ok(ret)
    }
}
