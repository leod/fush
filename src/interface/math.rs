use sealed::sealed;
use type_equals::TypeEquals;

use crate::{
    gl,
    sl::{self, ToSl},
    BlockDom, Gl, ToGl,
};

#[sealed]
pub trait MathDom: BlockDom {
    type F32: ToGl<Output = f32> + ToSl<Output = sl::F32> + TypeEquals<Other = <Self as BlockDom>::F32>;
    type I32: ToGl<Output = i32> + ToSl<Output = sl::I32>;
    type U32: ToGl<Output = u32> + ToSl<Output = sl::U32>;
    type Vec2: ToGl<Output = gl::Vec2> + ToSl<Output = sl::Vec2>;
    type Vec3: ToGl<Output = gl::Vec3> + ToSl<Output = sl::Vec3>;
    type Vec4: ToGl<Output = gl::Vec4> + ToSl<Output = sl::Vec4>;
    type IVec2: ToGl<Output = gl::IVec2> + ToSl<Output = sl::IVec2>;
    type IVec3: ToGl<Output = gl::IVec3> + ToSl<Output = sl::IVec3>;
    type IVec4: ToGl<Output = gl::IVec4> + ToSl<Output = sl::IVec4>;
    type UVec2: ToGl<Output = gl::UVec2> + ToSl<Output = sl::UVec2>;
    type UVec3: ToGl<Output = gl::UVec3> + ToSl<Output = sl::UVec3>;
    type UVec4: ToGl<Output = gl::UVec4> + ToSl<Output = sl::UVec4>;
    type Mat2: ToGl<Output = gl::Mat2> + ToSl<Output = sl::Mat2>;
    type Mat3: ToGl<Output = gl::Mat3> + ToSl<Output = sl::Mat3>;
    type Mat4: ToGl<Output = gl::Mat4> + ToSl<Output = sl::Mat4>;
}

#[sealed]
impl MathDom for Gl {
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

#[cfg(feature = "mint")]
pub struct Mint;

/*#[cfg(feature = "mint")]
#[sealed]
impl MathDom for Mint {
    type F32 = f32;
    type I32 = i32;
    type U32 = u32;
    type Vec2 = mint::Vector2<f32>;
    type Vec3 = mint::Vector3<f32>;
    type Vec4 = mint::Vector4<f32>;
    type IVec2 = mint::Vector2<i32>;
    type IVec3 = mint::Vector3<i32>;
    type IVec4 = mint::Vector4<i32>;
    type UVec2 = mint::Vector2<u32>;
    type UVec3 = mint::Vector3<u32>;
    type UVec4 = mint::Vector4<u32>;
    type Mat2 = mint::ColumnMatrix2<f32>;
    type Mat3 = mint::ColumnMatrix3<f32>;
    type Mat4 = mint::ColumnMatrix4<f32>;
}
*/

#[cfg(feature = "glam")]
#[derive(Debug, Copy, Clone)]
pub struct Glam;

#[cfg(feature = "glam")]
#[sealed]
impl super::block::BlockDom for Glam {
    type F32 = f32;
    type I32 = i32;
    type U32 = u32;
    type Vec2 = glam::Vec2;
    type Vec3 = glam::Vec3;
    type Vec4 = glam::Vec4;
    type IVec2 = glam::IVec2;
    type IVec3 = glam::IVec3;
    type IVec4 = glam::IVec4;
    type UVec2 = glam::UVec2;
    type UVec3 = glam::UVec3;
    type UVec4 = glam::UVec4;
    type Mat2 = glam::Mat2;
    type Mat3 = glam::Mat3;
    type Mat4 = glam::Mat4;
}

#[cfg(feature = "glam")]
#[sealed]
impl MathDom for Glam {
    type F32 = f32;
    type I32 = i32;
    type U32 = u32;
    type Vec2 = glam::Vec2;
    type Vec3 = glam::Vec3;
    type Vec4 = glam::Vec4;
    type IVec2 = glam::IVec2;
    type IVec3 = glam::IVec3;
    type IVec4 = glam::IVec4;
    type UVec2 = glam::UVec2;
    type UVec3 = glam::UVec3;
    type UVec4 = glam::UVec4;
    type Mat2 = glam::Mat2;
    type Mat3 = glam::Mat3;
    type Mat4 = glam::Mat4;
}
