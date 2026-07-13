use crate::subplot::Subplot;
use crate::PltError;

use std::collections::HashMap;

#[cfg(doc)]
use crate::figure::Figure;

/// Defines how and where Subplots are place in a [`Figure`].
pub trait Layout {
    fn subplots(self) -> Vec<(Subplot, FractionalArea)>;
}

/// A [`Layout`] in which a single subplot fills the whole figure.
pub struct SingleLayout {
    subplot: Subplot,
}
impl SingleLayout {
    /// The main constructor, setting the subplot.
    pub fn new(subplot: Subplot) -> Self {
        Self { subplot }
    }
}
impl Layout for SingleLayout {
    fn subplots(self) -> Vec<(Subplot, FractionalArea)> {
        vec![(
            self.subplot,
            FractionalArea { xmin: 0.0, xmax: 1.0, ymin: 0.0, ymax: 1.0},
        )]
    }
}

/// A [`Layout`] in which subplots are placed in a grid orientation in the figure.
pub struct GridLayout {
    row_heights: Vec<f64>,
    col_widths: Vec<f64>,
    subplots: HashMap<(usize, usize), Subplot>,
}
impl GridLayout {
    /// Creates an empty layout.
    pub fn new(nrows: usize, ncols: usize) -> Self {
        Self {
            row_heights: vec![1.0 / nrows as f64; nrows],
            col_widths: vec![1.0 / ncols as f64; ncols],
            subplots: HashMap::new(),
        }
    }
    /// Creates a uniform grid layout from a 2D array, filling only the spots with [`Some`] subplot.
    pub fn from_array<const N: usize, I>(subplots: I) -> Self 
    where
        I: IntoIterator<Item=[Option<Subplot>;N]>,
        <I as IntoIterator>::IntoIter: std::iter::ExactSizeIterator
    {
        let subplots = subplots.into_iter();
        let nrows = subplots.len();
        let ncols = N;

        let hash_iter = subplots.flatten()
            .enumerate()
            .filter_map(|(index, subplot)| {
                let row = index / ncols;
                let col = index % ncols;
                let subplot = subplot?;

                Some(((row, col), subplot))
            });

        Self {
            subplots: HashMap::from_iter(hash_iter),
            row_heights: vec![1.0 / nrows as f64; nrows],
            col_widths: vec![1.0 / ncols as f64; ncols],
        }
    }
    /// Adds or replaces a subplot at the specified location.
    pub fn insert(
        &mut self,
        (row, col): (usize, usize),
        subplot: Subplot,
    ) -> Result<(), PltError> {
        let nrows = self.row_heights.len();
        let ncols = self.col_widths.len();
        if (row + 1) > nrows {
            return Err(PltError::InvalidRow { row, nrows });
        }
        if (col + 1) > ncols {
            return Err(PltError::InvalidColumn { col, ncols });
        }

        self.subplots.insert((row, col), subplot);

        Ok(())
    }
}
impl GridLayout {
    fn subplot_area(&self, (row, col): (usize, usize)) -> Option<FractionalArea> {
        let nrows = self.row_heights.len();
        let ncols = self.col_widths.len();
        if (row + 1) > nrows || (col + 1) > ncols {
            return None
        }

        let xmin = self.col_widths.iter().take(col).sum();
        let xmax = xmin + self.col_widths[col];

        let ymax = 1.0 - self.row_heights.iter().take(row).sum::<f64>();
        let ymin = ymax - self.row_heights.get(row)?;

        Some(FractionalArea { xmin, xmax, ymin, ymax })
    }
}
impl Layout for GridLayout {
    fn subplots(self) -> Vec<(Subplot, FractionalArea)> {
        let areas: Vec<_> = self.subplots.keys()
            .map(|&(row, col)| {
                self.subplot_area((row, col))
                    .expect("all subplot entries should have valid row/col numbers")
            })
            .collect();

        self.subplots.into_values()
            .zip(areas)
            .collect()
    }
}

/// Defines an area of a figure in terms of fractional boundaries.
#[derive(Copy, Clone, Debug)]
pub struct FractionalArea {
    pub xmin: f64,
    pub xmax: f64,
    pub ymin: f64,
    pub ymax: f64,
}
impl FractionalArea {
    pub(crate) fn to_area(self, size: draw::Size) -> draw::Area {
        draw::Area {
            xmin: (self.xmin * size.width as f64).ceil() as u32,
            xmax: (self.xmax * size.width as f64).floor() as u32,
            ymin: (self.ymin * size.height as f64).ceil() as u32,
            ymax: (self.ymax * size.height as f64).floor() as u32,
        }
    }
    pub(crate) fn valid(&self) -> bool {
        self.xmin >= 0.0 && self.xmin <= 1.0
            && self.xmax >= 0.0 && self.xmax <= 1.0
            && self.ymin >= 0.0 && self.ymin <= 1.0
            && self.ymax >= 0.0 && self.ymax <= 1.0
            && self.xmin < self.xmax
            && self.ymin < self.ymax
    }
}
