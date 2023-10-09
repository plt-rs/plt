use crate::subplot::{Subplot, SubplotDescriptor};
use crate::PltError;

#[cfg(doc)]
use crate::figure::Figure;

/// Defines how and where Subplots are place in a [`Figure`].
pub trait Layout<'a> {
    fn subplots(self) -> Vec<(Subplot<'a>, FractionalArea)>;
}

/// A [`Layout`] in which a single subplot fills the whole figure.
pub struct SingleLayout<'a> {
    subplot: Subplot<'a>,
}
impl<'a> SingleLayout<'a> {
    /// The main constructor, setting the subplot.
    pub fn new(subplot: Subplot<'a>) -> Self {
        Self { subplot }
    }
}
impl<'a> Layout<'a> for SingleLayout<'a> {
    fn subplots(self) -> Vec<(Subplot<'a>, FractionalArea)> {
        vec![(
            self.subplot,
            FractionalArea { xmin: 0.0, xmax: 1.0, ymin: 0.0, ymax: 1.0},
        )]
    }
}

/// A [`Layout`] in which subplots are placed in a grid orientation in the figure.
pub struct GridLayout<'a> {
    subplots: ndarray::Array2<Subplot<'a>>,
    areas: ndarray::Array2<FractionalArea>,
    mask: ndarray::Array2<bool>,
}
impl<'a> GridLayout<'a> {
    /// Creates an empty layout.
    pub fn new(nrows: usize, ncols: usize) -> Self {
        let xextent = 1.0 / ncols as f64;
        let yextent = 1.0 / nrows as f64;
        let areas = (0..(nrows * ncols))
            .map(|index| {
                // get row and column indices
                let row = index / ncols;
                let col = index % ncols;

                let xmin = xextent * col as f64;
                let xmax = xmin + xextent;
                let ymin = yextent * (nrows - 1 - row) as f64;
                let ymax = ymin + yextent;

                FractionalArea { xmin, xmax, ymin, ymax }
            })
            .collect::<ndarray::Array1<_>>();
        let areas = areas.into_shape((nrows, ncols)).unwrap();

        Self {
            subplots: ndarray::Array2::from_elem(
                (nrows, ncols),
                Subplot::new(&SubplotDescriptor::default()),
            ),
            areas,
            mask: ndarray::Array2::from_elem((nrows, ncols), false),
        }
    }
    /// Creates a uniform grid layout from a 2D array, filling only the spots with [`Some`] subplot.
    pub fn from_array<A: Into<ndarray::Array2<Option<Subplot<'a>>>>>(subplots: A) -> Self {
        let subplots = subplots.into();

        let nrows = subplots.nrows();
        let ncols = subplots.ncols();

        let xextent = 1.0 / ncols as f64;
        let yextent = 1.0 / nrows as f64;
        let areas = (0..(nrows * ncols))
            .map(|index| {
                // get row and column indices
                let row = index / ncols;
                let col = index % ncols;

                let xmin = xextent * col as f64;
                let xmax = xmin + xextent;
                let ymin = yextent * (nrows - 1 - row) as f64;
                let ymax = ymin + yextent;

                FractionalArea { xmin, xmax, ymin, ymax }
            })
            .collect::<ndarray::Array1<_>>();
        let areas = areas.into_shape((nrows, ncols)).unwrap();

        let mask = subplots.map(|subplot| subplot.is_some());
        let subplots = subplots.mapv(|subplot| {
            subplot.unwrap_or_else(|| Subplot::new(&SubplotDescriptor::default()))
        });

        Self {
            subplots,
            areas,
            mask,
        }
    }
    /// Adds or replaces a subplot at the specified location.
    pub fn insert(
        &mut self,
        (row, col): (usize, usize),
        subplot: Subplot<'a>,
    ) -> Result<(), PltError> {
        if (row + 1) > self.subplots.nrows() {
            return Err(PltError::InvalidRow { row, nrows: self.subplots.nrows() });
        }
        if (col + 1) > self.subplots.ncols() {
            return Err(PltError::InvalidColumn { col, ncols: self.subplots.ncols() });
        }

        self.subplots[[row, col]] = subplot;
        self.mask[[row, col]] = true;

        Ok(())
    }
}
impl<'a> Layout<'a> for GridLayout<'a> {
    fn subplots(self) -> Vec<(Subplot<'a>, FractionalArea)> {
        Iterator::zip(
            self.subplots.indexed_iter().filter_map(|(index, subplot)|
                if self.mask[index] { Some(subplot) } else { None }
            ).cloned(),
            self.areas.indexed_iter().filter_map(|(index, area)|
                if self.mask[index] { Some(area) } else { None }
            ).cloned(),
        ).collect()
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
