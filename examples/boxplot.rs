use plotters::prelude::*;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = SVGBackend::new("plotters-doc-data/boxplot.svg", (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let (upper, lower) = root.split_vertically(512);

    let ping_wireless_1x = [
        41.6, 32.5, 33.1, 32.3, 36.7, 32.0, 33.1, 32.0, 32.9, 32.7, 34.5, 36.5, 31.9, 33.7, 32.6,
        35.1,
    ];
    let ping_wireless_8x = [
        42.3, 32.9, 32.9, 34.3, 32.0, 33.3, 31.5, 33.1, 33.2, 35.9, 42.3, 34.1, 34.2, 34.2, 32.4,
        33.0,
    ];
    let ping_wired_1x = [
        31.8, 28.6, 29.4, 28.8, 28.2, 28.8, 28.4, 28.6, 28.3, 28.5, 28.5, 28.5, 28.4, 28.6, 28.4,
        28.9,
    ];
    let ping_wired_8x = [
        33.3, 28.4, 28.7, 29.1, 29.6, 28.9, 28.6, 29.3, 28.6, 29.1, 28.7, 28.3, 28.3, 28.6, 29.4,
        33.1,
    ];
    // TODO: use group ranges
    let _groups = ["1.1.1.1", "8.8.8.8"];
    let series = [
        (0, "wireless", Quartiles::new(&ping_wireless_1x)),
        (1, "wireless", Quartiles::new(&ping_wireless_8x)),
        (0, "wired", Quartiles::new(&ping_wired_1x)),
        (1, "wired", Quartiles::new(&ping_wired_8x)),
    ];
    let mut styles = HashMap::new();
    styles.insert("wireless", (-15, &GREEN));
    styles.insert("wired", (15, &RED));

    let mut chart = ChartBuilder::on(&upper)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .caption("Ping Boxplot", ("sans-serif", 20).into_font())
        .build_ranged(20f32..40f32, -1..2)?;

    chart.configure_mesh().line_style_2(&WHITE).draw()?;
    chart.draw_series(series.iter().map(|x| {
        Boxplot::new_horizontal(x.0, &x.2)
            .width(20)
            .whisker_width(0.5)
            .offset(styles[x.1].0)
            .style(styles[x.1].1)
    }))?;

    let drawing_areas = lower.split_evenly((1, 2));
    let (left, right) = (&drawing_areas[0], &drawing_areas[1]);

    let values = Quartiles::new(&[6, 7, 15, 36, 39, 40, 41, 42, 43, 47, 49]);
    let mut chart = ChartBuilder::on(&left)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .caption("Vertical Boxplot", ("sans-serif", 20).into_font())
        .build_ranged(0..2, 0f32..100f32)?;

    chart.configure_mesh().line_style_2(&WHITE).draw()?;
    chart
        .plotting_area()
        .draw(&Boxplot::new_vertical(1, &values))?;

    let mut chart = ChartBuilder::on(&right)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .caption("Horizontal Boxplot", ("sans-serif", 20).into_font())
        .build_ranged(0f32..100f32, 0..2)?;

    chart.configure_mesh().line_style_2(&WHITE).draw()?;
    chart
        .plotting_area()
        .draw(&Boxplot::new_horizontal(1, &values))?;

    Ok(())
}
