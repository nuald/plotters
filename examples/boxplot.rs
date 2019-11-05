use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = SVGBackend::new("plotters-doc-data/boxplot.svg", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .caption("Boxplot", ("sans-serif", 50.0).into_font())
        .build_ranged(0..2, 0f32..90f32)?;

    chart.configure_mesh().line_style_2(&WHITE).draw()?;

    chart.plotting_area().draw(
        &Boxplot::new_vertical(1, &[6, 7, 15, 36, 39, 40, 41, 42, 43, 47, 49], 15)
            .whisker_width(0.5),
    )?;

    Ok(())
}
