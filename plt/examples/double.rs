#![allow(clippy::field_reassign_with_default)]

fn main() {
    // create data
    let xs: Vec<f64> = (0..=100)
        .map(|n: u32| n as f64 / 100.0)
        .collect();
    let y1s: Vec<f64> = xs.iter()
        .map(|x| x.powi(3))
        .collect();
    let y2s: Vec<f64> = y1s.iter()
        .map(|y| *y * 20.0)
        .rev()
        .collect();

    // subplot configuration
    let mut sp_desc = plt::SubplotDescriptor::default();
    sp_desc.title = "double plot";
    sp_desc.yaxis.label = "y1 data";
    sp_desc.secondary_yaxis.label = "y2 data";
    sp_desc.secondary_yaxis.major_ticks = plt::Ticker::linear(5);
    sp_desc.xaxis.label = "x data";

    // create subplot
    let mut sp = plt::Subplot::new(&sp_desc);

    // plot data on primary y-axis
    sp.plot(plt::PlotDescriptor::from_data(plt::PlotData::new(&xs, &y1s).unwrap()));

    // plot data on secondary y-axis
    sp.use_secondary_yaxis();
    sp.plot(plt::PlotDescriptor::from_data(plt::PlotData::new(&xs, &y2s).unwrap()));

    // make figure and add subplot
    let mut fig = <plt::Figure>::new(&plt::FigureDescriptor::default());
    fig.add_subplot((1, 1, 1), sp).unwrap();

    // save figure to file
    fig.draw_file(plt::FileFormat::Png, "test.png").unwrap();
    fig.draw_file(plt::FileFormat::Svg, "test.svg").unwrap();
}
