//! A plotting library with a focus on publication level aesthetics and control.
//!
//! To get started, see the [Examples](https://github.com/plt-rs/plt/tree/main/plt/examples) directory in the main repository.
//!
//! # Basic structure
//! - Plots are drawn on a [`Subplot`].
//! - A [`Figure`], containing one or more subplots, is drawn to a file.

mod figure;
mod subplot;

// bring pub elements from submodules into main lib module
pub use figure::*;
pub use subplot::*;

// re-export necessary elements from plt-draw
pub use draw::{Canvas as Backend, CairoCanvas as CairoBackend, Color, FontName, FileFormat};

/// The error type for this library.
#[derive(thiserror::Error, Debug)]
pub enum PltError {
    /// Returned in the case of input data in an invalid state.
    #[error("Input data is in an invalid state: `{0}`")]
    InvalidData(String),
    /// Returned in the case of a subplot index that is out of bounds.
    #[error("index `{index}` is out of range for figure with {nrows} rows and {ncols} columns")]
    InvalidIndex { index: u32, nrows: u32, ncols: u32 },
    /// Returned when tick mark locations has an unusable value.
    #[error("one or more ticks have invalid locations: `{0}`")]
    BadTickPlacement(String),
    /// Returned when a tick label is not drawable.
    #[error("{0}")]
    BadTickLabels(String),
}
