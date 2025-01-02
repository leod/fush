use std::{ops::Range, rc::Rc, time::Duration};

use super::{disjoint_timer_query::DisjointTimerQuery, PrimitiveMode};

#[derive(Debug, Clone)]
pub struct DrawCallInfo {
    pub name: String,
    pub vertex_data_lens: Vec<usize>,
    pub element_data_len: Option<usize>,
    pub mode: PrimitiveMode,
    pub index_range: Range<usize>,
    pub num_instances: usize,
}

#[derive(Debug, Clone)]
pub struct DrawCallTrace {
    pub info: DrawCallInfo,
    pub duration: Option<Duration>,
}

#[derive(Debug, Clone)]
pub struct FrameTrace {
    pub draw_calls: Vec<DrawCallTrace>,
}

struct DrawCall {
    info: DrawCallInfo,
    query: Option<Rc<DisjointTimerQuery>>,
}

#[derive(Default)]
pub struct Tracing {
    query_cache: Vec<Rc<DisjointTimerQuery>>,
    draw_calls: Vec<DrawCall>,
}

impl Tracing {
    pub fn start_frame(&mut self) -> Option<FrameTrace> {
        let draw_calls = self
            .draw_calls
            .iter()
            .map(|draw_call| DrawCallTrace {
                info: draw_call.info.clone(),
                duration: draw_call.query.as_ref().and_then(|query| query.get()),
            })
            .collect();

        self.query_cache.extend(
            self.draw_calls
                .drain(..)
                .filter_map(|draw_call| draw_call.query),
        );

        Some(FrameTrace { draw_calls })
    }

    pub fn start_draw_call(
        &mut self,
        info: DrawCallInfo,
        new_query: impl Fn() -> Option<DisjointTimerQuery>,
    ) -> Option<Rc<DisjointTimerQuery>> {
        let query = self
            .query_cache
            .pop()
            .map_or_else(|| new_query().map(Rc::new), Some);

        if let Some(query) = query.as_ref() {
            query.start();
        }

        self.draw_calls.push(DrawCall {
            info,
            query: query.clone(),
        });

        query
    }
}
