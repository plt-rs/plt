fn main() {
    // create true curve data
    let xs = ndarray::Array1::linspace(0.0, 10.0, 40);
    let ys = xs.iter()
        .map(|x: &f64| x.powi(3))
        .collect::<ndarray::Array1<_>>();

    // create new subplot
    let mut subplot = plt::Subplot::builder()
        .format(plt::SubplotFormat {
            font_size: 16.0,
            ..Default::default()
        })
        .xlabel("x [arbitrary units]")
        .xlimits(plt::Limits::Manual { min: 0.0, max: 10.0 })
        .ylabel("y [arbitrary units]")
        .ylimits(plt::Limits::Manual { min: 0.0, max: 1e3 })
        .xgrid(plt::Grid::Major)
        .ygrid(plt::Grid::Major)
        .build();

    // plot true line
    subplot.plotter()
        .line(Some(plt::LineStyle::Dashed))
        .label("true curve")
        .plot(plt::PlotData::new(&xs, &ys))
        .unwrap();

    // make figure and add subplots in grid
    let mut fig = <plt::Figure>::default();
    fig.set_layout(plt::GridLayout::from_array(vec![
        [Some(subplot.clone()), None],
        [Some(subplot.clone()), Some(subplot.clone())],
    ]))
    .unwrap();

    // save figure to file
    fig.draw_file(plt::FileFormat::Png, "example.png").unwrap();
}
