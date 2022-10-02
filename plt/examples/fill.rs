use plt::*;

fn main() {
    // create data
    let xs: Vec<f64> = (0..=100).map(|n: u32| n as f64).collect();
    let ys: Vec<f64> = xs.iter().map(|x| x.powi(3)).collect();

    let upper_errors: Vec<f64> = ys.iter().map(|y| 1.1 * y).collect();
    let lower_errors: Vec<f64> = ys.iter().map(|y| 0.9 * y).collect();

    // create subplot
    let mut sp = Subplot::builder()
        .label(Axis::X, "x data")
        .label(Axis::Y, "y data")
        .build();

    // plot data
    sp.fill_between(&xs, &upper_errors, &lower_errors).unwrap();
    sp.plot(&xs, &ys).unwrap();

    // make figure and add subplot
    let mut fig = <Figure>::default();
    fig.set_layout(SingleLayout::new(sp)).unwrap();

    // save figure to file
    fig.draw_file(FileFormat::Png, "example.png").unwrap();
}
