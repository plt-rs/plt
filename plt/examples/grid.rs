use plt::*;

fn main() {
    // create true curve data
    let xs = ndarray::Array1::linspace(0.0, 10.0, 40);
    let ys = xs.iter()
        .map(|x: &f64| x.powi(3))
        .collect::<ndarray::Array1<_>>();

    // create new subplot
    let mut subplot = Subplot::builder()
        .format(SubplotFormat {
            font_size: 16.0,
            ..Default::default()
        })
        .label(Axis::X, "x [arbitrary units]")
        .limits(Axis::X, Limits::Manual { min: 0.0, max: 10.0 })
        .label(Axis::Y, "y [arbitrary units]")
        .limits(Axis::Y, Limits::Manual { min: 0.0, max: 1e3 })
        .grid(Axis::BothPrimary, Grid::Major)
        .build();

    // plot true line
    subplot.plotter()
        .line(Some(LineStyle::Dashed))
        .label("true curve")
        .plot(&xs, &ys)
        .unwrap();

    // make figure and add subplots in grid
    let mut fig = <Figure>::default();
    fig.set_layout(GridLayout::from_array(vec![
        [Some(subplot.clone()), None],
        [Some(subplot.clone()), Some(subplot.clone())],
    ]))
    .unwrap();

    // save figure to file
    fig.draw_file(FileFormat::Png, "example.png").unwrap();
}
