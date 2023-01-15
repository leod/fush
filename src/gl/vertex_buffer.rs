use std::{marker::PhantomData, rc::Rc};

use crevice::std140::AsStd140;

use crate::{Block, Sl};

use super::{untyped, BufferUsage};

#[derive(Clone)]
pub struct VertexBuffer<V> {
    pub(crate) untyped: Rc<untyped::Buffer>,
    _phantom: PhantomData<V>,
}

impl<V: Block<Sl>> VertexBuffer<V> {
    pub(crate) fn from_untyped(untyped: untyped::Buffer) -> Self {
        assert!(vertex_size::<V>() > 0);
        assert_eq!(untyped.len() % vertex_size::<V>(), 0);

        Self {
            untyped: Rc::new(untyped),
            _phantom: PhantomData,
        }
    }

    pub fn gl(&self) -> &Rc<glow::Context> {
        self.untyped.gl()
    }

    pub fn usage(&self) -> BufferUsage {
        self.untyped.usage()
    }

    pub fn len(&self) -> usize {
        assert_eq!(self.untyped.len() % vertex_size::<V>(), 0);

        self.untyped.len() / vertex_size::<V>()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn set(&self, data: &[V]) {
        todo!()
    }
}

pub(super) const fn vertex_size<V: Block<Sl>>() -> usize {
    std::mem::size_of::<<V::InGl as AsStd140>::Output>()
}
