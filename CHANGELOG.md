# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

#### plt

- Layouts for setting subplot areas
- `visible` member of SubplotDescriptor to control the visibility of an axis line.

### Changed

#### plt

- Split `Ticker` into separate `TickSpacing` and `TickLabels` structs.
- `Auto` varients of `TickSpacing` and `TickLabels` now determine if they should exist
  based on whether a plot uses that axis.
- Deprecated `Figure::add_subplot` in favor of using layouts.

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
