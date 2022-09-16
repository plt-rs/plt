# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Major Changes

#### plt

- Separated `plot` into `plot`, `plot_owned`, `step`, and `step_owned`.
- `plot` functions now take data directly.
- Removed `SeriesData` and all implementors from public API.

### Breaking Changes

#### plt

- Changed default configuration for `Subplot` to include major and minor ticks on all axes.
- Removed `Subplot::builder_detailed`.
- Removed `Line` and `Marker` from public API.

### Changed

#### plt

- Changed how and when limits are determined internally.
- `PlotDescriptor` no longer holds data, data is passed separately.
- Changed doc / readme example to use `Vec` instead of `ndarray::Array1`.

## [0.3.1] - 2022-09-09

### Added

#### plt

- `SubplotBuilder` methods for setting axis line visiblility.
- `SubplotBuilder` methods for setting grid lines.

### Changed

#### plt

- Updated README


## [0.3.0] - 2022-09-07

### Added

#### plt

- `SubplotBuilder` and `Plotter` for builder pattern subplot and plot construction.
- `builder` function to `Subplot` for getting a builder.
- Default constructor for `Figure`.
- Layouts for setting subplot areas
- `visible` member of SubplotDescriptor to control the visibility of an axis line.
- grid example.

### Changed

#### plt

- Moved from an init struct pattern, to builder pattern for constructing plots and subplots.
- Changed `FigureDescriptor` to `FigureFormat`.
- Changed name of pretty example to detailed.
- Split `Ticker` into separate `TickSpacing` and `TickLabels` structs.
- `Auto` varients of `TickSpacing` and `TickLabels` now determine if they should exist
  based on whether a plot uses that axis.
- Updated examples.

#### plt-draw

- Changed `GraphicsType` to `ImageType`.

### Removed

#### plt

- `SubplotDescriptor`, `Axis`, and `PlotDescriptor` from public API.
- Removed `Figure::add_subplot` in favor of using layouts.

## [0.2.1] - 2022-09-02

### Added

#### plt

- `StepData` and `StepDataOwned` for making step plots.
- step example.

## [0.2.0] - 2022-08-30

### Added

#### plt

- Functionality for drawing to provided backend through `Figure::draw_to_backend`.

#### plt-draw

- Alternate `CairoCanvas` constructor for providing the `cairo::Context`: `CairoCanvas::from_context`.
- `size` function to the `Canvas` trait for getting the size of a canvas.

### Changed

### plt

- Changed default figsize to more reasonable value.

### Fixed

#### plt

- Figsize was only using one dimension.
- Fixed doc error.

## [0.1.0] - 2022-08-24

- Crates:
  - `plt`: The main plotting API
  - `plt-draw`: The drawing backend
- Examples:
  - simple
  - pretty
  - double
