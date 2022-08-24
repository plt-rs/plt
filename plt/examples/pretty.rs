use rand_distr::{Normal, Distribution};

fn main() {
    // create true curve data
    let xs = ndarray::Array1::linspace(0.0, 10.0, 40);
    let line_ys = xs.iter()
        .map(|x: &f64| x.powi(3))
        .collect::<ndarray::Array1<_>>();

    // create randomized scatter data
    let dist = Normal::new(0.0, 0.1).unwrap();
    let scatter_ys = line_ys.iter()
        .map(|y: &f64| *y + *y * dist.sample(&mut rand::thread_rng()))
        .collect::<ndarray::Array1<_>>();

    // create new subplot
    let mut subplot = plt::Subplot::new(&plt::SubplotDescriptor {
        format: plt::SubplotFormat {
            font_size: 16.0,
            ..Default::default()
        },
        legend: true,
        xaxis: plt::Axis {
            label: "x [arbitrary units]",
            limits: plt::Limits::Manual { min: 0.0, max: 10.0 },
            ..plt::SubplotDescriptor::detailed().xaxis
        },
        yaxis: plt::Axis {
            label: "y [arbitrary units]",
            limits: plt::Limits::Manual { min: 0.0, max: 1e3 },
            ..plt::SubplotDescriptor::detailed().yaxis
        },
        ..plt::SubplotDescriptor::detailed()
    });

    // plot scatter points
    subplot.plot(plt::PlotDescriptor {
        data: plt::PlotData::new(&xs, &scatter_ys).unwrap(),
        line: None,
        marker: Some(plt::Marker {
            style: plt::MarkerStyle::Circle,
            color_override: Some(plt::Color::TRANSPARENT),
            outline: Some(plt::Line {
                color_override: Some(plt::Color::BLACK),
                width: 2,
                ..Default::default()
            }),
            ..Default::default()
        }),
        label: "data",
    });
    // plot true line
    subplot.plot(plt::PlotDescriptor {
        data: plt::PlotData::new(&xs, &line_ys).unwrap(),
        line: Some(plt::Line {
            style: plt::LineStyle::Dashed,
            ..Default::default()
        }),
        label: "true curve",
        ..Default::default()
    });

    // make figure and add subplot
    let mut fig = <plt::Figure>::new(&plt::FigureDescriptor {
        //dpi: 300,
        //face_color: plt::Color::TRANSPARENT,
        ..Default::default()
    });
    fig.add_subplot((1, 1, 1), subplot).unwrap();

    // save figure to file
    fig.draw_file(plt::FileFormat::Png, "test.png").unwrap();
    fig.draw_file(plt::FileFormat::Svg, "test.svg").unwrap();
}
