use std::{rc::Rc, time::Duration};

use glow::HasContext;

use super::{
    context::ContextShared,
    error::{check_gl_error, QueryError},
};

pub struct DisjointTimerQuery {
    ctx: Rc<ContextShared>,
    id: glow::Query,
}

impl DisjointTimerQuery {
    pub(super) fn new(ctx: Rc<ContextShared>) -> Result<Self, QueryError> {
        if !ctx.caps().disjoint_timer_query_webgl2 {
            return Err(QueryError::Unsupported);
        }

        let gl = ctx.gl();
        let id = unsafe { gl.create_query() }.map_err(QueryError::ObjectCreation)?;

        check_gl_error(gl, "after query creation").map_err(QueryError::Unexpected)?;

        Ok(Self { ctx, id })
    }

    pub fn start(&self) {
        let gl = self.ctx.gl();

        unsafe {
            gl.begin_query(glow::TIME_ELAPSED, self.id);
        }
    }

    pub fn stop(&self) {
        let gl = self.ctx.gl();

        unsafe {
            gl.end_query(glow::TIME_ELAPSED);
        }
    }

    pub fn get(&self) -> Option<Duration> {
        let gl = self.ctx.gl();

        let available =
            unsafe { gl.get_query_parameter_u32(self.id, glow::QUERY_RESULT_AVAILABLE) };
        let disjoint = unsafe { gl.get_parameter_bool(0x8FBB) };

        if available != 0 && !disjoint {
            // FIXME: The spec
            // (<https://registry.khronos.org/webgl/extensions/EXT_disjoint_timer_query_webgl2/>)
            // seems to say that the query result should be an `u64`, which
            // makes sense, but I'm not sure how to retrieve an `u64` with
            // `glow`.
            let nanos = unsafe { gl.get_query_parameter_u32(self.id, glow::QUERY_RESULT) };

            Some(Duration::from_nanos(nanos as u64))
        } else {
            None
        }
    }
}

impl Drop for DisjointTimerQuery {
    fn drop(&mut self) {
        let gl = self.ctx.gl();

        unsafe {
            gl.delete_query(self.id);
        }
    }
}
