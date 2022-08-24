#![allow(clippy::field_reassign_with_default)]

fn main() {
    // create data
    let xs: Vec<f64> = (0..=100)
        .map(|n: u32| n as f64)
        .collect();
    let ys: Vec<f64> = xs.iter()
        .map(|x| x.powi(3))
        .collect();

    // subplot configuration
    let mut sp_desc = plt::SubplotDescriptor::default();
    sp_desc.title = "simple plot";
    sp_desc.yaxis.label = "y data";
    sp_desc.xaxis.label = "x data";

    // create subplot
    let mut sp = plt::Subplot::new(&sp_desc);

    // plot data
    sp.plot(plt::PlotDescriptor::from_data(plt::PlotData::new(&xs, &ys).unwrap()));

    // make figure and add subplot
    let mut fig = <plt::Figure>::new(&plt::FigureDescriptor::default());
    fig.add_subplot((1, 1, 1), sp).unwrap();

    // save figure to file
    fig.draw_file(plt::FileFormat::Png, "test.png").unwrap();
    fig.draw_file(plt::FileFormat::Svg, "test.svg").unwrap();
}
