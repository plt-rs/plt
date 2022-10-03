# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Breaking Changes

- All `Canvas` methods return results.

### Added

- Crate error type `DrawError`.
- Rename `graphics_type` to `image_format` to match type name.
- `fill_region` function for `Canvas` to facilitate a `fill_between` function in `plt`.

## [0.3.0] - 2022-09-07

### Changed

- Changed `GraphicsType` to `ImageType`.

## [0.2.0] - 2022-08-30

- Alternate `CairoCanvas` constructor for providing the `cairo::Context`: `CairoCanvas::from_context`.
- `size` function to the `Canvas` trait for getting the size of a canvas.
