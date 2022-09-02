# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
