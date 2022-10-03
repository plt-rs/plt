use plt::*;

use rand_distr::{Distribution, Normal};

fn main() {
    // create randomized scatter data
    let dist = Normal::new(0.0, 0.1).unwrap();
    let xs = (0..500)
        .map(|_| dist.sample(&mut rand::thread_rng()))
        .collect::<ndarray::Array1<_>>();
    let line_ys = xs.iter()
        .map(|x: &f64| 4.2 * x + 12.5)
        .collect::<ndarray::Array1<_>>();
    let scatter_ys = line_ys.iter()
        .map(|y: &f64| *y * (1.0 + dist.sample(&mut rand::thread_rng())))
        .collect::<ndarray::Array1<_>>();

    // create subplot
    let mut subplot = Subplot::builder()
        .format(SubplotFormat {
            font_size: 16.0,
            grid_color: Color::WHITE,
            line_color: Color::TRANSPARENT,
            plot_color: Color { r: 0.0, g: 0.1, b: 0.3, a: 0.2 },
            ..Default::default()
        })
        .label(Axis::X, "x data")
        .label(Axis::Y, "y data")
        .grid(Axis::BothPrimary, Grid::Major)
        .build();

    subplot.plotter()
        .line_color(Color { r: 0.4, g: 0.4, b: 0.4, a: 1.0 })
        .plot(&xs, &line_ys).unwrap();

    subplot.plotter()
        .line(None)
        .marker(Some(MarkerStyle::Circle))
        .marker_color(Color { r: 0.0, g: 0.2, b: 0.5, a: 0.5 })
        .marker_size(5)
        .plot(&xs, &scatter_ys).unwrap();

    // make figure and add subplot
    let mut fig = <Figure>::default();
    fig.set_layout(SingleLayout::new(subplot)).unwrap();

    // save figure to file
    fig.draw_file(FileFormat::Png, "example.png").unwrap();
}
