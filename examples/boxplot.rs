use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = SVGBackend::new("plotters-doc-data/boxplot.svg", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let (left, right) = root.split_horizontally(240);

    let mut chart = ChartBuilder::on(&left)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .caption("Vertical Boxplot", ("sans-serif", 20).into_font())
        .build_ranged(0..2, 0f32..90f32)?;

    chart.configure_mesh().line_style_2(&WHITE).draw()?;

    chart.plotting_area().draw(
        &Boxplot::new_vertical(1, &[6, 7, 15, 36, 39, 40, 41, 42, 43, 47, 49])
            .width(15)
            .whisker_width(0.5),
    )?;

    let mut chart = ChartBuilder::on(&right)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .caption("Horizontal Boxplot", ("sans-serif", 20).into_font())
        .build_ranged(0f32..90f32, 0..2)?;

    chart.configure_mesh().line_style_2(&WHITE).draw()?;

    chart.plotting_area().draw(
        &Boxplot::new_horizontal(1, &[6, 7, 15, 36, 39, 40, 41, 42, 43, 47, 49])
            .width(15)
            .whisker_width(0.5),
    )?;

    chart.plotting_area().draw(
        &Boxplot::new_horizontal(1, &[7, 15, 36, 39, 40, 41])
            .width(15)
            .whisker_width(0.5)
            .offset(20),
    )?;

    Ok(())
}
