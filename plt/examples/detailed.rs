use plt::*;

use rand_distr::{Distribution, Normal};

fn main() {
    // create true curve data
    let xs = ndarray::Array1::linspace(0.0, 10.0, 40);
    let line_ys = xs.iter()
        .map(|x: &f64| x.powi(3))
        .collect::<ndarray::Array1<_>>();
    let upper_errors: Vec<f64> = line_ys.iter().map(|y| 1.1 * y).collect();
    let lower_errors: Vec<f64> = line_ys.iter().map(|y| 0.9 * y).collect();

    // create randomized scatter data
    let dist = Normal::new(0.0, 0.1).unwrap();
    let scatter_ys = line_ys.iter()
        .map(|y: &f64| *y + *y * dist.sample(&mut rand::thread_rng()))
        .collect::<ndarray::Array1<_>>();

    // create new subplot
    let mut subplot = Subplot::builder()
        .format(SubplotFormat {
            font_size: 16.0,
            ..Default::default()
        })
        .label(Axes::X, "x [arbitrary units]")
        .limits(Axes::X, Limits::Manual { min: 0.0, max: 10.0 })
        .label(Axes::Y, "y [arbitrary units]")
        .limits(Axes::Y, Limits::Manual { min: 0.0, max: 1e3 })
        .grid(Axes::BothPrimary, Grid::Major)
        .build();

    // plot true line
    subplot.fill_between(&xs, &upper_errors, &lower_errors).unwrap();
    subplot.plotter()
        .line(Some(LineStyle::Dashed))
        .label("true curve")
        .plot(&xs, &line_ys)
        .unwrap();

    // plot scatter points
    subplot.plotter()
        .line(None)
        .marker(Some(MarkerStyle::Circle))
        .marker_color(Color::TRANSPARENT)
        .marker_outline(true)
        .marker_outline_color(Color::BLACK)
        .label("data")
        .plot(&xs, &scatter_ys)
        .unwrap();

    // make figure and add subplot
    let mut fig = <Figure>::new(&FigureFormat {
        size: FigSize { width: 8.0, height: 6.0 },
        //face_color: plt::Color::TRANSPARENT, // uncomment for transparent background
        ..Default::default()
    });
    fig.set_layout(SingleLayout::new(subplot)).unwrap();

    // save figure to file
    fig.draw_file(FileFormat::Png, "example.png").unwrap();
    fig.draw_file(FileFormat::Svg, "example.svg").unwrap();
}
