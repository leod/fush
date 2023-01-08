use std::marker::PhantomData;

use crate::{gen::glsl, sl::Object, FragmentInterface, ResourceInterface, Sl, VertexInterface};

use super::{primitives::value_arg, Bool, Varying, Vec2, Vec4, F32, U32};

pub struct VertexInput<V> {
    pub vertex: V,
    pub vertex_id: U32,
    pub instance_id: U32,
}

pub struct VertexOutput<W> {
    pub position: Vec4<f32>,
    pub varying: W,
    pub point_size: Option<F32>,
}

impl<W> VertexOutput<W> {
    pub fn new(position: Vec4<f32>, varying: W) -> Self {
        Self {
            position,
            varying,
            point_size: None,
        }
    }
}

pub struct FragmentInput<W> {
    pub varying: W,
    pub fragment_coord: Vec4<f32>,
    pub front_facing: Bool,
    pub point_coord: Vec2<f32>,
}

pub struct FragmentOutput<F> {
    pub fragment: F,
    pub fragment_depth: Option<F32>,
}

pub struct ProgramDef<R, A, F> {
    _phantom: PhantomData<(R, A, F)>,
}

impl<R, V, F> ProgramDef<R, V, F>
where
    R: ResourceInterface<Sl, InSl = R>,
    V: VertexInterface<Sl, InSl = V>,
    F: FragmentInterface<Sl, InSl = F>,
{
    pub fn new<W>(
        vertex_shader: fn(R, VertexInput<V>) -> VertexOutput<W>,
        fragment_shader: fn(R, FragmentInput<W>) -> FragmentOutput<F>,
    ) -> Self
    where
        W: Varying,
    {
        let resources = R::shader_input("resources");

        let vertex_source = {
            let varying_attributes = W::attributes("output");

            let input = VertexInput {
                vertex: V::shader_input("vertex"),
                vertex_id: value_arg("gl_VertexID"),
                instance_id: value_arg("gl_InstanceID"),
            };

            let output = vertex_shader(resources, input);
            let mut exprs = vec![("gl_Position", output.position.expr())];
            exprs.extend(
                varying_attributes
                    .iter()
                    .zip(output.varying.shader_outputs())
                    .map(|((name, _), expr)| (name.as_str(), expr)),
            );

            let mut source = String::new();
            glsl::write_shader_stage(&mut source, &exprs).unwrap();

            source
        };

        println!("{vertex_source}");

        Self {
            _phantom: PhantomData,
        }
    }
}
