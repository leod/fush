use bytemuck::{Pod, Zeroable};
use sealed::sealed;

use crate::{
    gl::{Sampler2dBinding, Texture2dBinding, UniformBufferBinding, VertexBufferBinding},
    sl::{self, Scalar, Vec2},
    Gl, Numeric, Sl, Uniform,
};

use super::{
    Attachment, Attributes, FragmentDomain, Primitive, Resource, ResourceDomain, ToPod, Vertex,
};

// Uniform interface

impl Uniform<Gl> for bool {
    type InGl = Self;
    type InSl = Scalar<Self>;
}

impl Uniform<Gl> for f32 {
    type InGl = Self;
    type InSl = Scalar<Self>;
}

impl Uniform<Gl> for i32 {
    type InGl = Self;
    type InSl = Scalar<Self>;
}

impl Uniform<Gl> for u32 {
    type InGl = Self;
    type InSl = Scalar<Self>;
}

impl<T: Primitive> Uniform<Gl> for mint::Vector2<T> {
    type InGl = T::Vec2;
    type InSl = Vec2<T>;
}

#[sealed]
impl super::FieldDomain for Gl {
    type Scalar<T: Primitive> = T;
    type Vec2<T: Primitive> = mint::Vector2<T>;

    type Bool = bool;
    type F32 = f32;
    type I32 = i32;
    type U32 = u32;
}

// Vertex interface

#[derive(Clone, Copy, Eq, PartialEq, Pod, Zeroable)]
#[repr(transparent)]
pub struct BoolField(u32);

impl ToPod for bool {
    type Output = BoolField;

    fn to_pod(self) -> Self::Output {
        BoolField(self as u32)
    }
}

impl ToPod for f32 {
    type Output = Self;

    fn to_pod(self) -> Self::Output {
        self
    }
}

impl ToPod for i32 {
    type Output = Self;

    fn to_pod(self) -> Self::Output {
        self
    }
}

impl ToPod for u32 {
    type Output = Self;

    fn to_pod(self) -> Self::Output {
        self
    }
}

impl<T: Primitive> ToPod for mint::Vector2<T> {
    type Output = [<T as ToPod>::Output; 2];

    fn to_pod(self) -> Self::Output {
        [self.x.to_pod(), self.y.to_pod()]
    }
}

impl Vertex<Gl> for bool {
    type InGl = Self;
    type InSl = Scalar<Self>;
}

impl Vertex<Gl> for f32 {
    type InGl = Self;
    type InSl = Scalar<Self>;
}

impl Vertex<Gl> for i32 {
    type InGl = Self;
    type InSl = Scalar<Self>;
}

impl Vertex<Gl> for u32 {
    type InGl = Self;
    type InSl = Scalar<Self>;
}

impl<T: Primitive> Vertex<Gl> for mint::Vector2<T> {
    type InGl = T::Vec2;
    type InSl = Vec2<T>;
}

// Attributes interface

impl<V: Vertex<Gl>> Attributes<Gl> for VertexBufferBinding<V> {
    type InGl = Self;
    type InSl = V::InSl;
}

#[sealed]
impl super::AttributesDomain for Gl {
    type Vertex<V: Vertex<Gl>> = VertexBufferBinding<V>;
}

// Resource interface

impl<T: Numeric> Resource<Gl> for Sampler2dBinding<T> {
    type InGl = Self;
    type InSl = sl::Sampler2d<T>;
}

impl<U: Uniform<Sl>> Resource<Gl> for UniformBufferBinding<U> {
    type InGl = Self;
    type InSl = U::InSl;
}

impl ResourceDomain for Gl {
    type Sampler2d<T: Numeric> = Sampler2dBinding<T>;
    type Uniform<U: Uniform<Gl>> = UniformBufferBinding<U::InSl>;
}

// Fragment interface

impl Attachment<Gl> for Texture2dBinding {}

impl FragmentDomain for Gl {
    type Attachment2d = Texture2dBinding;
}
