mod erased;
#[doc(hidden)]
pub mod fields;
mod resource;
pub mod show;
mod stage;
mod vertex;

use std::marker::PhantomData;

pub use erased::{ErasedFStage, ErasedShader, ErasedVStage};
pub use resource::{Resource, Resources, UniformBlock, UniformBlockField};
pub use stage::{FArg, FOut, VArg, VOut};
pub use vertex::{
    Attributes, Fragment, FragmentField, Interpolants, InterpolantsField, Vertex, VertexField,
};

use crate::{expose::Expose, Posh};

use self::fields::{Fields, InputFields};

/// Description of a shader.
pub struct Shader<R, V, F> {
    erased: ErasedShader,
    _phantom: PhantomData<(R, V, F)>,
}

impl<R, V, F> Shader<R, V, F>
where
    R: Expose,
    V: Expose,
    F: Expose,
    R::Rep: Resources,
    V::Rep: Attributes,
    F::Rep: Fragment,
{
    pub fn new<W, VStage, FStage>(v_stage: VStage, f_stage: FStage) -> Self
    where
        W: Expose,
        W::Rep: Interpolants,
        VStage: FnOnce(Posh<R>, VArg<V>) -> VOut<W>,
        FStage: FnOnce(Posh<R>, FArg<W>) -> FOut<F>,
    {
        // FIXME: stage arg handling
        let v_out = v_stage(
            <R::Rep as InputFields>::stage_input("res"),
            ErasedVStage::stage_arg(),
        );
        let f_out = f_stage(
            <R::Rep as InputFields>::stage_input("res"),
            ErasedFStage::stage_arg(),
        );

        let res = <R::Rep as Fields>::fields("res");
        let v_stage = ErasedVStage::new::<V, W>(v_out);
        let f_stage = ErasedFStage::new::<W, F>(f_out);

        let erased = ErasedShader {
            res,
            v_stage,
            f_stage,
        };

        Self {
            erased,
            _phantom: PhantomData,
        }
    }

    pub fn erased(&self) -> &ErasedShader {
        &self.erased
    }
}
