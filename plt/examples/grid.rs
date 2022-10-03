use plt::*;

use rand_distr::{Distribution, Normal};

fn main() {
    // create data
    let norm = Normal::new(0.0, 0.15).unwrap();
    let xs: Vec<f64> = (0..3_000)
        .map(|_| 10.0 * norm.sample(&mut rand::thread_rng()))
        .collect();
    let ys: Vec<f64> = (0..3_000)
        .map(|_| 10.0 * norm.sample(&mut rand::thread_rng()))
        .collect();

    // histogram data
    let bin_edges = ndarray::Array1::linspace(-6.0, 6.0, 31);
    let xhist = histogram(&xs, bin_edges.as_slice().unwrap());
    let yhist = histogram(&ys, bin_edges.as_slice().unwrap());

    // create center subplot
    let mut center_sp = Subplot::builder()
        .format(SubplotFormat {
            font_size: 16.0,
            ..Default::default()
        })
        .minor_tick_marks(Axis::BothPrimary, TickSpacing::Count(1))
        .major_tick_marks(Axis::BothSecondary, TickSpacing::None)
        .minor_tick_marks(Axis::BothSecondary, TickSpacing::None)
        .limits(Axis::BothPrimary, Limits::Manual { min: -6.0, max: 6.0 })
        .build();

    // scatter plot
    center_sp.plotter()
        .line(None)
        .marker(Some(MarkerStyle::Circle))
        .marker_color(Color::TRANSPARENT)
        .marker_outline(true)
        .marker_outline_color(Color::BLACK)
        .plot(&xs, &ys)
        .unwrap();
    center_sp.plotter()
        .line(None)
        .marker(Some(MarkerStyle::Circle))
        .marker_color(Color::WHITE)
        .plot(&xs, &ys)
        .unwrap();
    center_sp.plotter()
        .line(None)
        .marker(Some(MarkerStyle::Circle))
        .marker_color(Color { r: 0.8, g: 0.0, b: 0.3, a: 0.1 })
        .plot(&xs, &ys)
        .unwrap();

    // create top subplot
    let mut top_sp = Subplot::builder()
        .format(SubplotFormat {
            font_size: 16.0,
            ..Default::default()
        })
        .limits(Axis::X, Limits::Manual { min: -6.0, max: 6.0 })
        .limits(Axis::Y, Limits::Manual { min: 0.0, max: 500.0 })
        .minor_tick_marks(Axis::BothPrimary, TickSpacing::Count(1))
        .major_tick_marks(Axis::BothSecondary, TickSpacing::None)
        .minor_tick_marks(Axis::BothSecondary, TickSpacing::None)
        .build();

    // step plot the x-value histogram
    top_sp.plotter()
        .line_color(Color::BLACK)
        .line_width(2)
        .step(&bin_edges, &xhist)
        .unwrap();

    // create right subplot
    let mut right_sp = Subplot::builder()
        .format(SubplotFormat {
            font_size: 16.0,
            ..Default::default()
        })
        .limits(Axis::X, Limits::Manual { min: -6.0, max: 6.0 })
        .limits(Axis::Y, Limits::Manual { min: 0.0, max: 500.0 })
        .minor_tick_marks(Axis::BothPrimary, TickSpacing::Count(1))
        .major_tick_marks(Axis::BothSecondary, TickSpacing::None)
        .minor_tick_marks(Axis::BothSecondary, TickSpacing::None)
        .build();

    // step plot the y-value histogram
    right_sp.plotter()
        .line_color(Color::BLACK)
        .line_width(2)
        .step(&bin_edges, &yhist)
        .unwrap();

    // setup the layout
    let layout = GridLayout::from_array(vec![
        [Some(top_sp), None],
        [Some(center_sp.clone()), Some(right_sp.clone())],
    ]);

    // make figure and add subplots in grid
    let mut fig = <Figure>::default();
    fig.set_layout(layout).unwrap();

    // save figure to file
    fig.draw_file(FileFormat::Png, "example.png").unwrap();
}

// returns a new histogram with bins defined by `bin_edges` and filled from `data`.
fn histogram(data: &[f64], bin_edges: &[f64]) -> ndarray::Array1<f64> {
    let mut hist = ndarray::Array1::from_elem(bin_edges.len() - 1, 10.0);

    for d in data {
        for (i, bin) in bin_edges.windows(2).enumerate() {
            let lower = bin[0];
            let upper = bin[1];
            if (lower..upper).contains(d) {
                hist[i] += 1.0;
            }
        }
    }

    hist
}
