#![crate_name = "amethyst_renderer"]
#![crate_type = "lib"]
#![doc(html_logo_url = "http://tinyurl.com/hgsb45k")]

//! High-level rendering engine with multiple backends.

extern crate gfx;

mod backend;
mod frame;
mod frontend;
mod pipeline;

pub use self::frame::{Frame, FrameBuilder};
pub use self::frontend::Renderer;
pub use self::pipeline::{Pipeline, PipelineBuilder, Stage, Step};
