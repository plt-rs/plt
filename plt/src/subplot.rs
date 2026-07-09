use crate::{Color, FontName, PltError};

use std::{array, f64, iter};

/// The object that represents a whole subplot and is used to draw plotted data.
#[derive(Clone, Debug)]
pub struct Subplot {
    pub(crate) format: SubplotFormat,
    pub(crate) plot_order: Vec<PlotType>,
    pub(crate) plot_infos: Vec<PlotInfo>,
    pub(crate) fill_infos: Vec<FillInfo>,
    pub(crate) title: String,
    pub(crate) xaxis: AxisDescriptor,
    pub(crate) yaxis: AxisDescriptor,
    pub(crate) secondary_xaxis: AxisDescriptor,
    pub(crate) secondary_yaxis: AxisDescriptor,
}
impl Subplot {
    /// Returns a builder with default settings for constructing a subplot.
    pub fn builder() -> SubplotBuilder {
        SubplotBuilder { desc: SubplotDescriptor::default() }
    }

    /// Returns a [`Plotter`] for plotting X, Y data on this subplot.
    pub fn plotter(&mut self) -> Plotter<'_> {
        Plotter {
            subplot: self,
            desc: PlotDescriptor::default(),
        }
    }

    /// Returns a [`Filler`] for filling a region of the subplot with a color.
    pub fn filler(&mut self) -> Filler<'_> {
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
impl Subplot {
    /// Internal constructor.
    pub(crate) fn new(desc: &SubplotDescriptor) -> Self {
        Self {
            format: desc.format.clone(),
            plot_order: vec![],
            plot_infos: vec![],
            fill_infos: vec![],
            title: desc.title.clone(),
            xaxis: desc.xaxis.clone(),
            yaxis: desc.yaxis.clone(),
            secondary_xaxis: desc.secondary_xaxis.clone(),
            secondary_yaxis: desc.secondary_yaxis.clone(),
        }
    }
}
impl Subplot {
    /// Internal plot setup function.
    fn plot_desc(
        &mut self,
        desc: PlotDescriptor,
        data: SeriesData,
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
            data,
            line,
            marker,
            xaxis: desc.xaxis,
            yaxis: desc.yaxis,
            pixel_perfect: desc.pixel_perfect,
        });
        self.plot_order.push(PlotType::Series);
    }

    /// Internal fill between setup function.
    fn fill_between_desc(
        &mut self,
        desc: FillDescriptor,
        data: FillData,
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
            data,
            color_override: desc.color_override,
            xaxis: desc.xaxis,
            yaxis: desc.yaxis,
        });
        self.plot_order.push(PlotType::Fill);
    }
}

/// Builds and sets the configuration for a [`Subplot`].
pub struct SubplotBuilder {
    desc: SubplotDescriptor,
}
impl SubplotBuilder {
    /// Builds the subplot.
    pub fn build(self) -> Subplot {
        Subplot::new(&self.desc)
    }

    /// Sets the title of the subplot.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.desc.title = title.into();
        self
    }

    /// Sets the format of the subplot.
    pub fn format(mut self, format: SubplotFormat) -> Self {
        self.desc.format = format;
        self
    }

    /// Sets axis labels.
    pub fn label(mut self, axes: Axes, label: impl Into<String>) -> Self {
        let label = label.into();
        let axes = self.axes(axes);
        for axis in axes {
            axis.label = label.clone();
        }

        self
    }
    /// Sets the x-axis label.
    /// Shortcut for calling `.label(Axes::X, label)`.
    pub fn xlabel(self, label: impl Into<String>) -> Self {
        self.label(Axes::X, label)
    }
    /// Sets the y-axis label.
    /// Shortcut for calling `.label(Axes::Y, label)`.
    pub fn ylabel(self, label: impl Into<String>) -> Self {
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
impl SubplotBuilder {
    fn axes(&mut self, axes: Axes) -> Vec<&mut AxisDescriptor> {
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
pub struct Plotter<'b> {
    subplot: &'b mut Subplot,
    desc: PlotDescriptor,
}
impl<'b> Plotter<'b> {
    /// Takes data to be plotted and consumes the plotter.
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
    {
        let xdata: Vec<f64> = xs.into_iter().map(|f| f.f64()).collect();
        let ydata: Vec<f64> = ys.into_iter().map(|f| f.f64()).collect();

        if xdata.len() != ydata.len() {
            return Err(PltError::InvalidData(
                "Data is not correctly sized. x-data and y-data should be same length".to_owned()
            ));
        } else if xdata.iter().any(|x| x.is_nan()) {
            return Err(PltError::InvalidData("x-data has NaN value".to_owned()));
        } else if ydata.iter().any(|y| y.is_nan()) {
            return Err(PltError::InvalidData("y-data has NaN value".to_owned()));
        }

        let data = SeriesData::new_series(xdata, ydata);

        self.subplot.plot_desc(self.desc, data);

        Ok(())
    }

    /// Takes step data to be plotted and consumes the plotter.
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
    {
        let step_data: Vec<f64> = steps.into_iter().map(|f| f.f64()).collect();
        let ydata: Vec<f64> = ys.into_iter().map(|f| f.f64()).collect();

        if step_data.len() != ydata.len() + 1 {
            return Err(PltError::InvalidData(
                "Data is not correctly sized. There should be one more step than y-value".to_owned()
            ));
        } else if step_data.iter().any(|step| step.is_nan()) {
            return Err(PltError::InvalidData("step-data has NaN value".to_owned()));
        } else if ydata.iter().any(|y| y.is_nan()) {
            return Err(PltError::InvalidData("y-data has NaN value".to_owned()));
        }

        self.desc.pixel_perfect = true;

        let data = SeriesData::new_step(step_data, ydata);

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
pub struct Filler<'b> {
    subplot: &'b mut Subplot,
    desc: FillDescriptor,
}
impl<'b> Filler<'b> {
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
    {
        let xdata = xs.into_iter().map(|f| f.f64());
        let y1data = y1s.into_iter().map(|f| f.f64());
        let y2data = y2s.into_iter().map(|f| f.f64());

        let data = FillData::new(xdata, y1data, y2data);

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
pub(crate) struct SubplotDescriptor {
    /// The format of this subplot.
    pub format: SubplotFormat,
    /// The title displayed at the top of this subplot.
    pub title: String,
    /// The default axis corresponding to x-values.
    pub xaxis: AxisDescriptor,
    /// The default axis corresponding to y-values.
    pub yaxis: AxisDescriptor,
    /// The secondary axis corresponding to x-values.
    pub secondary_xaxis: AxisDescriptor,
    /// The secondary axis corresponding to y-values.
    pub secondary_yaxis: AxisDescriptor,
}
impl Default for SubplotDescriptor {
    fn default() -> Self {
        Self {
            format: SubplotFormat::default(),
            title: String::new(),
            xaxis: AxisDescriptor {
                label: String::new(),
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
                label: String::new(),
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
                label: String::new(),
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
                label: String::new(),
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
pub(crate) struct AxisDescriptor {
    /// The label desplayed by the axis.
    pub label: String,
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

#[derive(Clone, Debug)]
pub(crate) struct PlotInfo {
    // TODO implement legend
    #[allow(dead_code)]
    pub label: String,
    pub data: SeriesData,
    pub line: Option<Line>,
    pub marker: Option<Marker>,
    pub xaxis: AxisType,
    pub yaxis: AxisType,
    pub pixel_perfect: bool,
}

#[derive(Clone, Debug)]
pub(crate) struct FillInfo {
    #[allow(dead_code)]
    pub label: String,
    pub data: FillData,
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

/// Holds series data to be plotted.
#[derive(Clone, Debug)]
pub(crate) enum SeriesData {
    Series { xdata: Vec<f64>, ydata: Vec<f64> },
    Step { edges: Vec<f64>, ydata: Vec<f64> },
}
impl SeriesData {
    /// Main constructor for line/marker series data, taking x-values and y-values.
    pub fn new_series(
        xs: impl IntoIterator<Item = f64>,
        ys: impl IntoIterator<Item = f64>,
    ) -> Self {
        Self::Series {
            xdata: xs.into_iter().collect(),
            ydata: ys.into_iter().collect(),
        }
    }

    /// Main constructor for step data, taking step edges and y-values.
    /// There should be one more step edge than y-values.
    pub fn new_step(
        edges: impl IntoIterator<Item = f64>,
        ydata: impl IntoIterator<Item = f64>,
    ) -> Self {
        Self::Step {
            edges: edges.into_iter().collect(),
            ydata: ydata.into_iter().collect(),
        }
    }

    /// Returns data in an [`Iterator`] over x, y pairs.
    pub fn data(&self) -> Box<dyn Iterator<Item = (f64, f64)> + '_> {
        match self {
            Self::Series { xdata, ydata } => Box::new(iter::zip(
                xdata.iter().copied(),
                ydata.iter().copied(),
            )),
            Self::Step { edges, ydata } => Box::new(iter::zip(
                edges.iter().copied().flat_map(|x| [x, x]).skip(1),
                ydata.iter().copied().flat_map(|y| [y, y]),
            )),
        }
    }

    /// The smallest x-value.
    pub fn xmin(&self) -> f64 {
        let xdata = match self {
            Self::Series { xdata, .. } => xdata,
            Self::Step { edges, .. } => edges,
        };
        xdata.iter().copied().fold(f64::INFINITY, f64::min)
    }
    /// The largest x-value.
    pub fn xmax(&self) -> f64 {
        let xdata = match self {
            Self::Series { xdata, .. } => xdata,
            Self::Step { edges, .. } => edges,
        };
        xdata.iter().copied().fold(f64::NEG_INFINITY, f64::max)
    }
    /// The smallest y-value.
    pub fn ymin(&self) -> f64 {
        let ydata = match self {
            Self::Series { ydata, .. } => ydata,
            Self::Step { ydata, .. } => ydata,
        };
        ydata.iter().copied().fold(f64::INFINITY, f64::min)
    }
    /// The largest y-value.
    pub fn ymax(&self) -> f64 {
        let ydata = match self {
            Self::Series { ydata, .. } => ydata,
            Self::Step { ydata, .. } => ydata,
        };
        ydata.iter().copied().fold(f64::NEG_INFINITY, f64::max)
    }
}

/// Holds data describing an area to be filled.
#[derive(Clone, Debug)]
pub(crate) struct FillData {
    xdata: Vec<f64>,
    y1_data: Vec<f64>,
    y2_data: Vec<f64>,
}
impl FillData {
    /// Main constructor, taking separate x-values and the y-values of each curve.
    pub fn new(
        xs: impl IntoIterator<Item = f64>,
        y1s: impl IntoIterator<Item = f64>,
        y2s: impl IntoIterator<Item = f64>,
    ) -> Self {
        Self {
            xdata: xs.into_iter().collect(),
            y1_data: y1s.into_iter().collect(),
            y2_data: y2s.into_iter().collect(),
        }
    }

    /// Returns data for the first curve in an [`Iterator`] over x, y pairs.
    pub fn curve1(&self) -> impl DoubleEndedIterator<Item = (f64, f64)> + '_ {
        iter::zip(self.xdata.iter().copied(), self.y1_data.iter().copied())
    }
    /// Returns data for the second curve in an [`Iterator`] over x, y pairs.
    pub fn curve2(&self) -> impl DoubleEndedIterator<Item = (f64, f64)> + '_ {
        iter::zip(self.xdata.iter().copied(), self.y2_data.iter().copied())
    }

    /// The smallest x-value.
    pub fn xmin(&self) -> f64 {
        self.xdata.iter().copied().fold(f64::INFINITY, f64::min)
    }
    /// The largest x-value.
    pub fn xmax(&self) -> f64 {
        self.xdata.iter().copied().fold(f64::NEG_INFINITY, f64::max)
    }
    /// The smallest y-value.
    pub fn ymin(&self) -> f64 {
        f64::min(
            self.y1_data.iter().copied().fold(f64::INFINITY, f64::min),
            self.y2_data.iter().copied().fold(f64::INFINITY, f64::min),
        )
    }
    /// The largest y-value.
    pub fn ymax(&self) -> f64 {
        f64::max(
            self.y1_data.iter().copied().fold(f64::NEG_INFINITY, f64::max),
            self.y2_data.iter().copied().fold(f64::NEG_INFINITY, f64::max),
        )
    }
}
