use crate::{PltError, Color, FontName};

use std::{f64, iter};

/// Describes the configuration of a [`Subplot`].
#[derive(Clone, Debug)]
pub struct SubplotDescriptor<'a> {
    /// The format of this subplot.
    pub format: SubplotFormat,
    /// The title displayed at the top of this subplot.
    pub title: &'a str,
    /// Determines if there is a legend.
    pub legend: bool,
    /// The default axis corresponding to x-values.
    pub xaxis: Axis<&'a str>,
    /// The default axis corresponding to y-values.
    pub yaxis: Axis<&'a str>,
    /// The secondary axis corresponding to x-values.
    pub secondary_xaxis: Axis<&'a str>,
    /// The secondary axis corresponding to y-values.
    pub secondary_yaxis: Axis<&'a str>,
}
impl SubplotDescriptor<'_> {
    /// Constructor for describing a subplot with a high level of detail.
    pub fn detailed() -> Self {
        Self {
            format: SubplotFormat::default(),
            title: "",
            legend: false,
            xaxis: Axis::primary_detailed(),
            yaxis: Axis::primary_detailed(),
            secondary_xaxis: Axis::secondary_detailed(),
            secondary_yaxis: Axis::secondary_detailed(),
        }
    }
}
impl Default for SubplotDescriptor<'_> {
    fn default() -> Self {
        Self {
            format: SubplotFormat::default(),
            title: "",
            legend: false,
            xaxis: Axis::primary_default(),
            yaxis: Axis::primary_default(),
            secondary_xaxis: Axis::secondary_default(),
            secondary_yaxis: Axis::secondary_default(),
        }
    }
}

/// The formatting for a subplot.
#[derive(Clone, Debug)]
pub struct SubplotFormat {
    /// The color used for plotted markers and lines, when there the color cycle is empty.
    pub default_marker_color: Color,
    /// The background color of the plotting area.
    pub plot_color: Color,
    /// The default width of all nonplot lines in the subplot.
    pub line_width: u32,
    /// The default color of all nonplot lines in the subplot.
    pub line_color: Color,
    /// The color of grid lines.
    pub grid_color: Color,
    /// The name of the default font used.
    pub font_name: FontName,
    /// The size of the default font used.
    pub font_size: f32,
    /// The default color of text.
    pub text_color: Color,
    /// The length of major tick marks, from center of the axis, out.
    pub tick_length: u32,
    /// The direction that axis tick marks point.
    pub tick_direction: TickDirection,
    /// Overrides the default length of minor tick marks.
    /// Otherwise computed from [`Self::tick_length`].
    pub override_minor_tick_length: Option<u32>,
    /// The default colors cycled through for plot marker and line colors.
    pub color_cycle: Vec<Color>,
}
impl SubplotFormat {
    /// Constructor for a dark themed format.
    pub fn dark() -> Self {
        let line_color = Color { r: 0.659, g: 0.600, b: 0.518, a: 1.0 };
        let color_cycle = vec![
            Color { r: 0.271, g: 0.522, b: 0.533, a: 1.0 }, // blue
            Color { r: 0.839, g: 0.365, b: 0.055, a: 1.0 }, // orange
            Color { r: 0.596, g: 0.592, b: 0.102, a: 1.0 }, // green
            Color { r: 0.694, g: 0.384, b: 0.525, a: 1.0 }, // purple
            Color { r: 0.800, g: 0.141, b: 0.114, a: 1.0 }, // red
        ];

        Self {
            default_marker_color: line_color,
            plot_color: Color { r: 0.157, g: 0.157, b: 0.157, a: 1.0 },
            grid_color: Color { r: 0.250, g: 0.250, b: 0.250, a: 1.0 },
            line_width: 2,
            line_color,
            font_name: FontName::default(),
            font_size: 20.0,
            text_color: line_color,
            tick_length: 8,
            tick_direction: TickDirection::Inner,
            override_minor_tick_length: None,
            color_cycle,
        }
    }
}
impl Default for SubplotFormat {
    fn default() -> Self {
        let color_cycle = vec![
            Color { r: 0.271, g: 0.522, b: 0.533, a: 1.0 }, // blue
            Color { r: 0.839, g: 0.365, b: 0.055, a: 1.0 }, // orange
            Color { r: 0.596, g: 0.592, b: 0.102, a: 1.0 }, // green
            Color { r: 0.694, g: 0.384, b: 0.525, a: 1.0 }, // purple
            Color { r: 0.800, g: 0.141, b: 0.114, a: 1.0 }, // red
        ];

        Self {
            default_marker_color: Color::BLACK,
            plot_color: Color::TRANSPARENT,
            line_width: 2,
            line_color: Color::BLACK,
            grid_color: Color { r: 0.750, g: 0.750, b: 0.750, a: 1.0 },
            font_name: FontName::default(),
            font_size: 20.0,
            text_color: Color::BLACK,
            tick_length: 8,
            tick_direction: TickDirection::Inner,
            override_minor_tick_length: None,
            color_cycle,
        }
    }
}

/// Indicates which side of the axes ticks should point towards.
#[derive(Copy, Clone, Debug)]
pub enum TickDirection {
    Inner,
    Outer,
    Both,
}

/// Configuration for an axis.
#[derive(Clone, Debug)]
pub struct Axis<S: AsRef<str>> {
    /// The label desplayed by the axis.
    pub label: S,
    /// Determines the major tick mark locations and labels on this axis.
    pub major_ticks: Ticker,
    /// Determines the minor tick mark locations and labels on this axis.
    pub minor_ticks: Ticker,
    /// Sets which, if any, tick marks on this axis have grid lines.
    pub grid: Grid,
    /// How the maximum and minimum plotted values should be set.
    pub limits: Limits,
}
impl<S: AsRef<str> + Default> Axis<S> {
    pub(crate) fn primary_default() -> Self {
        Self {
            label: S::default(),
            major_ticks: Ticker::auto(),
            minor_ticks: Ticker::null(),
            grid: Grid::None,
            limits: Limits::Auto,
        }
    }
    pub(crate) fn primary_detailed() -> Self {
        Self {
            label: S::default(),
            major_ticks: Ticker::auto(),
            minor_ticks: Ticker::auto()
                .with_labels(&[]),
            grid: Grid::Major,
            limits: Limits::Auto,
        }
    }
    pub(crate) fn secondary_default() -> Self {
        Self {
            label: S::default(),
            major_ticks: Ticker::null(),
            minor_ticks: Ticker::null(),
            grid: Grid::None,
            limits: Limits::Auto,
        }
    }
    pub(crate) fn secondary_detailed() -> Self {
        Self {
            label: S::default(),
            major_ticks: Ticker::auto()
                .with_labels(&[]),
            minor_ticks: Ticker::auto()
                .with_labels(&[]),
            grid: Grid::None,
            limits: Limits::Auto,
        }
    }
}

/// Defines how axis tick marks and tick labels should be created.
#[derive(Clone, Debug)]
pub struct Ticker {
    pub(crate) spacing: TickSpacing,
    pub(crate) labels: TickLabels,
}
impl Ticker {
    /// Tick marks and labels are determined by the library.
    pub fn auto() -> Self {
        Self {
            spacing: TickSpacing::Auto,
            labels: TickLabels::Auto,
        }
    }
    /// A set number of tick marks and labels are spaced linearly.
    pub fn linear(count: u16) -> Self {
        Self {
            spacing: TickSpacing::Count(count),
            labels: TickLabels::Auto,
        }
    }
    /// No tick marks.
    pub fn null() -> Self {
        Self {
            spacing: TickSpacing::None,
            labels: TickLabels::None,
        }
    }
    /// Manually set tick marks.
    pub fn manual(ticks: &[f64]) -> Self {
        Self {
            spacing: TickSpacing::Manual(ticks.to_vec()),
            labels: TickLabels::Auto,
        }
    }
    /// A builder like modifier for manually setting labels.
    pub fn with_labels(mut self, labels: &[String]) -> Self {
        self.labels = TickLabels::Manual {
            labels: labels.to_vec(),
            multiplier: 0,
            offset: 0.0,
        };

        self
    }
}

/// Indicates which, if any, tick marks on an axis should have grid lines.
#[derive(Copy, Clone, Debug)]
pub enum Grid {
    /// Grid lines extend from only the major tick marks.
    Major,
    /// Grid lines extend from the major and minor tick marks.
    Full,
    /// No Grid lines from this axis.
    None,
}

/// How the maximum and minimum plotted values of an axis should be set.
#[derive(Copy, Clone, Debug)]
pub enum Limits {
    /// Limits are determined by the library.
    Auto,
    /// Limits are set manually.
    Manual { min: f64, max: f64 },
}

/// Describes data and how it should be plotted.
#[derive(Clone, Debug)]
pub struct PlotDescriptor<'a, D: SeriesData> {
    /// The label corresponding to this data, displayed in a legend.
    pub label: &'a str,
    /// The data to be plotted.
    pub data: D,
    /// The format of lines, optionally drawn between data points.
    pub line: Option<Line>,
    /// The format of markers, optionally drawn at data points.
    pub marker: Option<Marker>,
}
impl<'a, D: SeriesData> PlotDescriptor<'a, D> {
    /// Creates a plot descriptor from just data with other values defaulted.
    pub fn from_data(data: D) -> Self {
        Self {
            label: "",
            data,
            line: Some(Line::default()),
            marker: None,
        }
    }
}
impl<D: SeriesData + Default> Default for PlotDescriptor<'_, D> {
    fn default() -> Self {
        Self {
            label: "",
            data: D::default(),
            line: Some(Line::default()),
            marker: None,
        }
    }
}

/// Holds borrowed data to be plotted.
#[derive(Copy, Clone, Debug)]
pub struct PlotData<'a> {
    xdata: ndarray::ArrayView1<'a, f64>,
    ydata: ndarray::ArrayView1<'a, f64>,
}
impl Default for PlotData<'_> {
    fn default() -> Self {
        Self {
            xdata: ndarray::ArrayView1::<f64>::from(&[]),
            ydata: ndarray::ArrayView1::<f64>::from(&[]),
        }
    }
}
impl SeriesData for PlotData<'_> {
    fn data<'b>(&'b self) -> Box<dyn Iterator<Item = (f64, f64)> + 'b> {
        Box::new(
            iter::zip(self.xdata.iter().cloned(), self.ydata.iter().cloned())
        )
    }

    fn xmin(&self) -> f64 {
        self.xdata.iter().fold(f64::INFINITY, |a, &b| a.min(b))
    }
    fn xmax(&self) -> f64 {
        self.xdata.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b))
    }
    fn ymin(&self) -> f64 {
        self.ydata.iter().fold(f64::INFINITY, |a, &b| a.min(b))
    }
    fn ymax(&self) -> f64 {
        self.ydata.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b))
    }
}
impl<'a> PlotData<'a> {
    /// Main constructor, taking separate array views of x-values and y-values.
    pub fn new<
        Xs: Into<ndarray::ArrayView1<'a, f64>>,
        Ys: Into<ndarray::ArrayView1<'a, f64>>,
    >(xs: Xs, ys: Ys) -> Result<Self, PltError> {
        let xdata = xs.into();
        let ydata = ys.into();

        // check that data is valid
        if xdata.len() != ydata.len() {
            return Err(PltError::InvalidData("X and Y data are different lengths".to_owned()))
        } else if xdata.iter().any(|&v| v.is_nan()) {
            return Err(PltError::InvalidData("X data has a NaN value".to_owned()))
        } else if ydata.iter().any(|&v| v.is_nan()) {
            return Err(PltError::InvalidData("Y data has a NaN value".to_owned()))
        }

        Ok(Self {
            xdata,
            ydata,
        })
    }
}

/// Holds owned data to be plotted.
#[derive(Clone, Debug)]
pub struct PlotDataOwned {
    xdata: ndarray::Array1<f64>,
    ydata: ndarray::Array1<f64>,
}
impl Default for PlotDataOwned {
    fn default() -> Self {
        Self {
            xdata: ndarray::Array1::<f64>::default(0),
            ydata: ndarray::Array1::<f64>::default(0),
        }
    }
}
impl SeriesData for PlotDataOwned {
    fn data(&self) -> Box<dyn Iterator<Item = (f64, f64)> + '_> {
        Box::new(
            iter::zip(self.xdata.iter().cloned(), self.ydata.iter().cloned())
        )
    }

    fn xmin(&self) -> f64 {
        self.xdata.iter().fold(f64::INFINITY, |a, &b| a.min(b))
    }
    fn xmax(&self) -> f64 {
        self.xdata.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b))
    }
    fn ymin(&self) -> f64 {
        self.ydata.iter().fold(f64::INFINITY, |a, &b| a.min(b))
    }
    fn ymax(&self) -> f64 {
        self.ydata.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b))
    }
}
impl PlotDataOwned {
    /// Main constructor, taking separate arrays of x-values and y-values.
    pub fn new<
        Xs: Into<ndarray::Array1<f64>>,
        Ys: Into<ndarray::Array1<f64>>,
    >(xs: Xs, ys: Ys) -> Result<Self, PltError> {
        let xdata = xs.into();
        let ydata = ys.into();

        // check that data is valid
        if xdata.len() != ydata.len() {
            return Err(PltError::InvalidData("X and Y data are different lengths".to_owned()))
        } else if xdata.iter().any(|&v| v.is_nan()) {
            return Err(PltError::InvalidData("X data has a NaN value".to_owned()))
        } else if ydata.iter().any(|&v| v.is_nan()) {
            return Err(PltError::InvalidData("Y data has a NaN value".to_owned()))
        }

        Ok(Self {
            xdata,
            ydata,
        })
    }
}

/// Holds borrowed step data to be plotted.
#[derive(Copy, Clone, Debug)]
pub struct StepData<'a> {
    edges: ndarray::ArrayView1<'a, f64>,
    ydata: ndarray::ArrayView1<'a, f64>,
}
impl Default for StepData<'_> {
    fn default() -> Self {
        Self {
            edges: ndarray::ArrayView1::<f64>::from(&[]),
            ydata: ndarray::ArrayView1::<f64>::from(&[]),
        }
    }
}
impl SeriesData for StepData<'_> {
    fn data<'b>(&'b self) -> Box<dyn Iterator<Item = (f64, f64)> + 'b> {
        Box::new(iter::zip(
            self.edges.windows(2).into_iter().flatten().cloned(),
            self.ydata.iter().map(|y| [y, y]).flatten().cloned(),
        ))
    }

    fn xmin(&self) -> f64 {
        self.edges.iter().fold(f64::INFINITY, |a, &b| a.min(b))
    }
    fn xmax(&self) -> f64 {
        self.edges.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b))
    }
    fn ymin(&self) -> f64 {
        self.ydata.iter().fold(f64::INFINITY, |a, &b| a.min(b))
    }
    fn ymax(&self) -> f64 {
        self.ydata.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b))
    }
}
impl<'a> StepData<'a> {
    /// Main constructor, taking separate array views of steps and y-values.
    pub fn new<
        Es: Into<ndarray::ArrayView1<'a, f64>>,
        Ys: Into<ndarray::ArrayView1<'a, f64>>,
    >(edges: Es, ys: Ys) -> Result<Self, PltError> {
        let edges = edges.into();
        let ydata = ys.into();

        // check that data is valid
        if edges.len() != (ydata.len() + 1) {
            return Err(PltError::InvalidData("X and Y data are different lengths".to_owned()))
        } else if edges.iter().any(|&v| v.is_nan()) {
            return Err(PltError::InvalidData("X data has a NaN value".to_owned()))
        } else if ydata.iter().any(|&v| v.is_nan()) {
            return Err(PltError::InvalidData("Y data has a NaN value".to_owned()))
        }

        Ok(Self {
            edges,
            ydata,
        })
    }
}

/// Holds owned data to be plotted.
#[derive(Clone, Debug)]
pub struct StepDataOwned {
    edges: ndarray::Array1<f64>,
    ydata: ndarray::Array1<f64>,
}
impl Default for StepDataOwned {
    fn default() -> Self {
        Self {
            edges: ndarray::Array1::<f64>::default(0),
            ydata: ndarray::Array1::<f64>::default(0),
        }
    }
}
impl SeriesData for StepDataOwned {
    fn data(&self) -> Box<dyn Iterator<Item = (f64, f64)> + '_> {
        Box::new(iter::zip(
            self.edges.windows(2).into_iter().flatten().cloned(),
            self.ydata.iter().map(|y| [y, y]).flatten().cloned(),
        ))
    }

    fn xmin(&self) -> f64 {
        self.edges.iter().fold(f64::INFINITY, |a, &b| a.min(b))
    }
    fn xmax(&self) -> f64 {
        self.edges.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b))
    }
    fn ymin(&self) -> f64 {
        self.ydata.iter().fold(f64::INFINITY, |a, &b| a.min(b))
    }
    fn ymax(&self) -> f64 {
        self.ydata.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b))
    }
}
impl StepDataOwned {
    /// Main constructor, taking separate arrays of steps and y-values.
    pub fn new<
        Es: Into<ndarray::Array1<f64>>,
        Ys: Into<ndarray::Array1<f64>>,
    >(edges: Es, ys: Ys) -> Result<Self, PltError> {
        let edges = edges.into();
        let ydata = ys.into();

        // check that data is valid
        if edges.len() != (ydata.len() + 1) {
            return Err(PltError::InvalidData("X and Y data are different lengths".to_owned()))
        } else if edges.iter().any(|&v| v.is_nan()) {
            return Err(PltError::InvalidData("X data has a NaN value".to_owned()))
        } else if ydata.iter().any(|&v| v.is_nan()) {
            return Err(PltError::InvalidData("Y data has a NaN value".to_owned()))
        }

        Ok(Self {
            edges,
            ydata,
        })
    }
}

/// Format for lines plotted between data points.
#[derive(Copy, Clone, Debug)]
pub struct Line {
    /// The style of line drawn.
    pub style: LineStyle,
    /// The width of the line.
    pub width: u32,
    /// Optionally overrides the default color of the line.
    pub color_override: Option<Color>,
}
impl Default for Line {
    fn default() -> Self {
        Self {
            style: LineStyle::Solid,
            width: 3,
            color_override: None,
        }
    }
}

/// Plotting line styles.
#[derive(Copy, Clone, Debug)]
pub enum LineStyle {
    /// A solid line.
    Solid,
    /// A dashed line with regular sized dashes.
    Dashed,
    /// A dashed line with short dashes.
    ShortDashed,
}

/// Format for markers drawn at data points.
#[derive(Clone, Debug)]
pub struct Marker {
    /// The shape of the marker.
    pub style: MarkerStyle,
    /// The size of the marker.
    pub size: u32,
    /// Optionally overrides the default fill color of the marker.
    pub color_override: Option<Color>,
    /// Optionally adds an outline.
    pub outline: Option<Line>,
}
impl Default for Marker {
    fn default() -> Self {
        Self {
            style: MarkerStyle::Circle,
            size: 3,
            color_override: None,
            outline: None,
        }
    }
}

/// Marker shapes.
#[derive(Copy, Clone, Debug)]
pub enum MarkerStyle {
    /// A circular marker.
    Circle,
    /// A square marker.
    Square,
}

/// The object that represents a whole subplot and is used to draw plotted data.
#[derive(Debug)]
pub struct Subplot<'a> {
    pub(crate) format: SubplotFormat,
    pub(crate) plot_infos: Vec<PlotInfo<'a>>,
    pub(crate) title: String,
    pub(crate) xaxis: AxisBuf,
    pub(crate) yaxis: AxisBuf,
    pub(crate) secondary_xaxis: AxisBuf,
    pub(crate) secondary_yaxis: AxisBuf,
    pub(crate) primary_xaxis_id: AxisType,
    pub(crate) primary_yaxis_id: AxisType,
}
impl<'a> Subplot<'a> {
    /// The main constructor.
    pub fn new(desc: &SubplotDescriptor) -> Self {
        Self {
            format: desc.format.clone(),
            plot_infos: vec![],
            title: desc.title.to_string(),
            xaxis: desc.xaxis.to_buf(),
            yaxis: desc.yaxis.to_buf(),
            secondary_xaxis: desc.secondary_xaxis.to_buf(),
            secondary_yaxis: desc.secondary_yaxis.to_buf(),
            primary_xaxis_id: AxisType::X,
            primary_yaxis_id: AxisType::Y,
        }
    }

    /// Plots x, y data points.
    pub fn plot<D: SeriesData + 'a>(&mut self, desc: PlotDescriptor<'a, D>) {
        self.plot_infos.push(
            PlotInfo {
                label: desc.label.to_string(),
                data: Box::new(desc.data),
                line: desc.line,
                marker: desc.marker,
                xaxis: self.primary_xaxis_id,
                yaxis: self.primary_yaxis_id,
            }
        );
    }

    /// Returns the format of this plot.
    pub fn format(&self) -> &SubplotFormat {
        &self.format
    }

    /// Switch y-axis used by plotting calls after this point to the secondary y-axis.
    pub fn use_secondary_yaxis(&mut self) {
        self.primary_yaxis_id = AxisType::SecondaryY;
    }

    /// Switch y-axis used by plotting calls after this point to the default y-axis.
    pub fn use_default_yaxis(&mut self) {
        self.primary_yaxis_id = AxisType::Y;
    }
}

// traits

/// Implemented for data that can be represented by pairs of floats to be plotted.
pub trait SeriesData: std::fmt::Debug {
    /// Returns data in an [`Iterator`] over x, y pairs.
    fn data<'a>(&'a self) -> Box<dyn Iterator<Item = (f64, f64)> + 'a>;
    /// The smallest x-value.
    fn xmin(&self) -> f64;
    /// The largest x-value.
    fn xmax(&self) -> f64;
    /// The smallest y-value.
    fn ymin(&self) -> f64;
    /// The largest y-value.
    fn ymax(&self) -> f64;
}

// private

#[derive(Clone, Debug)]
pub(crate) enum TickSpacing {
    Auto,
    Count(u16),
    None,
    Manual(Vec<f64>),
}

#[derive(Clone, Debug)]
pub(crate) enum TickLabels {
    Auto,
    None,
    Manual { labels: Vec<String>, multiplier: i32, offset: f64 },
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub(crate) enum AxisType {
    X,
    Y,
    SecondaryX,
    SecondaryY,
}
impl AxisType {
    pub(crate) fn iter() -> std::array::IntoIter<Self, 4> {
        [
            Self::X,
            Self::Y,
            Self::SecondaryX,
            Self::SecondaryY,
        ].into_iter()
    }
}

pub(crate) type AxisBuf = Axis<String>;
impl<S: AsRef<str>> Axis<S> {
    fn to_buf(&self) -> AxisBuf {
        AxisBuf {
            label: self.label.as_ref().to_string(),
            major_ticks: self.major_ticks.clone(),
            minor_ticks: self.minor_ticks.clone(),
            grid: self.grid,
            limits: self.limits,
        }
    }
}

#[derive(Debug)]
pub(crate) struct PlotInfo<'a> {
    //TODO implement legend
    #[allow(dead_code)]
    pub label: String,
    pub data: Box<dyn SeriesData + 'a>,
    pub line: Option<Line>,
    pub marker: Option<Marker>,
    pub xaxis: AxisType,
    pub yaxis: AxisType,
}
