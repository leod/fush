mod interface;

pub mod gl;
#[macro_use]
pub mod sl;

pub use interface::{
    Block, BlockDom, FsInterface, FsInterfaceDom, MathDom, UniformInterface, UniformInterfaceDom,
    UniformNonUnit, UniformUnion, VsInterface, VsInterfaceDom,
};

#[cfg(feature = "glam")]
pub use interface::Glam;
#[cfg(feature = "mint")]
pub use interface::Mint;

pub use posh_derive::{Block, FsInterface, UniformInterface, VsInterface};

pub use bytemuck;
pub use crevice;
#[cfg(feature = "glam")]
pub use glam;
pub use glow;
#[cfg(feature = "mint")]
pub use mint;

use bytemuck::Pod;
use crevice::std140::AsStd140;

use sl::ToSl;

/// The graphics library's view of shader inputs and outputs.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Gl;

/// The shading language's view of shader inputs and outputs.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Sl;

pub trait ToGl {
    // FIXME: Consider adding a `Block<Gl, Gl = Self::Output>` requirement here.
    type Output: Copy + Pod + AsStd140 + ToSl;

    fn to_gl(self) -> Self::Output;
}

impl ToGl for f32 {
    type Output = f32;

    fn to_gl(self) -> Self::Output {
        self
    }
}

impl ToGl for i32 {
    type Output = i32;

    fn to_gl(self) -> Self::Output {
        self
    }
}

impl ToGl for u32 {
    type Output = u32;

    fn to_gl(self) -> Self::Output {
        self
    }
}

// Hidden unstable symbols, needed for `posh-derive`.
#[doc(hidden)]
pub mod internal {
    pub use super::{
        interface::{FragmentVisitor, UniformVisitor, VertexField, VertexVisitor},
        sl::{
            dag::{Expr, StructType, Type},
            primitives::{field, simplify_struct_literal, value_arg},
            unique_struct_type,
        },
    };

    #[doc(hidden)]
    pub fn join_ident_path(lhs: &str, rhs: &str) -> String {
        format!("{lhs}_{rhs}")
    }
}

// Re-export `crate` as `posh` for `posh-derive`.
extern crate self as posh;
