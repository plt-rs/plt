# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Major Changes

- Changed `SubplotBuilder` methods to take `Axes` type for specifying which axes are modified.
- Separated `plot` into `plot`, `plot_owned`, `step`, and `step_owned`.
- `plot` functions now take data directly.
- Removed `SeriesData` and all implementors from public API.

### Breaking Changes

- `PltError`, `LineStyle`, and `MarkerStyle` are now `non_exhaustive`.
- Minor tick mark locations are now spaced according to major ticks.
- Minor tick marks extend beyond major ticks when appropriate.
- Step plots are now pixel perfect.
- Moved some plt-draw re-exports to separate submodule.
- Changed default configuration for `Subplot` to include major and minor ticks on all axes.
- Removed `Subplot::builder_detailed`.
- Removed `Line` and `Marker` from public API.
- Changed default figure size.

### Changed

- Changed how and when limits are determined internally.
- `PlotDescriptor` no longer holds data, data is passed separately.
- Changed doc / readme example to use `Vec` instead of `ndarray::Array1`.

### Added

- `Axes` type for specifying which axes should be modified by `SubplotBuilder` methods.
- `fill_between` functionality.

## [0.3.1] - 2022-09-09

### Added

- `SubplotBuilder` methods for setting axis line visiblility.
- `SubplotBuilder` methods for setting grid lines.

### Changed

- Updated README


## [0.3.0] - 2022-09-07

### Added

- `SubplotBuilder` and `Plotter` for builder pattern subplot and plot construction.
- `builder` function to `Subplot` for getting a builder.
- Default constructor for `Figure`.
- Layouts for setting subplot areas
- `visible` member of SubplotDescriptor to control the visibility of an axis line.
- grid example.

### Changed

- Moved from an init struct pattern, to builder pattern for constructing plots and subplots.
- Changed `FigureDescriptor` to `FigureFormat`.
- Changed name of pretty example to detailed.
- Split `Ticker` into separate `TickSpacing` and `TickLabels` structs.
- `Auto` varients of `TickSpacing` and `TickLabels` now determine if they should exist
  based on whether a plot uses that axis.
- Updated examples.

### Removed

- `SubplotDescriptor`, `Axis`, and `PlotDescriptor` from public API.
- Removed `Figure::add_subplot` in favor of using layouts.

## [0.2.1] - 2022-09-02

### Added

- `StepData` and `StepDataOwned` for making step plots.
- step example.

## [0.2.0] - 2022-08-30

### Added

- Functionality for drawing to provided backend through `Figure::draw_to_backend`.

### Changed

- Changed default figsize to more reasonable value.

### Fixed

- Figsize was only using one dimension.
- Fixed doc error.
