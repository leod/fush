use bytemuck::Pod;
use crevice::std140::AsStd140;
use sealed::sealed;

use crate::{
    gl,
    sl::{self, program_def::VertexAttributeDef, ToSl},
    Gl, Glam, Sl, ToGl,
};

use super::math::MathDom;

/// A view of block attributes.
///
/// See [`Block`] for more details.
#[sealed(pub(crate))]
pub trait BlockDom: Copy {
    /// A floating-point value.
    ///
    /// Has [`f32`] as its physical view and [`sl::F32`] as its logical view.
    type F32: Block<Self> + sl::ToSl<Output = sl::F32>;

    /// A signed integer value.
    ///
    /// Has [`i32`] as its physical view and [`sl::I32`] as its logical view.
    type I32: Block<Self> + sl::ToSl<Output = sl::I32>;

    /// An unsigned integer value.
    ///
    /// Has [`u32`] as its physical view and [`sl::U32`] as its logical view.
    type U32: Block<Self> + sl::ToSl<Output = sl::U32>;

    /// A boolean value.
    ///
    /// Has [`gl::Bool`](crate::gl::Bool) as its physical view and [`sl::Bool`] as its
    /// logical view.
    ///
    /// TODO: Bool in `Block`. Need to special case vertex attributes.
    //type Bool: Block<Self> + sl::ToSl<Output = sl::Bool>;

    /// A two-crate::dimensional floating-point vector.
    ///
    /// Has [`gl::Vec2`](crate::gl::Vec2) as its physical view and [`sl::Vec2`]
    /// as its logical view.
    type Vec2: Block<Self> + sl::ToSl<Output = sl::Vec2>;

    /// A three-dimensional floating-point vector.
    ///
    /// Has [`gl::Vec3`](crate::gl::Vec3) as its physical view and [`sl::Vec3`]
    /// as its logical view.
    type Vec3: Block<Self> + sl::ToSl<Output = sl::Vec3>;

    /// A four-dimensional floating-point vector.
    ///
    /// Has [`gl::Vec4`](crate::gl::Vec4) as its physical view and [`sl::Vec4`]
    /// as its logical view.
    type Vec4: Block<Self> + sl::ToSl<Output = sl::Vec4>;

    /// A two-dimensional signed integer vector.
    ///
    /// Has [`gl::IVec2`](crate::gl::IVec2) as its physical view and
    /// [`sl::IVec2`] as its logical view.
    type IVec2: Block<Self> + sl::ToSl<Output = sl::IVec2>;

    /// A three-dimensional signed integer vector.
    ///
    /// Has [`gl::IVec3`](crate::gl::IVec3) as its physical view and
    /// [`sl::IVec3`] as its logical view.
    type IVec3: Block<Self> + sl::ToSl<Output = sl::IVec3>;

    /// A four-dimensional signed integer vector.
    ///
    /// Has [`gl::IVec4`](crate::gl::IVec4) as its physical view and
    /// [`sl::IVec4`] as its logical view.
    type IVec4: Block<Self> + sl::ToSl<Output = sl::IVec4>;

    /// A two-dimensional unsigned integer vector.
    ///
    /// Has [`gl::UVec2`](crate::gl::UVec2) as its physical view and
    /// [`sl::UVec2`] as its logical view.
    type UVec2: Block<Self> + sl::ToSl<Output = sl::UVec2>;

    /// A three-dimensional unsigned integer vector.
    ///
    /// Has [`gl::UVec3`](crate::gl::UVec3) as its physical view and
    /// [`sl::UVec3`] as its logical view.
    type UVec3: Block<Self> + sl::ToSl<Output = sl::UVec3>;

    /// A four-dimensional unsigned integer vector.
    ///
    /// Has [`gl::UVec4`](crate::gl::UVec4) as its physical view and
    /// [`sl::UVec4`] as its logical view.
    type UVec4: Block<Self> + sl::ToSl<Output = sl::UVec4>;

    /// A two-by-two floating-point matrix.
    ///
    /// Has [`gl::Mat2`](crate::gl::Mat2) as its physical view and [`sl::Mat2`]
    /// as its logical view.
    type Mat2: Block<Self> + sl::ToSl<Output = sl::Mat2>;

    /// A three-by-three floating-point matrix.
    ///
    /// Has [`gl::Mat3`](crate::gl::Mat3) as its physical view and [`sl::Mat3`]
    /// as its logical view.
    type Mat3: Block<Self> + sl::ToSl<Output = sl::Mat3>;

    /// A four-by-four floating-point matrix.
    ///
    /// Has [`gl::Mat4`](crate::gl::Mat4) as its physical view and [`sl::Mat4`]
    /// as its logical view.
    type Mat4: Block<Self> + sl::ToSl<Output = sl::Mat4>;
}

#[sealed]
impl BlockDom for Gl {
    type F32 = f32;
    type I32 = i32;
    type U32 = u32;
    type Vec2 = gl::Vec2;
    type Vec3 = gl::Vec3;
    type Vec4 = gl::Vec4;
    type IVec2 = gl::IVec2;
    type IVec3 = gl::IVec3;
    type IVec4 = gl::IVec4;
    type UVec2 = gl::UVec2;
    type UVec3 = gl::UVec3;
    type UVec4 = gl::UVec4;
    type Mat2 = gl::Mat2;
    type Mat3 = gl::Mat3;
    type Mat4 = gl::Mat4;
}

#[sealed]
impl BlockDom for Sl {
    type F32 = sl::F32;
    type I32 = sl::I32;
    type U32 = sl::U32;
    type Vec2 = sl::Vec2;
    type Vec3 = sl::Vec3;
    type Vec4 = sl::Vec4;
    type IVec2 = sl::IVec2;
    type IVec3 = sl::IVec3;
    type IVec4 = sl::IVec4;
    type UVec2 = sl::UVec2;
    type UVec3 = sl::UVec3;
    type UVec4 = sl::UVec4;
    type Mat2 = sl::Mat2;
    type Mat3 = sl::Mat3;
    type Mat4 = sl::Mat4;
}

/// Plain-old vertex or uniform block data.
///
/// Types that implement [`Block`] can be used as fields in types that implement
/// [`VsInterface`] or [`UniformInterface`]. This allows them to be passed to
/// shaders in draw calls.
///
/// `Block` declarations are generic in [`BlockDom`] and can be instantiated as
/// their [`Sl`] view or their [`Gl`] view. The views have the following purpose
/// respectively:
///
/// 1. `Block<Sl>` is a view of block data as seen in shader definitions.
///
/// 2. `Block<Gl>` is a view of block data as passed to host-side draw calls in
///    the form of buffer bindings.
///
/// By convention, the generic view parameter of `Block` declarations is named
/// `D`.
///
/// User-defined types should implement this trait with the [derive
/// macro](`posh_derive::Block`).
///
/// # Example
///
/// ```
/// use posh::{sl, Block, BlockDom, Sl};
///
/// #[derive(Clone, Copy, Block)]
/// #[repr(C)]
/// struct SomeColor<D: BlockDom> {
///     rainbow: D::U32,
///     thing: D::Vec2,
/// }
///
/// #[derive(Clone, Copy, Block)]
/// #[repr(C)]
/// struct MyVertex<D: BlockDom> {
///     position: D::Vec3,
///     normal: D::Vec3,
///     color: SomeColor<D>,
/// }
///
/// // A function in the shading language that does something with `MyVertex`.
/// fn my_extrude(vertex: MyVertex<Sl>, offset: sl::F32) -> sl::Vec3 {
///     vertex.position + vertex.normal * offset
/// }
/// ```
///
/// # Safety
///
/// TODO
pub unsafe trait Block<D: BlockDom>: sl::ToSl {
    /// The physical view of `Self`.
    ///
    /// This is the type through which the host provides block data in draw
    /// calls.
    type Gl: Block<Gl> + AsStd140 + Pod + sl::ToSl<Output = Self::Sl>;

    /// The logical view of `Self`.
    ///
    /// This is the type through which shaders access block data.
    type Sl: Block<Sl> + sl::Varying + sl::ToSl<Output = Self::Sl>;

    type Math<M: MathDom>: ToGl<Output = Self::Gl>;

    #[doc(hidden)]
    fn uniform_input(_path: &str) -> Self {
        unimplemented!()
    }

    #[doc(hidden)]
    fn vertex_input(_path: &str) -> Self {
        unimplemented!()
    }

    #[doc(hidden)]
    fn vertex_attribute_defs(path: &str) -> Vec<VertexAttributeDef> {
        <Self::Sl as Block<Sl>>::vertex_attribute_defs(path)
    }
}

macro_rules! glam_type {
    (F32) => {
        f32
    };
    (I32) => {
        i32
    };
    (U32) => {
        u32
    };
    (Bool) => {
        gl::Bool
    };
    ($ty:ident) => {
        glam::$ty
    };
}

macro_rules! impl_block {
    ($gl:ty, $ty:ident) => {
        unsafe impl Block<Gl> for $gl {
            type Gl = $gl;
            type Sl = sl::$ty;
            type Math<M: MathDom> = <M as MathDom>::$ty;
        }

        unsafe impl Block<Sl> for sl::$ty {
            type Gl = $gl;
            type Sl = sl::$ty;
            type Math<M: MathDom> = <M as MathDom>::$ty;

            fn uniform_input(path: &str) -> Self {
                <Self as sl::Object>::from_arg(path)
            }

            fn vertex_input(path: &str) -> Self {
                // FIXME: Cast from u32 to bool!
                <Self as sl::Object>::from_arg(path)
            }

            fn vertex_attribute_defs(path: &str) -> Vec<VertexAttributeDef> {
                vec![VertexAttributeDef {
                    name: path.to_string(),
                    ty: <Self as sl::Object>::ty().built_in_type().unwrap(),
                    offset: 0,
                }]
            }
        }

        #[cfg(feature = "glam")]
        unsafe impl Block<Glam> for glam_type!($ty) {
            type Gl = $gl;
            type Sl = sl::$ty;
            type Math<M: MathDom> = <M as MathDom>::$ty;
        }
    };
}

impl_block!(f32, F32);
impl_block!(i32, I32);
impl_block!(u32, U32);
impl_block!(gl::Vec2, Vec2);
impl_block!(gl::Vec3, Vec3);
impl_block!(gl::Vec4, Vec4);
impl_block!(gl::IVec2, IVec2);
impl_block!(gl::IVec3, IVec3);
impl_block!(gl::IVec4, IVec4);
impl_block!(gl::UVec2, UVec2);
impl_block!(gl::UVec3, UVec3);
impl_block!(gl::UVec4, UVec4);
impl_block!(gl::Mat2, Mat2);
impl_block!(gl::Mat3, Mat3);
impl_block!(gl::Mat4, Mat4);
