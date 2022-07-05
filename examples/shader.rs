use posh::{
    shader::{FStageIn, FStageOut, Shader, VStageIn, VStageOut},
    var, vec3, Expose, Rep,
};

#[derive(Expose)]
#[expose_derive(UniformBlock)]
struct ModelToClip {
    model_to_view: [f32; 3],
    view_to_clip: [f32; 3],
}

#[derive(Expose)]
#[expose_derive(Resources)]
struct Resources {
    one: ModelToClip,
    two: ModelToClip,
}

#[derive(Expose)]
#[expose_derive(Vertex)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    thickness: f32,
}

#[derive(Expose)]
#[expose_derive(Vertex)]
struct Instance {
    color: [f32; 3],
}

#[derive(Expose)]
#[expose_derive(VOutputs)]
struct VOutputs {
    color: [f32; 3],
    normal: [f32; 3],
}

#[derive(Expose)]
#[expose_derive(FOutputs)]
struct FOutputs {
    color: [f32; 3],
    normal: [f32; 3],
}

fn vertex(res: Rep<Resources>, arg: VStageIn<Vertex>) -> VStageOut<VOutputs> {
    let outputs = Rep::<VOutputs> {
        color: vec3(255.0, 0.0, 0.0),
        normal: res.two.model_to_view * arg.vertex.normal,
    };
    let position = res.one.view_to_clip * res.one.model_to_view * arg.vertex.position;

    VStageOut { outputs, position }
}

fn vertex2(res: Rep<Resources>, arg: VStageIn<(Vertex, Instance)>) -> VStageOut<VOutputs> {
    let (vertex, instance) = arg.vertex;

    let outputs = Rep::<VOutputs> {
        color: instance.color,
        normal: res.one.model_to_view * vertex.normal,
    };
    let position = res.one.model_to_view * vertex.position;

    VStageOut { outputs, position }
}

fn fragment(_: Rep<Resources>, arg: FStageIn<VOutputs>) -> FStageOut<FOutputs> {
    let outputs = var(Rep::<FOutputs> {
        color: arg.inputs.color,
        normal: arg.inputs.normal,
    });

    FStageOut::outputs(fragment)
}

struct MyShader {
    shader: Shader<Resources, Vertex, FOutputs>,
}

struct MyShader2 {
    shader: Shader<Resources, (Vertex, Instance), FOutputs>,
}

fn main() {
    let my_shader = MyShader {
        shader: Shader::new(vertex, fragment),
    };

    let my_shader2 = MyShader2 {
        shader: Shader::new(vertex2, fragment),
    };

    let shaduer: Shader<Resources, _, _> = Shader::new(vertex2, fragment);
}
