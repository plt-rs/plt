use crate::{Color, FontName, PltError};

use std::{array, fmt::{self, Formatter}, f64, iter};

/// The object that represents a whole subplot and is used to draw plotted data.
#[derive(Clone, Debug)]
pub struct Subplot<'a> {
    pub(crate) format: SubplotFormat,
    pub(crate) plot_order: Vec<PlotType>,
    pub(crate) plot_infos: Vec<PlotInfo<'a>>,
    pub(crate) fill_infos: Vec<FillInfo<'a>>,
    pub(crate) title: String,
    pub(crate) xaxis: AxisBuf,
    pub(crate) yaxis: AxisBuf,
    pub(crate) secondary_xaxis: AxisBuf,
    pub(crate) secondary_yaxis: AxisBuf,
}
impl<'a> Subplot<'a> {
    /// Returns a builder with default settings for constructing a subplot.
    pub fn builder() -> SubplotBuilder<'a> {
        SubplotBuilder { desc: SubplotDescriptor::default() }
    }

    /// Returns a [`Plotter`] for plotting X, Y data on this subplot.
    pub fn plotter<'b>(&'b mut self) -> Plotter<'a, 'b> {
        Plotter {
            subplot: self,
            desc: PlotDescriptor::default(),
        }
    }

    /// Returns a [`Filler`] for filling a region of the subplot with a color.
    pub fn filler<'b>(&'b mut self) -> Filler<'a, 'b> {
        Filler {
            subplot: self,
            desc: FillDescriptor::default(),
        }
    }

    /// Plots X, Y data on this subplot with default plot formatting.
    /// Shortcut for calling `.plotter().plot()` on a [`Subplot`].
    pub fn plot<Xs, Ys, Fx, Fy>(
        &mut self,
        xs: Xs,
        ys: Ys,
    ) -> Result<(), PltError>
    where
        Fx: IntoF64,
        Fy: IntoF64,
        Xs: IntoIterator<Item=Fx>,
        Ys: IntoIterator<Item=Fy>,
        <Xs as IntoIterator>::IntoIter: iter::ExactSizeIterator + Clone + 'a,
        <Ys as IntoIterator>::IntoIter: iter::ExactSizeIterator + Clone + 'a,
    {
        let plotter = Plotter {
            subplot: self,
            desc: PlotDescriptor::default(),
        };

        plotter.plot(xs, ys)
    }

    /// Plots step plot data on this subplot with default plot formatting.
    /// Shortcut for calling `.plotter().step()` on a [`Subplot`].
    pub fn step<Xs, Ys, Fx, Fy>(
        &mut self,
        steps: Xs,
        ys: Ys,
    ) -> Result<(), PltError>
    where
        Fx: IntoF64,
        Fy: IntoF64,
        Xs: IntoIterator<Item=Fx>,
        Ys: IntoIterator<Item=Fy>,
        <Xs as IntoIterator>::IntoIter: iter::ExactSizeIterator + Clone + 'a,
        <Ys as IntoIterator>::IntoIter: iter::ExactSizeIterator + Clone + 'a,
    {
        let plotter = Plotter {
            subplot: self,
            desc: PlotDescriptor::default(),
        };

        plotter.step(steps, ys)
    }

    /// Fills an area between two curves on the subplot with default formatting.
    /// Shortcut for calling `.filler().fill_between()` on a [`Subplot`].
    pub fn fill_between<Xs, Y1s, Y2s, Fx, Fy1, Fy2>(
        &mut self,
        xs: Xs,
        y1s: Y1s,
        y2s: Y2s,
    ) -> Result<(), PltError>
    where
        Fx: IntoF64,
        Fy1: IntoF64,
        Fy2: IntoF64,
        Xs: IntoIterator<Item=Fx>,
        Y1s: IntoIterator<Item=Fy1>,
        Y2s: IntoIterator<Item=Fy2>,
        <Xs as IntoIterator>::IntoIter: iter::ExactSizeIterator
            + iter::DoubleEndedIterator + Clone + 'a,
        <Y1s as IntoIterator>::IntoIter: iter::ExactSizeIterator
            + iter::DoubleEndedIterator + Clone + 'a,
        <Y2s as IntoIterator>::IntoIter: iter::ExactSizeIterator
            + iter::DoubleEndedIterator + Clone + 'a,
    {
        let filler = Filler {
            subplot: self,
            desc: FillDescriptor::default(),
        };

        filler.fill_between(xs, y1s, y2s)
    }

    /// Returns the format of this plot.
    pub fn format(&self) -> &SubplotFormat {
        &self.format
    }
}
impl<'a> Subplot<'a> {
    /// Internal constructor.
    pub(crate) fn new(desc: &SubplotDescriptor) -> Self {
        Self {
            format: desc.format.clone(),
            plot_order: vec![],
            plot_infos: vec![],
            fill_infos: vec![],
            title: desc.title.to_string(),
            xaxis: desc.xaxis.to_buf(),
            yaxis: desc.yaxis.to_buf(),
            secondary_xaxis: desc.secondary_xaxis.to_buf(),
            secondary_yaxis: desc.secondary_yaxis.to_buf(),
        }
    }
}
impl<'a> Subplot<'a> {
    /// Internal plot setup function.
    fn plot_desc<D: SeriesData + Clone + 'a>(
        &mut self,
        desc: PlotDescriptor,
        data: D,
    ) {
        let line = if desc.line {
            Some(desc.line_format)
        } else {
            None
        };
        let marker = if desc.marker {
            Some(desc.marker_format)
        } else {
            None
        };

        let xaxis = match desc.xaxis {
            AxisType::X => &mut self.xaxis,
            AxisType::Y => &mut self.yaxis,
            AxisType::SecondaryX => &mut self.secondary_xaxis,
            AxisType::SecondaryY => &mut self.secondary_yaxis,
        };
        match xaxis.limit_policy {
            Limits::Auto => {
                // span
                xaxis.span = if let Some((xmin, xmax)) = xaxis.span {
                    Some((f64::min(xmin, data.xmin()), f64::max(xmax, data.xmax())))
                } else {
                    Some((data.xmin(), data.xmax()))
                };

                // limits
                let (xmin, xmax) = xaxis.span.unwrap();
                let extent = xmax - xmin;
                xaxis.limits = if extent > 0.0 {
                    Some((xmin - 0.05 * extent, xmax + 0.05 * extent))
                } else {
                    Some((xmin - 1.0, xmax + 1.0))
                };
            },
            Limits::Manual { min: _, max: _ } => {},
        };

        let yaxis = match desc.yaxis {
            AxisType::X => &mut self.xaxis,
            AxisType::Y => &mut self.yaxis,
            AxisType::SecondaryX => &mut self.secondary_xaxis,
            AxisType::SecondaryY => &mut self.secondary_yaxis,
        };
        match yaxis.limit_policy {
            Limits::Auto => {
                // span
                yaxis.span = if let Some((ymin, ymax)) = yaxis.span {
                    Some((f64::min(ymin, data.ymin()), f64::max(ymax, data.ymax())))
                } else {
                    Some((data.ymin(), data.ymax()))
                };

                // limits
                let (ymin, ymax) = yaxis.span.unwrap();
                let extent = ymax - ymin;
                yaxis.limits = if extent > 0.0 {
                    Some((ymin - 0.05 * extent, ymax + 0.05 * extent))
                } else {
                    Some((ymin - 1.0, ymax + 1.0))
                };
            },
            Limits::Manual { min: _, max: _ } => {},
        };

        self.plot_infos.push(PlotInfo {
            label: desc.label.to_string(),
            data: Box::new(data),
            line,
            marker,
            xaxis: desc.xaxis,
            yaxis: desc.yaxis,
            pixel_perfect: desc.pixel_perfect,
        });
        self.plot_order.push(PlotType::Series);
    }

    /// Internal fill between setup function.
    fn fill_between_desc<D: FillData + 'a>(
        &mut self,
        desc: FillDescriptor,
        data: D,
    ) {
        let xaxis = match desc.xaxis {
            AxisType::X => &mut self.xaxis,
            AxisType::Y => &mut self.yaxis,
            AxisType::SecondaryX => &mut self.secondary_xaxis,
            AxisType::SecondaryY => &mut self.secondary_yaxis,
        };
        match xaxis.limit_policy {
            Limits::Auto => {
                // span
                xaxis.span = if let Some((xmin, xmax)) = xaxis.span {
                    Some((f64::min(xmin, data.xmin()), f64::max(xmax, data.xmax())))
                } else {
                    Some((data.xmin(), data.xmax()))
                };

                // limits
                let (xmin, xmax) = xaxis.span.unwrap();
                let extent = xmax - xmin;
                xaxis.limits = if extent > 0.0 {
                    Some((xmin - 0.05 * extent, xmax + 0.05 * extent))
                } else {
                    Some((xmin - 1.0, xmax + 1.0))
                };
            },
            Limits::Manual { min: _, max: _ } => {},
        };

        let yaxis = match desc.yaxis {
            AxisType::X => &mut self.xaxis,
            AxisType::Y => &mut self.yaxis,
            AxisType::SecondaryX => &mut self.secondary_xaxis,
            AxisType::SecondaryY => &mut self.secondary_yaxis,
        };
        match yaxis.limit_policy {
            Limits::Auto => {
                // span
                yaxis.span = if let Some((ymin, ymax)) = yaxis.span {
                    Some((f64::min(ymin, data.ymin()), f64::max(ymax, data.ymax())))
                } else {
                    Some((data.ymin(), data.ymax()))
                };

                // limits
                let (ymin, ymax) = yaxis.span.unwrap();
                let extent = ymax - ymin;
                yaxis.limits = if extent > 0.0 {
                    Some((ymin - 0.05 * extent, ymax + 0.05 * extent))
                } else {
                    Some((ymin - 1.0, ymax + 1.0))
                };
            },
            Limits::Manual { min: _, max: _ } => {},
        };

        self.fill_infos.push(FillInfo {
            label: desc.label.to_string(),
            data: Box::new(data),
            color_override: desc.color_override,
            xaxis: desc.xaxis,
            yaxis: desc.yaxis,
        });
        self.plot_order.push(PlotType::Fill);
    }
}

/// Builds and sets the configuration for a [`Subplot`].
pub struct SubplotBuilder<'a> {
    desc: SubplotDescriptor<'a>,
}
impl<'a> SubplotBuilder<'a> {
    /// Builds the subplot.
    pub fn build(self) -> Subplot<'a> {
        Subplot::new(&self.desc)
    }

    /// Sets the title of the subplot.
    pub fn title(mut self, title: &'a str) -> Self {
        self.desc.title = title;
        self
    }

    /// Sets the format of the subplot.
    pub fn format(mut self, format: SubplotFormat) -> Self {
        self.desc.format = format;
        self
    }

    /// Sets axis labels.
    pub fn label(mut self, axes: Axes, label: &'a str) -> Self {
        let axes = self.axes(axes);
        for axis in axes {
            axis.label = label;
        }

        self
    }
    /// Sets the x-axis label.
    /// Shortcut for calling `.label(Axes::X, label)`.
    pub fn xlabel(self, label: &'a str) -> Self {
        self.label(Axes::X, label)
    }
    /// Sets the y-axis label.
    /// Shortcut for calling `.label(Axes::Y, label)`.
    pub fn ylabel(self, label: &'a str) -> Self {
        self.label(Axes::Y, label)
    }

    /// Sets axis limits.
    pub fn limits(mut self, axes: Axes, limits: Limits) -> Self {
        let axes = self.axes(axes);
        for axis in axes {
            if let Limits::Manual { min, max } = limits {
                axis.limits = Some((min, max));
                axis.span = Some((min, max));
            }
            axis.limit_policy = limits;
        }

        self
    }
    /// Sets the x-axis limits.
    /// Shortcut for calling `.limits(Axes::X, limits)`.
    pub fn xlimits(self, limits: Limits) -> Self {
        self.limits(Axes::X, limits)
    }
    /// Sets the y-axis limits.
    /// Shortcut for calling `.limits(Axes::Y, limits)`.
    pub fn ylimits(self, limits: Limits) -> Self {
        self.limits(Axes::Y, limits)
    }

    /// Sets axis grid settings.
    pub fn grid(mut self, axes: Axes, grid: Grid) -> Self {
        let axes = self.axes(axes);
        for axis in axes {
            axis.grid = grid;
        }

        self
    }
    /// Turns on the major tick mark grid for the primary axes.
    /// Shortcut for calling `.grid(Axes::BothPrimary, Grid::Major)`.
    pub fn standard_grid(self) -> Self {
        self.grid(Axes::BothPrimary, Grid::Major)
    }

    /// Sets major tick mark locations.
    pub fn major_tick_marks(mut self, axes: Axes, spacing: TickSpacing) -> Self {
        let axes = self.axes(axes);
        for axis in axes {
            axis.major_tick_marks = spacing.clone();
        }

        self
    }

    /// Sets major tick mark labels.
    pub fn major_tick_labels(mut self, axes: Axes, labels: TickLabels) -> Self {
        let axes = self.axes(axes);
        for axis in axes {
            axis.major_tick_labels = labels.clone();
        }

        self
    }

    /// Sets minor tick mark locations.
    pub fn minor_tick_marks(mut self, axes: Axes, spacing: TickSpacing) -> Self {
        let axes = self.axes(axes);
        for axis in axes {
            axis.minor_tick_marks = spacing.clone();
        }

        self
    }

    /// Sets minor tick mark labels.
    pub fn minor_tick_labels(mut self, axes: Axes, labels: TickLabels) -> Self {
        let axes = self.axes(axes);
        for axis in axes {
            axis.minor_tick_labels = labels.clone();
        }

        self
    }

    /// Sets the visibility of axis lines.
    pub fn visible(mut self, axes: Axes, visible: bool) -> Self {
        let axes = self.axes(axes);
        for axis in axes {
            axis.visible = visible;
        }

        self
    }
}
impl<'a> SubplotBuilder<'a> {
    fn axes<'b>(&'b mut self, axes: Axes) -> Vec<&'b mut AxisDescriptor<&'a str>> {
        match axes {
            Axes::X => vec![&mut self.desc.xaxis],
            Axes::Y => vec![&mut self.desc.yaxis],
            Axes::SecondaryX => vec![&mut self.desc.secondary_xaxis],
            Axes::SecondaryY => vec![&mut self.desc.secondary_yaxis],
            Axes::BothX => vec![
                &mut self.desc.xaxis,
                &mut self.desc.secondary_xaxis,
            ],
            Axes::BothY => vec![
                &mut self.desc.yaxis,
                &mut self.desc.secondary_yaxis,
            ],
            Axes::BothPrimary => vec![
                &mut self.desc.xaxis,
                &mut self.desc.yaxis,
            ],
            Axes::BothSecondary => vec![
                &mut self.desc.secondary_xaxis,
                &mut self.desc.secondary_yaxis,
            ],
            Axes::All => vec![
                &mut self.desc.xaxis,
                &mut self.desc.yaxis,
                &mut self.desc.secondary_xaxis,
                &mut self.desc.secondary_yaxis,
            ],
        }
    }
}

/// Identifies one or more plot axes.
#[derive(Copy, Clone, Debug)]
pub enum Axes {
    X,
    Y,
    SecondaryX,
    SecondaryY,
    BothX,
    BothY,
    BothPrimary,
    BothSecondary,
    All,
}

/// The formatting for a subplot.
#[derive(Clone, Debug)]
pub struct SubplotFormat {
    /// The color used for plotted markers and lines, when there the color cycle is empty.
    pub default_marker_color: Color,
    /// The color used for filling regions, when there the color cycle is empty.
    pub default_fill_color: Color,
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
            default_fill_color: Color { r: 1.0, g: 0.0, b: 0.0, a: 0.5 },
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
            default_fill_color: Color { r: 1.0, g: 0.0, b: 0.0, a: 0.5 },
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
    /// Ticks are inside the axis lines.
    Inner,
    /// Ticks are outside the axis lines.
    Outer,
    /// Ticks are both inside and outside the axis lines.
    Both,
}

/// Describes how tick mark locations are determined, if at all.
#[derive(Clone, Debug)]
pub enum TickSpacing {
    /// Tick marks are present and located by the library.
    On,
    /// Tick marks are only present if a plot uses this axis.
    Auto,
    /// No tick marks on this axis.
    None,
    /// There are a set number of tick marks, evenly spaced.
    Count(u16),
    /// Tick marks are manually placed.
    Manual(Vec<f64>),
}

/// Describes how and whether tick mark labels are set.
#[derive(Clone, Debug)]
pub enum TickLabels {
    /// Tick labels are present and determined by the library.
    On,
    /// Tick labels are only present if a plot uses this axis.
    Auto,
    /// No tick labels on this axis.
    None,
    /// Tick labels are manually set.
    Manual(Vec<String>),
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

/// Plots data on a subplot using the builder pattern.
pub struct Plotter<'a, 'b> {
    subplot: &'b mut Subplot<'a>,
    desc: PlotDescriptor,
}
impl<'a, 'b> Plotter<'a, 'b> {
    /// Borrows data to be plotted and consumes the plotter.
    pub fn plot<Xs, Ys, Fx, Fy>(
        self,
        xs: Xs,
        ys: Ys,
    ) -> Result<(), PltError>
    where
        Fx: IntoF64,
        Fy: IntoF64,
        Xs: IntoIterator<Item=Fx>,
        Ys: IntoIterator<Item=Fy>,
        <Xs as IntoIterator>::IntoIter: iter::ExactSizeIterator + Clone + 'a,
        <Ys as IntoIterator>::IntoIter: iter::ExactSizeIterator + Clone + 'a,
    {
        let xdata = xs.into_iter().map(|f| f.f64());
        let ydata = ys.into_iter().map(|f| f.f64());

        if xdata.len() != ydata.len() {
            return Err(PltError::InvalidData(
                "Data is not correctly sized. x-data and y-data should be same length".to_owned()
            ));
        } else if xdata.clone().any(|x| x.is_nan()) {
            return Err(PltError::InvalidData("x-data has NaN value".to_owned()));
        } else if ydata.clone().any(|y| y.is_nan()) {
            return Err(PltError::InvalidData("y-data has NaN value".to_owned()));
        }

        let data = PlotData::new(xdata, ydata);

        self.subplot.plot_desc(self.desc, data);

        Ok(())
    }

    /// Borrows step data to be plotted and consumes the plotter.
    pub fn step<Xs, Ys, Fx, Fy>(
        mut self,
        steps: Xs,
        ys: Ys,
    ) -> Result<(), PltError>
    where
        Fx: IntoF64,
        Fy: IntoF64,
        Xs: IntoIterator<Item=Fx>,
        Ys: IntoIterator<Item=Fy>,
        <Xs as IntoIterator>::IntoIter: iter::ExactSizeIterator + Clone + 'a,
        <Ys as IntoIterator>::IntoIter: iter::ExactSizeIterator + Clone + 'a,
    {
        let step_data = steps.into_iter().map(|f| f.f64());
        let ydata = ys.into_iter().map(|f| f.f64());

        if step_data.len() != ydata.len() + 1 {
            return Err(PltError::InvalidData(
                "Data is not correctly sized. There should be one more step than y-value".to_owned()
            ));
        } else if step_data.clone().any(|step| step.is_nan()) {
            return Err(PltError::InvalidData("step-data has NaN value".to_owned()));
        } else if ydata.clone().any(|y| y.is_nan()) {
            return Err(PltError::InvalidData("y-data has NaN value".to_owned()));
        }

        self.desc.pixel_perfect = true;

        let data = StepData::new(step_data, ydata);

        self.subplot.plot_desc(self.desc, data);

        Ok(())
    }

    /// Uses the secondary X-Axis to reference x-data.
    pub fn use_secondary_xaxis(mut self) -> Self {
        self.desc.xaxis = AxisType::SecondaryX;

        self
    }

    /// Uses the secondary Y-Axis to reference y-data.
    pub fn use_secondary_yaxis(mut self) -> Self {
        self.desc.yaxis = AxisType::SecondaryY;

        self
    }

    /// Labels the data for use in a legend.
    pub fn label<S: AsRef<str>>(mut self, label: S) -> Self {
        self.desc.label = label.as_ref().to_string();

        self
    }

    /// Defines whether to draw lines between points and the line style.
    /// By default, lines are drawn and `Solid`.
    pub fn line(mut self, line_style: Option<LineStyle>) -> Self {
        if let Some(line_style) = line_style {
            self.desc.line = true;
            self.desc.line_format.style = line_style;
        } else {
            self.desc.line = false;
        }

        self
    }

    /// Sets the width of the lines.
    pub fn line_width(mut self, width: u32) -> Self {
        self.desc.line_format.width = width;

        self
    }

    /// Overrides the default line color.
    /// By default, line colors are determined by cycling through [`SubplotFormat::color_cycle`].
    pub fn line_color(mut self, color: Color) -> Self {
        self.desc.line_format.color_override = Some(color);

        self
    }

    /// Defines whether to draw markers at points and the marker style.
    /// By default, markers are not drawn.
    pub fn marker(mut self, marker_style: Option<MarkerStyle>) -> Self {
        if let Some(marker_style) = marker_style {
            self.desc.marker = true;
            self.desc.marker_format.style = marker_style;
        } else {
            self.desc.marker = false;
        }

        self
    }

    /// Sets the marker size.
    pub fn marker_size(mut self, size: u32) -> Self {
        self.desc.marker_format.size = size;

        self
    }

    /// Overrides the default marker color.
    /// By default, marker colors are determined by cycling through [`SubplotFormat::color_cycle`].
    pub fn marker_color(mut self, color: Color) -> Self {
        self.desc.marker_format.color_override = Some(color);

        self
    }

    /// Sets whether to draw marker outlines.
    /// By default, marker outlines are not drawn.
    pub fn marker_outline(mut self, on: bool) -> Self {
        self.desc.marker_format.outline = on;

        self
    }

    /// Overrides the default outline color for marker outlines.
    /// By default, marker outline colors are determined by cycling through [`SubplotFormat::color_cycle`].
    pub fn marker_outline_color(mut self, color: Color) -> Self {
        self.desc.marker_format.outline_format.color_override = Some(color);

        self
    }

    /// Sets the width of marker outlines.
    pub fn marker_outline_width(mut self, width: u32) -> Self {
        self.desc.marker_format.outline_format.width = width;

        self
    }

    /// Sets the line style of marker outlines.
    /// Defaults to `Solid`.
    pub fn marker_outline_style(mut self, line_style: LineStyle) -> Self {
        self.desc.marker_format.outline_format.style = line_style;

        self
    }
}

/// Fills a region of a subplot with a color.
pub struct Filler<'a, 'b> {
    subplot: &'b mut Subplot<'a>,
    desc: FillDescriptor,
}
impl<'a, 'b> Filler<'a, 'b> {
    /// Fills an area between two curves on the subplot.
    pub fn fill_between<Xs, Y1s, Y2s, Fx, Fy1, Fy2>(
        self,
        xs: Xs,
        y1s: Y1s,
        y2s: Y2s,
    ) -> Result<(), PltError>
    where
        Fx: IntoF64,
        Fy1: IntoF64,
        Fy2: IntoF64,
        Xs: IntoIterator<Item=Fx>,
        Y1s: IntoIterator<Item=Fy1>,
        Y2s: IntoIterator<Item=Fy2>,
        <Xs as IntoIterator>::IntoIter: iter::ExactSizeIterator + iter::DoubleEndedIterator + Clone + 'a,
        <Y1s as IntoIterator>::IntoIter: iter::ExactSizeIterator + iter::DoubleEndedIterator + Clone + 'a,
        <Y2s as IntoIterator>::IntoIter: iter::ExactSizeIterator + iter::DoubleEndedIterator + Clone + 'a,
    {
        let xdata = xs.into_iter().map(|f| f.f64());
        let y1data = y1s.into_iter().map(|f| f.f64());
        let y2data = y2s.into_iter().map(|f| f.f64());

        let data = FillBetweenData::new(xdata, y1data, y2data);

        self.subplot.fill_between_desc(self.desc, data);

        Ok(())
    }

    /// Uses the secondary Y-Axis to reference y-data.
    pub fn use_secondary_yaxis(mut self) -> Self {
        self.desc.yaxis = AxisType::SecondaryY;

        self
    }

    /// Labels the data for use in a legend.
    pub fn label<S: AsRef<str>>(mut self, label: S) -> Self {
        self.desc.label = label.as_ref().to_string();

        self
    }

    /// Overrides the default fill color.
    /// By default, line colors are determined by cycling through [`SubplotFormat::color_cycle`]
    /// with an alpha value of 0.5.
    pub fn color(mut self, color: Color) -> Self {
        self.desc.color_override = Some(color);

        self
    }
}

/// Plotting line styles.
#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
pub enum LineStyle {
    /// A solid line.
    Solid,
    /// A dashed line with regular sized dashes.
    Dashed,
    /// A dashed line with short dashes.
    ShortDashed,
}

/// Marker shapes.
#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
pub enum MarkerStyle {
    /// A circular marker.
    Circle,
    /// A square marker.
    Square,
}

// private

/// Describes the configuration of a [`Subplot`].
#[derive(Clone, Debug)]
pub(crate) struct SubplotDescriptor<'a> {
    /// The format of this subplot.
    pub format: SubplotFormat,
    /// The title displayed at the top of this subplot.
    pub title: &'a str,
    /// The default axis corresponding to x-values.
    pub xaxis: AxisDescriptor<&'a str>,
    /// The default axis corresponding to y-values.
    pub yaxis: AxisDescriptor<&'a str>,
    /// The secondary axis corresponding to x-values.
    pub secondary_xaxis: AxisDescriptor<&'a str>,
    /// The secondary axis corresponding to y-values.
    pub secondary_yaxis: AxisDescriptor<&'a str>,
}
impl Default for SubplotDescriptor<'_> {
    fn default() -> Self {
        Self {
            format: SubplotFormat::default(),
            title: "",
            xaxis: AxisDescriptor {
                label: "",
                major_tick_marks: TickSpacing::On,
                major_tick_labels: TickLabels::Auto,
                minor_tick_marks: TickSpacing::On,
                minor_tick_labels: TickLabels::None,
                grid: Grid::None,
                limit_policy: Limits::Auto,
                limits: None,
                span: None,
                visible: true,
            },
            yaxis: AxisDescriptor {
                label: "",
                major_tick_marks: TickSpacing::On,
                major_tick_labels: TickLabels::Auto,
                minor_tick_marks: TickSpacing::On,
                minor_tick_labels: TickLabels::None,
                grid: Grid::None,
                limit_policy: Limits::Auto,
                limits: None,
                span: None,
                visible: true,
            },
            secondary_xaxis: AxisDescriptor {
                label: "",
                major_tick_marks: TickSpacing::On,
                major_tick_labels: TickLabels::Auto,
                minor_tick_marks: TickSpacing::On,
                minor_tick_labels: TickLabels::None,
                grid: Grid::None,
                limit_policy: Limits::Auto,
                limits: None,
                span: None,
                visible: true,
            },
            secondary_yaxis: AxisDescriptor {
                label: "",
                major_tick_marks: TickSpacing::On,
                major_tick_labels: TickLabels::Auto,
                minor_tick_marks: TickSpacing::On,
                minor_tick_labels: TickLabels::None,
                grid: Grid::None,
                limit_policy: Limits::Auto,
                limits: None,
                span: None,
                visible: true,
            },
        }
    }
}

/// Represents different plottable dataset types.
#[derive(Copy, Clone, Debug)]
pub(crate) enum PlotType {
    Series,
    Fill,
}

/// Describes data and how it should be plotted.
#[derive(Clone, Debug)]
pub(crate) struct PlotDescriptor {
    /// The label corresponding to this data, displayed in a legend.
    pub label: String,
    /// Whether to draw lines between data points.
    pub line: bool,
    /// Whether to draw markers at data points.
    pub marker: bool,
    /// The format of lines, optionally drawn between data points.
    pub line_format: Line,
    /// The format of markers, optionally drawn at data points.
    pub marker_format: Marker,
    /// Which axis to use as the x-axis.
    pub xaxis: AxisType,
    /// Which axis to use as the y-axis.
    pub yaxis: AxisType,
    /// If plot points should be rounded to the nearest dot (pixel).
    pub pixel_perfect: bool,
}
impl Default for PlotDescriptor {
    fn default() -> Self {
        Self {
            label: String::new(),
            line: true,
            marker: false,
            line_format: Line::default(),
            marker_format: Marker::default(),
            xaxis: AxisType::X,
            yaxis: AxisType::Y,
            pixel_perfect: false,
        }
    }
}

/// Describes how to fill a specified area on a plot.
#[derive(Clone, Debug)]
pub(crate) struct FillDescriptor {
    /// The label corresponding to this data, displayed in a legend.
    pub label: String,
    /// The color to fill the area with.
    pub color_override: Option<Color>,
    /// Which axis to use as the x-axis.
    pub xaxis: AxisType,
    /// Which axis to use as the y-axis.
    pub yaxis: AxisType,
}
impl Default for FillDescriptor {
    fn default() -> Self {
        Self {
            label: String::new(),
            color_override: None,
            xaxis: AxisType::X,
            yaxis: AxisType::Y,
        }
    }
}

/// Format for lines plotted between data points.
#[derive(Copy, Clone, Debug)]
pub(crate) struct Line {
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

/// Format for markers drawn at data points.
#[derive(Clone, Debug)]
pub(crate) struct Marker {
    /// The shape of the marker.
    pub style: MarkerStyle,
    /// The size of the marker.
    pub size: u32,
    /// Optionally overrides the default fill color of the marker.
    pub color_override: Option<Color>,
    /// Whether to draw an outline.
    pub outline: bool,
    /// Format of an optional outline.
    pub outline_format: Line,
}
impl Default for Marker {
    fn default() -> Self {
        Self {
            style: MarkerStyle::Circle,
            size: 3,
            color_override: None,
            outline: false,
            outline_format: Line {
                width: 2,
                ..Default::default()
            },
        }
    }
}

/// Configuration for an axis.
#[derive(Clone, Debug)]
pub(crate) struct AxisDescriptor<S: AsRef<str>> {
    /// The label desplayed by the axis.
    pub label: S,
    /// Determines the major tick mark locations on this axis.
    pub major_tick_marks: TickSpacing,
    /// Determines the major tick labels on this axis.
    pub major_tick_labels: TickLabels,
    /// Determines the minor tick mark locations and labels on this axis.
    pub minor_tick_marks: TickSpacing,
    /// Determines the minor tick labels on this axis.
    pub minor_tick_labels: TickLabels,
    /// Sets which, if any, tick marks on this axis have grid lines.
    pub grid: Grid,
    /// How the maximum and minimum plotted values should be set.
    pub limit_policy: Limits,
    /// The range of values covered by the axis, if the axis is plotted on.
    pub limits: Option<(f64, f64)>,
    /// The maximum and minimum plotted values, if the axis is plotted on.
    pub span: Option<(f64, f64)>,
    /// Whether to draw the axis line.
    pub visible: bool,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub(crate) enum AxisType {
    X,
    Y,
    SecondaryX,
    SecondaryY,
}
impl AxisType {
    pub(crate) fn iter() -> array::IntoIter<Self, 4> {
        [Self::X, Self::Y, Self::SecondaryX, Self::SecondaryY].into_iter()
    }
}

pub(crate) type AxisBuf = AxisDescriptor<String>;
impl<S: AsRef<str>> AxisDescriptor<S> {
    fn to_buf(&self) -> AxisBuf {
        AxisBuf {
            label: self.label.as_ref().to_string(),
            major_tick_marks: self.major_tick_marks.clone(),
            major_tick_labels: self.major_tick_labels.clone(),
            minor_tick_marks: self.minor_tick_marks.clone(),
            minor_tick_labels: self.minor_tick_labels.clone(),
            grid: self.grid,
            limit_policy: self.limit_policy,
            limits: self.limits,
            span: self.span,
            visible: self.visible,
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct PlotInfo<'a> {
    // TODO implement legend
    #[allow(dead_code)]
    pub label: String,
    pub data: Box<dyn SeriesData + 'a>,
    pub line: Option<Line>,
    pub marker: Option<Marker>,
    pub xaxis: AxisType,
    pub yaxis: AxisType,
    pub pixel_perfect: bool,
}

#[derive(Clone, Debug)]
pub(crate) struct FillInfo<'a> {
    #[allow(dead_code)]
    pub label: String,
    pub data: Box<dyn FillData + 'a>,
    pub color_override: Option<Color>,
    pub xaxis: AxisType,
    pub yaxis: AxisType,
}

pub trait IntoF64 {
    fn f64(self) -> f64;
}
impl IntoF64 for f64 {
    #[inline]
    fn f64(self) -> f64 {
        self
    }
}
impl IntoF64 for &f64 {
    #[inline]
    fn f64(self) -> f64 {
        *self
    }
}
impl IntoF64 for f32 {
    #[inline]
    fn f64(self) -> f64 {
        self as f64
    }
}
impl IntoF64 for &f32 {
    #[inline]
    fn f64(self) -> f64 {
        *self as f64
    }
}
impl IntoF64 for u8 {
    #[inline]
    fn f64(self) -> f64 {
        self as f64
    }
}
impl IntoF64 for &u8 {
    #[inline]
    fn f64(self) -> f64 {
        *self as f64
    }
}
impl IntoF64 for u16 {
    #[inline]
    fn f64(self) -> f64 {
        self as f64
    }
}
impl IntoF64 for &u16 {
    #[inline]
    fn f64(self) -> f64 {
        *self as f64
    }
}
impl IntoF64 for u32 {
    #[inline]
    fn f64(self) -> f64 {
        self as f64
    }
}
impl IntoF64 for &u32 {
    #[inline]
    fn f64(self) -> f64 {
        *self as f64
    }
}
impl IntoF64 for i8 {
    #[inline]
    fn f64(self) -> f64 {
        self as f64
    }
}
impl IntoF64 for &i8 {
    #[inline]
    fn f64(self) -> f64 {
        *self as f64
    }
}
impl IntoF64 for i16 {
    #[inline]
    fn f64(self) -> f64 {
        self as f64
    }
}
impl IntoF64 for &i16 {
    #[inline]
    fn f64(self) -> f64 {
        *self as f64
    }
}
impl IntoF64 for i32 {
    #[inline]
    fn f64(self) -> f64 {
        self as f64
    }
}
impl IntoF64 for &i32 {
    #[inline]
    fn f64(self) -> f64 {
        *self as f64
    }
}

/// Holds data to be plotted.
#[derive(Copy, Clone)]
pub(crate) struct PlotData<Ix, Iy>
where
    Ix: Iterator<Item=f64> + Clone,
    Iy: Iterator<Item=f64> + Clone,
{
    xdata: Ix,
    ydata: Iy,
}
impl<Ix, Iy> fmt::Debug for PlotData<Ix, Iy> 
where
    Ix: Iterator<Item=f64> + Clone,
    Iy: Iterator<Item=f64> + Clone,
{
    fn fmt(&self, _: &mut Formatter) -> Result<(), fmt::Error> {
        Ok(())
    }
}
impl<Ix, Iy> SeriesData for PlotData<Ix, Iy> 
where
    Ix: Iterator<Item=f64> + Clone,
    Iy: Iterator<Item=f64> + Clone,
{
    fn data<'b>(&'b self) -> Box<dyn Iterator<Item = (f64, f64)> + 'b> {
        Box::new(iter::zip(
            self.xdata.clone(),
            self.ydata.clone(),
        ))
    }

    fn xmin(&self) -> f64 {
        self.xdata.clone().fold(f64::INFINITY, |a, b| a.min(b))
    }
    fn xmax(&self) -> f64 {
        self.xdata.clone().fold(f64::NEG_INFINITY, |a, b| a.max(b))
    }
    fn ymin(&self) -> f64 {
        self.ydata.clone().fold(f64::INFINITY, |a, b| a.min(b))
    }
    fn ymax(&self) -> f64 {
        self.ydata.clone().fold(f64::NEG_INFINITY, |a, b| a.max(b))
    }
}
impl<Ix, Iy> PlotData<Ix, Iy>
where
    Ix: Iterator<Item=f64> + Clone,
    Iy: Iterator<Item=f64> + Clone,
{
    /// Main constructor, taking separate array views of x-values and y-values.
    pub fn new(
        xs: Ix,
        ys: Iy,
    ) -> Self {
        let xdata = xs;
        let ydata = ys;

        Self { xdata, ydata }
    }
}

/// Holds borrowed step data to be plotted.
#[derive(Copy, Clone)]
pub(crate) struct StepData<Iedge, Idata>
where
    Iedge: Iterator<Item=f64> + Clone,
    Idata: Iterator<Item=f64> + Clone,
{
    edges: Iedge,
    ydata: Idata,
}
impl<Iedge, Idata> fmt::Debug for StepData<Iedge, Idata> 
where
    Iedge: Iterator<Item=f64> + Clone,
    Idata: Iterator<Item=f64> + Clone,
{
    fn fmt(&self, _: &mut Formatter) -> Result<(), fmt::Error> {
        Ok(())
    }
}
impl<Iedge, Idata> SeriesData for StepData<Iedge, Idata>
where
    Iedge: Iterator<Item=f64> + Clone,
    Idata: Iterator<Item=f64> + Clone,
{
    fn data<'b>(&'b self) -> Box<dyn Iterator<Item = (f64, f64)> + 'b> {
        Box::new(iter::zip(
            self.edges.clone().flat_map(|x| [x, x]).skip(1),
            self.ydata.clone().flat_map(|y| [y, y]),
        ))
    }

    fn xmin(&self) -> f64 {
        self.edges.clone().fold(f64::INFINITY, |a, b| a.min(b))
    }
    fn xmax(&self) -> f64 {
        self.edges.clone().fold(f64::NEG_INFINITY, |a, b| a.max(b))
    }
    fn ymin(&self) -> f64 {
        self.ydata.clone().fold(f64::INFINITY, |a, b| a.min(b))
    }
    fn ymax(&self) -> f64 {
        self.ydata.clone().fold(f64::NEG_INFINITY, |a, b| a.max(b))
    }
}
impl<Iedge, Idata> StepData<Iedge, Idata>
where
    Iedge: Iterator<Item=f64> + Clone,
    Idata: Iterator<Item=f64> + Clone,
{
    /// Main constructor, taking separate array views of steps and y-values.
    /// There should be one more step edge than y-values.
    pub fn new(
        edges: Iedge,
        ydata: Idata,
    ) -> Self {
        Self { edges, ydata }
    }
}

/// Holds borrowed data describing an area to be filled.
#[derive(Copy, Clone)]
pub(crate) struct FillBetweenData<Ix, Iy1, Iy2>
where
    Ix: Iterator<Item=f64> + iter::ExactSizeIterator + iter::DoubleEndedIterator + Clone,
    Iy1: Iterator<Item=f64> + iter::ExactSizeIterator + iter::DoubleEndedIterator + Clone,
    Iy2: Iterator<Item=f64> + iter::ExactSizeIterator + iter::DoubleEndedIterator + Clone,
{
    xdata: Ix,
    y1_data: Iy1,
    y2_data: Iy2,
}
impl<Ix, Iy1, Iy2> fmt::Debug for FillBetweenData<Ix, Iy1, Iy2> 
where
    Ix: Iterator<Item=f64> + iter::ExactSizeIterator + iter::DoubleEndedIterator + Clone,
    Iy1: Iterator<Item=f64> + iter::ExactSizeIterator + iter::DoubleEndedIterator + Clone,
    Iy2: Iterator<Item=f64> + iter::ExactSizeIterator + iter::DoubleEndedIterator + Clone,
{
    fn fmt(&self, _: &mut Formatter) -> Result<(), fmt::Error> {
        Ok(())
    }
}
impl<Ix, Iy1, Iy2> FillData for FillBetweenData<Ix, Iy1, Iy2>
where
    Ix: Iterator<Item=f64> + iter::ExactSizeIterator + iter::DoubleEndedIterator + Clone,
    Iy1: Iterator<Item=f64> + iter::ExactSizeIterator + iter::DoubleEndedIterator + Clone,
    Iy2: Iterator<Item=f64> + iter::ExactSizeIterator + iter::DoubleEndedIterator + Clone,
{
    fn curve1<'b>(&'b self) -> Box<dyn DoubleEndedIterator<Item = (f64, f64)> + 'b> {
        Box::new(iter::zip(
            self.xdata.clone(),
            self.y1_data.clone(),
        ))
    }

    fn curve2<'b>(&'b self) -> Box<dyn DoubleEndedIterator<Item = (f64, f64)> + 'b> {
        Box::new(iter::zip(
            self.xdata.clone(),
            self.y2_data.clone(),
        ))
    }

    fn xmin(&self) -> f64 {
        self.xdata.clone().fold(f64::INFINITY, |a, b| a.min(b))
    }
    fn xmax(&self) -> f64 {
        self.xdata.clone().fold(f64::NEG_INFINITY, |a, b| a.max(b))
    }
    fn ymin(&self) -> f64 {
        f64::min(
            self.y1_data.clone().fold(f64::INFINITY, |a, b| a.min(b)),
            self.y2_data.clone().fold(f64::INFINITY, |a, b| a.min(b)),
        )
    }
    fn ymax(&self) -> f64 {
        f64::max(
            self.y1_data.clone().fold(f64::NEG_INFINITY, |a, b| a.max(b)),
            self.y2_data.clone().fold(f64::NEG_INFINITY, |a, b| a.max(b)),
        )
    }
}
impl<Ix, Iy1, Iy2> FillBetweenData<Ix, Iy1, Iy2>
where
    Ix: Iterator<Item=f64> + iter::ExactSizeIterator + iter::DoubleEndedIterator + Clone,
    Iy1: Iterator<Item=f64> + iter::ExactSizeIterator + iter::DoubleEndedIterator + Clone,
    Iy2: Iterator<Item=f64> + iter::ExactSizeIterator + iter::DoubleEndedIterator + Clone,
{
    /// Main constructor, taking separate array views of x-values and y-values.
    pub fn new(
        xs: Ix,
        y1s: Iy1,
        y2s: Iy2,
    ) -> Self {
        Self {
            xdata: xs,
            y1_data: y1s,
            y2_data: y2s,
        }
    }
}

// traits

/// Implemented for data that can be represented by pairs of floats to be plotted.
pub(crate) trait SeriesData: dyn_clone::DynClone + fmt::Debug {
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

dyn_clone::clone_trait_object!(SeriesData);

pub(crate) trait FillData: dyn_clone::DynClone + fmt::Debug {
    /// Returns data for the first curve in an [`Iterator`] over x, y pairs.
    fn curve1<'a>(&'a self) -> Box<dyn DoubleEndedIterator<Item = (f64, f64)> + 'a>;
    /// Returns data for the second curve in an [`Iterator`] over x, y pairs.
    fn curve2<'a>(&'a self) -> Box<dyn DoubleEndedIterator<Item = (f64, f64)> + 'a>;
    /// The smallest x-value.
    fn xmin(&self) -> f64;
    /// The largest x-value.
    fn xmax(&self) -> f64;
    /// The smallest y-value.
    fn ymin(&self) -> f64;
    /// The largest y-value.
    fn ymax(&self) -> f64;
}

dyn_clone::clone_trait_object!(FillData);
