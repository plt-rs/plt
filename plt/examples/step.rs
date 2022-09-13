fn main() {
    // create true curve data
    let bins = vec![0.0, 10.0, 20.0, 30.0, 40.0, 50.0];
    let ys = vec![1.0, 2.5, 1.9, 3.7, 0.2];

    // create new subplot
    let mut subplot = plt::Subplot::builder()
        .format(plt::SubplotFormat {
            font_size: 16.0,
            ..Default::default()
        })
        .xlabel("x [arbitrary units]")
        .xlimits(plt::Limits::Manual { min: 0.0, max: 50.0 })
        .xmajor_tick_marks(plt::TickSpacing::Count(6))
        .xminor_tick_marks(plt::TickSpacing::Count(31))
        .ylabel("y [arbitrary units]")
        .ylimits(plt::Limits::Manual { min: 0.0, max: 5.0 })
        .secondary_xmajor_tick_marks(plt::TickSpacing::Count(6))
        .secondary_xminor_tick_marks(plt::TickSpacing::Count(31))
        .xgrid(plt::Grid::Major)
        .ygrid(plt::Grid::Major)
        .build();

    // plot step
    subplot.plot(plt::StepData::new(&bins, &ys)).unwrap();

    // make figure and add subplot
    let mut fig = <plt::Figure>::default();
    fig.set_layout(plt::SingleLayout::new(subplot)).unwrap();

    // save figure to file
    fig.draw_file(plt::FileFormat::Png, "example.png").unwrap();
}
