use std::rc::Rc;

use bytemuck::Pod;

use crate::{
    gl::{BufferError, BufferUsage, ElementType, ProgramError, VertexArrayError},
    program_def::{ProgramDef, VertexDef},
};

use super::{Buffer, Capabilities, Program, VertexArray};

pub struct Context {
    gl: Rc<glow::Context>,
    capabilities: Capabilities,
}

impl Context {
    pub fn new(gl: glow::Context) -> Self {
        let capabilities = Capabilities::new(&gl);

        Self {
            gl: Rc::new(gl),
            capabilities,
        }
    }

    pub fn gl(&self) -> &Rc<glow::Context> {
        &self.gl
    }

    pub fn capabilities(&self) -> &Capabilities {
        &self.capabilities
    }

    pub fn create_buffer<T: Pod>(
        &self,
        data: &[T],
        usage: BufferUsage,
    ) -> Result<Buffer, BufferError> {
        Buffer::new(self.gl.clone(), data, usage)
    }

    pub fn create_vertex_array(
        &self,
        vertex_buffers: &[(&Buffer, VertexDef)],
        element_buffer: Option<(&Buffer, ElementType)>,
    ) -> Result<VertexArray, VertexArrayError> {
        VertexArray::new(self.gl.clone(), vertex_buffers, element_buffer)
    }

    pub fn create_program(&self, def: ProgramDef) -> Result<Program, ProgramError> {
        Program::new(self.gl.clone(), def)
    }
}
