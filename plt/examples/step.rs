use plt::*;

fn main() {
    // create true curve data
    let bins = vec![0.0, 10.0, 20.0, 30.0, 40.0, 50.0];
    let ys = vec![1.0, 2.5, 1.9, 3.7, 0.2];

    // create new subplot
    let mut subplot = Subplot::builder()
        .format(SubplotFormat {
            font_size: 16.0,
            ..Default::default()
        })
        .label(Axis::X, "x [arbitrary units]")
        .limits(Axis::X, Limits::Manual { min: 0.0, max: 50.0 })
        .major_tick_marks(Axis::BothX, TickSpacing::Count(6))
        .minor_tick_marks(Axis::BothX, TickSpacing::Count(31))
        .label(Axis::Y, "y [arbitrary units]")
        .limits(Axis::Y, Limits::Manual { min: 0.0, max: 5.0 })
        .grid(Axis::BothPrimary, Grid::Major)
        .build();

    // plot step
    subplot.step(&bins, &ys).unwrap();

    // make figure and add subplot
    let mut fig = <Figure>::default();
    fig.set_layout(SingleLayout::new(subplot)).unwrap();

    // save figure to file
    fig.draw_file(FileFormat::Png, "example.png").unwrap();
}
