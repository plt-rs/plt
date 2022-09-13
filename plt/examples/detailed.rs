use rand_distr::{Distribution, Normal};

fn main() {
    // create true curve data
    let xs = ndarray::Array1::linspace(0.0, 10.0, 40);
    let line_ys = xs.iter()
        .map(|x: &f64| x.powi(3))
        .collect::<ndarray::Array1<_>>();

    // create randomized scatter data
    let dist = Normal::new(0.0, 0.1).unwrap();
    let scatter_ys = line_ys.iter()
        .map(|y: &f64| *y + *y * dist.sample(&mut rand::thread_rng()))
        .collect::<ndarray::Array1<_>>();

    // create new subplot
    let mut subplot = plt::Subplot::builder_detailed()
        .format(plt::SubplotFormat {
            font_size: 16.0,
            ..Default::default()
        })
        .xlabel("x [arbitrary units]")
        .xlimits(plt::Limits::Manual { min: 0.0, max: 10.0 })
        .ylabel("y [arbitrary units]")
        .ylimits(plt::Limits::Manual { min: 0.0, max: 1e3 })
        .build();

    // plot scatter points
    subplot.plotter()
        .line(None)
        .marker(Some(plt::MarkerStyle::Circle))
        .marker_color(plt::Color::TRANSPARENT)
        .marker_outline(true)
        .marker_outline_color(plt::Color::BLACK)
        .label("data")
        .plot(plt::PlotData::new(&xs, &scatter_ys))
        .unwrap();

    // plot true line
    subplot.plotter()
        .line(Some(plt::LineStyle::Dashed))
        .label("true curve")
        .plot(plt::PlotData::new(&xs, &line_ys))
        .unwrap();

    // make figure and add subplot
    let mut fig = <plt::Figure>::new(&plt::FigureFormat {
        size: plt::FigSize { width: 8.0, height: 6.0 },
        //face_color: plt::Color::TRANSPARENT, // uncomment for transparent background
        ..Default::default()
    });
    fig.set_layout(plt::SingleLayout::new(subplot)).unwrap();

    // save figure to file
    fig.draw_file(plt::FileFormat::Png, "example.png").unwrap();
    fig.draw_file(plt::FileFormat::Svg, "example.svg").unwrap();
}
