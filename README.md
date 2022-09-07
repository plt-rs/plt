# plt

[![Crates.io](https://img.shields.io/crates/v/plt)](https://crates.io/crates/plt)
[![docs.rs](https://img.shields.io/docsrs/plt)](https://docs.rs/plt)

A plotting library with a focus on publication level aesthetics and ergonomic control.

## Structure
- Plots are drawn on a [`Subplot`].
- One or more subplots are organized in a [`Layout`].
- The layout is added to a [`Figure`], which is used to draw to a file or [`Backend`].

## Use

To get started, see the [Examples](https://github.com/plt-rs/plt/tree/main/plt/examples) directory in the main repository.

### Example
```rust
   // create data
   //let xs = ...;
   //let ys = ...;

   // create subplot
   let mut sp = plt::Subplot::builder()
       .xlabel("x data")
       .ylabel("y data")
       .build();

   // plot data
   sp.plot(plt::PlotData::new(&xs, &ys)).unwrap();

   // make figure and add subplot
   let mut fig = <plt::Figure>::default();
   fig.set_layout(plt::SingleLayout::new(sp)).unwrap();

   // save figure to file
   fig.draw_file(plt::FileFormat::Png, "example.png").unwrap();
```

## Dependencies

The package currently depends on [Cairo](https://www.cairographics.org).

### Debian / Ubuntu
`apt install libcairo2-dev`

### Arch
`pacman -Syu cairo`
