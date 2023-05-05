use std::marker::PhantomData;

use crate::{
    interface::FragmentVisitor,
    sl::{self, ColorSample},
    FsInterface, Gl, Sl,
};

use super::{
    raw::{self},
    ColorSampler2d, Sampler2dSettings,
};

#[derive(Clone)]
pub struct ColorAttachment<S = sl::Vec4> {
    raw: raw::Attachment,
    _phantom: PhantomData<S>,
}

impl<S> ColorAttachment<S> {
    pub(super) fn from_raw(raw: raw::Attachment) -> Self {
        Self {
            raw,
            _phantom: PhantomData,
        }
    }

    pub fn as_color_sampler(&self, settings: Sampler2dSettings) -> ColorSampler2d<S> {
        ColorSampler2d::from_raw(self.raw.sampler(settings, None))
    }
}

impl<'a, S: ColorSample> From<&'a ColorAttachment<S>> for Framebuffer<S> {
    fn from(value: &'a ColorAttachment<S>) -> Self {
        Framebuffer(FramebufferInternal::Color(value.clone()))
    }
}

impl<S: ColorSample> From<ColorAttachment<S>> for Framebuffer<S> {
    fn from(value: ColorAttachment<S>) -> Self {
        Framebuffer(FramebufferInternal::Color(value))
    }
}

#[derive(Clone)]
pub struct DepthAttachment {
    raw: raw::Attachment,
}

impl DepthAttachment {
    pub(super) fn from_raw(raw: raw::Attachment) -> Self {
        Self { raw }
    }
}

impl<'a> From<&'a DepthAttachment> for Framebuffer<()> {
    fn from(value: &'a DepthAttachment) -> Self {
        Framebuffer(FramebufferInternal::Depth(value.clone()))
    }
}

impl From<DepthAttachment> for Framebuffer<()> {
    fn from(value: DepthAttachment) -> Self {
        Framebuffer(FramebufferInternal::Depth(value))
    }
}

#[derive(Clone)]
enum FramebufferInternal<F: FsInterface<Sl>> {
    Default,
    Depth(DepthAttachment),
    Color(F::Gl),
    ColorDepth {
        color: F::Gl,
        depth: DepthAttachment,
    },
}

#[derive(Clone)]
pub struct Framebuffer<F: FsInterface<Sl> = sl::Vec4>(FramebufferInternal<F>);

impl<F: FsInterface<Sl>> Framebuffer<F> {
    pub fn color(color: &F::Gl) -> Self {
        Framebuffer(FramebufferInternal::Color(color.clone()))
    }

    pub fn color_and_depth(color: &F::Gl, depth: &DepthAttachment) -> Self {
        Framebuffer(FramebufferInternal::ColorDepth {
            color: color.clone(),
            depth: depth.clone(),
        })
    }

    pub fn raw(&self) -> raw::Framebuffer {
        use FramebufferInternal::*;

        match &self.0 {
            Default => raw::Framebuffer::Default,
            Depth(depth) => raw::Framebuffer::Attachments {
                attachments: vec![depth.raw.clone()],
            },
            Color(color) => raw::Framebuffer::Attachments {
                attachments: raw_color_attachments(color),
            },
            ColorDepth { color, depth } => {
                let mut attachments = raw_color_attachments(color);
                attachments.push(depth.raw.clone());
                raw::Framebuffer::Attachments { attachments }
            }
        }
    }
}

impl Framebuffer<()> {
    pub fn depth(depth: &DepthAttachment) -> Self {
        Framebuffer(FramebufferInternal::Depth(depth.clone()))
    }
}

impl Default for Framebuffer<sl::Vec4> {
    fn default() -> Self {
        Framebuffer(FramebufferInternal::Default)
    }
}

impl<'a, F: FsInterface<Sl>> From<&'a Framebuffer<F>> for Framebuffer<F> {
    fn from(value: &'a Framebuffer<F>) -> Self {
        value.clone()
    }
}

fn raw_color_attachments<F: FsInterface<Gl>>(attachments: &F) -> Vec<raw::Attachment> {
    struct Visitor(Vec<raw::Attachment>);

    impl<'a> FragmentVisitor<'a, Gl> for Visitor {
        fn accept<S: ColorSample>(&mut self, _: &str, attachment: &ColorAttachment<S>) {
            self.0.push(attachment.raw.clone());
        }
    }

    // TODO: Remove hardcoded path names.
    let mut visitor = Visitor(Vec::new());
    attachments.visit("fragment_output", &mut visitor);

    visitor.0
}
