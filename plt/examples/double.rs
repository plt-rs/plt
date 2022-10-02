use plt::*;

fn main() {
    // create data
    let xs: Vec<f64> = (0..=100).map(|n: u32| n as f64 / 100.0).collect();
    let y1s: Vec<f64> = xs.iter().map(|x| x.powi(3)).collect();
    let y2s: Vec<f64> = y1s.iter().map(|y| *y * 20.0).rev().collect();

    // create subplot
    let mut sp = Subplot::builder()
        .title("double plot")
        .label(Axis::X, "x data")
        .label(Axis::Y, "y1 data")
        .label(Axis::SecondaryY, "y2 data")
        .build();

    // plot data on primary y-axis
    sp.plot(&xs, &y1s).unwrap();

    // plot data on secondary y-axis
    sp.plotter()
        .use_secondary_yaxis()
        .plot(&xs, &y2s)
        .unwrap();

    // make figure and add subplot
    let mut fig = <Figure>::default();
    fig.set_layout(SingleLayout::new(sp)).unwrap();

    // save figure to file
    fig.draw_file(FileFormat::Png, "example.png").unwrap();
}
