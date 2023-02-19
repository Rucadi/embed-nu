use nu_protocol::{
    ast::Block,
    engine::{EngineState, StateWorkingSet},
    Span,
};

use crate::error::{CrateError, CrateResult};

pub trait NewEmpty {
    fn empty() -> Self;
}

impl NewEmpty for Span {
    #[inline]
    fn empty() -> Self {
        Span::new(0, 0)
    }
}

pub fn parse_nu_script(engine_state: &mut EngineState, contents: String) -> CrateResult<Block> {
    let mut working_set = StateWorkingSet::new(&engine_state);
    let (block, err) = nu_parser::parse(&mut working_set, None, &contents.into_bytes(), false, &[]);

    if let Some(err) = err {
        Err(CrateError::from(err))
    } else {
        let delta = working_set.render();
        engine_state.merge_delta(delta)?;

        Ok(block)
    }
}
