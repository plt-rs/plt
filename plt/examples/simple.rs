use plt::*;

fn main() {
    let xs: Vec<f64> = (0..=100).map(|n: u32| n as f64 * 0.1).collect();
    let ys: Vec<f64> = xs.iter().map(|x| x.powi(3)).collect();

    // create subplot
    let mut subplot = Subplot::builder()
        .xlabel("X")
        .ylabel("Y")
        .build();

    // plot data
    subplot.plot(&xs, &ys).unwrap();

    // make figure and add subplot
    let mut fig = <Figure>::default();
    fig.set_layout(SingleLayout::new(subplot)).unwrap();

    // save figure to file
    fig.draw_file(FileFormat::Png, "example.png").unwrap();
}
