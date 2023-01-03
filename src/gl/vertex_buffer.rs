use std::{cell::Cell, marker::PhantomData, rc::Rc};

use crate::{Sl, ToPod, Vertex};

use super::{untyped, BufferUsage};

#[derive(Clone)]
pub struct VertexBuffer<V: Vertex<Sl>> {
    pub(crate) untyped: untyped::Buffer,
    _phantom: PhantomData<V>,
}

fn vertex_size<V: Vertex<Sl>>() -> usize {
    std::mem::size_of::<<V::InGl as ToPod>::Output>()
}

impl<V: Vertex<Sl>> VertexBuffer<V> {
    /// # Panics
    ///
    /// Panics if the length of `untyped` is not a multiple of the size of
    /// `V::Pod`.
    ///
    /// # TODO
    ///
    /// Since `untyped::Buffer` is `Rc`-cloneable, the underlying buffer can
    /// still be modified. Check if we want to allow this.
    pub fn from_untyped(untyped: untyped::Buffer) -> Self {
        assert!(vertex_size::<V>() > 0);
        assert_eq!(untyped.len() % vertex_size::<V>(), 0);

        Self {
            untyped,
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
        self.untyped.len() / vertex_size::<V>()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn set(&self, data: &[V::Pod]) {
        self.untyped.set(data);
    }
}
