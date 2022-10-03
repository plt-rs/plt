//! A plotting library with a focus on publication level aesthetics and ergonomic control.
//!
//! # Structure
//! - Plots are drawn on a [`Subplot`].
//! - One or more subplots are organized in a [`Layout`].
//! - The layout is added to a [`Figure`], which is used to draw to a file or directly to a [`backend::Canvas`].
//!
//! # Use
//!
//! To get started, see the [examples](https://github.com/plt-rs/plt/tree/main/plt/examples) directory in the main repository.
//!
//! ### Example
//! ```rust
//!# use plt::*;
//!// create data
//!let xs: Vec<f64> = (0..=100).map(|n: u32| n as f64 * 0.1).collect();
//!let ys: Vec<f64> = xs.iter().map(|x| x.powi(3)).collect();
//!
//!// create subplot
//!let mut sp = Subplot::builder()
//!    .label(Axes::X, "x data")
//!    .label(Axes::Y, "y data")
//!    .build();
//!
//!// plot data
//!sp.plot(&xs, &ys).unwrap();
//!
//!// make figure and add subplot
//!let mut fig = <Figure>::default();
//!fig.set_layout(SingleLayout::new(sp)).unwrap();
//!
//!// save figure to file
//!fig.draw_file(FileFormat::Png, "example.png").unwrap();
//! ```
//!
//! # Dependencies
//!
//! The package currently depends on [Cairo](https://www.cairographics.org).
//!
//! ### Debian / Ubuntu
//! `apt install libcairo2-dev`
//!
//! ### Arch
//! `pacman -Syu cairo`

mod figure;
mod layout;
mod subplot;

// bring pub elements from submodules into main lib module
pub use figure::*;
pub use layout::*;
pub use subplot::*;

// re-export necessary elements from plt-draw
pub use draw::{Color, FileFormat, FontName};

// re-export backend canvas in separate module
/// Re-exports of neccessary plt-draw backend elements.
pub mod backend {
    pub use draw::Canvas;
    #[cfg(feature = "cairo")]
    pub use draw_cairo::CairoCanvas;
}

/// The error type for this library.
#[derive(thiserror::Error, Debug)]
pub enum PltError {
    /// Returned in the case of input data in an invalid state.
    #[error("Input data is in an invalid state: `{0}`")]
    InvalidData(String),
    /// Returned in the case of a subplot index that is out of bounds.
    #[error("index `{index}` is out of range for figure with {nrows} rows and {ncols} columns")]
    InvalidIndex { index: u32, nrows: u32, ncols: u32 },
    /// Returned in the case of a subplot index that is out of bounds.
    #[error("row index `{row}` is out of range for layout with {nrows} rows")]
    InvalidRow { row: usize, nrows: usize },
    /// Returned in the case of a subplot index that is out of bounds.
    #[error("column index `{col}` is out of range for layout with {ncols} columns")]
    InvalidColumn { col: usize, ncols: usize },
    /// Returned when tick mark locations has an unusable value.
    #[error("one or more ticks have invalid locations: `{0}`")]
    BadTickPlacement(String),
    /// Returned when a tick label is not drawable.
    #[error("{0}")]
    BadTickLabels(String),
    /// Returned when the provided area of a subplot is not valid.
    #[error("{0:?} is not a valid fractional area")]
    InvalidSubplotArea(layout::FractionalArea),
    /// Returned when the drawing backend returns an error.
    #[error(transparent)]
    DrawError(#[from] draw::DrawError)
}
