fn main() {
    // create data
    let xs: Vec<f64> = (0..=100)
        .map(|n: u32| n as f64)
        .collect();
    let ys: Vec<f64> = xs.iter()
        .map(|x| x.powi(3))
        .collect();

    // create subplot
    let mut sp = plt::Subplot::builder()
        .title("simple plot")
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
}
