fn main() {
    // create true curve data
    let bins = vec![0.0, 10.0, 20.0, 30.0, 40.0, 50.0];
    let ys =   vec![1.0, 2.5,  1.9,  3.7,  0.2];

    // create new subplot
    let mut subplot = plt::Subplot::new(&plt::SubplotDescriptor {
        format: plt::SubplotFormat {
            font_size: 16.0,
            ..Default::default()
        },
        xaxis: plt::Axis {
            label: "x [arbitrary units]",
            limits: plt::Limits::Manual { min: 0.0, max: 50.0 },
            major_tick_marks: plt::TickSpacing::Count(6),
            minor_tick_marks: plt::TickSpacing::Count(31),
            ..plt::SubplotDescriptor::detailed().xaxis
        },
        yaxis: plt::Axis {
            label: "y [arbitrary units]",
            limits: plt::Limits::Manual { min: 0.0, max: 5.0 },
            ..plt::SubplotDescriptor::detailed().yaxis
        },
        secondary_xaxis: plt::Axis {
            major_tick_marks: plt::TickSpacing::Count(6),
            minor_tick_marks: plt::TickSpacing::Count(31),
            ..plt::SubplotDescriptor::detailed().xaxis
        },
        ..plt::SubplotDescriptor::detailed()
    });

    // plot step
    subplot.plot(plt::PlotDescriptor {
        data: plt::StepData::new(&bins, &ys).unwrap(),
        ..Default::default()
    });

    // make figure and add subplot
    let mut fig = <plt::Figure>::new(&plt::FigureDescriptor::default());
    fig.add_subplot((1, 1, 1), subplot).unwrap();

    // save figure to file
    fig.draw_file(plt::FileFormat::Png, "test.png").unwrap();
    fig.draw_file(plt::FileFormat::Svg, "test.svg").unwrap();
}
