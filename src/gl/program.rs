use std::{cell::RefCell, marker::PhantomData};

use crate::{
    compile::compile,
    interface::ResourceInterfaceVisitor,
    sl::{FragmentInput, FragmentOutput, Varying, VertexInput, VertexOutput},
    FragmentInterface, Gl, ResourceInterface, Sl, VertexInterface,
};

use super::{
    untyped, Context, CreateProgramError, DrawParams, GeometryStream, Surface, UniformBufferBinding,
};

pub struct Program<R, A, F> {
    untyped: untyped::Program,
    uniform_buffers: RefCell<Vec<untyped::Buffer>>,
    _phantom: PhantomData<(R, A, F)>,
}

impl<R, V, F> Program<R, V, F>
where
    R: ResourceInterface<Sl, InSl = R>,
    V: VertexInterface<Sl, InSl = V>,
    F: FragmentInterface<Sl, InSl = F>,
{
    pub(crate) fn new<W>(
        context: &Context,
        vertex_shader: fn(R, VertexInput<V>) -> VertexOutput<W>,
        fragment_shader: fn(R, FragmentInput<W>) -> FragmentOutput<F>,
    ) -> Result<Self, CreateProgramError>
    where
        W: Varying,
    {
        let program_def = compile(vertex_shader, fragment_shader);

        println!(
            "{}\n==================={}",
            program_def.vertex_shader_source, program_def.fragment_shader_source
        );

        let untyped = context.untyped.create_program(program_def)?;

        Ok(Program {
            untyped,
            uniform_buffers: RefCell::new(Vec::new()),
            _phantom: PhantomData,
        })
    }

    pub fn draw<S>(
        &self,
        resource: R::InGl,
        geometry: GeometryStream<V>,
        surface: &S,
        draw_params: &DrawParams,
    ) where
        S: Surface<F>,
    {
        // TODO: Surface stuff.

        let mut uniform_buffers = self.uniform_buffers.borrow_mut();
        let mut resource_visitor = ResourceVisitor {
            uniform_buffers: &mut uniform_buffers,
        };
        resource.visit("", &mut resource_visitor);

        // FIXME: Safety: Check element range.
        unsafe {
            self.untyped.draw(&uniform_buffers, geometry.untyped);
        }

        uniform_buffers.clear();
    }
}

struct ResourceVisitor<'a> {
    uniform_buffers: &'a mut Vec<untyped::Buffer>,
}

impl<'a> ResourceInterfaceVisitor<Gl> for ResourceVisitor<'a> {
    fn accept_sampler2d<T: crate::Numeric>(
        &mut self,
        path: &str,
        sampler: &<Gl as crate::ResourceDomain>::Sampler2d<T>,
    ) {
        todo!()
    }

    fn accept_uniform<U: crate::Uniform<Sl, InSl = U>>(
        &mut self,
        _: &str,
        uniform: &UniformBufferBinding<U>,
    ) {
        self.uniform_buffers.push(uniform.untyped.clone());
    }
}
