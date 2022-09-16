fn main() {
    // create data
    let xs: Vec<f64> = (0..=100).map(|n: u32| n as f64).collect();
    let top: Vec<f64> = xs.iter().map(|x| x.powi(3)).collect();
    let bottom: Vec<f64> = xs.iter().map(|_| 0.0).collect();

    // create subplot
    let mut sp = plt::Subplot::builder()
        .title("simple plot")
        .xlabel("x data")
        .ylabel("y data")
        .build();

    // plot data
    sp.fill_between(&xs, &top, &bottom).unwrap();

    // make figure and add subplot
    let mut fig = <plt::Figure>::default();
    fig.set_layout(plt::SingleLayout::new(sp)).unwrap();

    // save figure to file
    fig.draw_file(plt::FileFormat::Png, "example.png").unwrap();
}
