use itertools::Itertools;
use plotters::data::fitting_range;
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
    let dataset = [
        ("1.1.1.1", "wireless", Quartiles::new(&ping_wireless_1x)),
        ("8.8.8.8", "wireless", Quartiles::new(&ping_wireless_8x)),
        ("1.1.1.1", "wired", Quartiles::new(&ping_wired_1x)),
        ("8.8.8.8", "wired", Quartiles::new(&ping_wired_8x)),
    ];

    let category = Category::new(
        "Host",
        dataset
            .iter()
            .unique_by(|x| x.0)
            .sorted_by(|a, b| a.2.median().partial_cmp(&b.2.median()).unwrap())
            .map(|x| x.0)
            .collect(),
    );

    let mut colors = [GREEN, RED].iter();
    let mut offsets = (-15..).step_by(30);
    let mut series = HashMap::new();
    for x in dataset.iter() {
        let entry = series
            .entry(x.1)
            .or_insert_with(|| (Vec::new(), colors.next().unwrap(), offsets.next().unwrap()));
        entry.0.push((x.0, &x.2));
    }

    let values: Vec<f32> = dataset
        .iter()
        .map(|x| x.2.values().to_vec())
        .flatten()
        .collect();
    let values_range = fitting_range(values.iter());

    let mut chart = ChartBuilder::on(&upper)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .caption("Ping Boxplot", ("sans-serif", 20).into_font())
        .build_ranged(
            values_range.start - 1.0..values_range.end + 1.0,
            category.range(),
        )?;

    chart
        .configure_mesh()
        .x_desc("Ping, ms")
        .y_desc(category.name())
        .line_style_2(&WHITE)
        .draw()?;

    for (label, (values, style, offset)) in &series {
        let style_copy = *style;
        chart
            .draw_series(values.iter().map(|x| {
                Boxplot::new_horizontal(category.get(&x.0).unwrap(), &x.1)
                    .width(20)
                    .whisker_width(0.5)
                    .style(*style)
                    .offset(*offset)
            }))?
            .label(*label)
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], style_copy));
    }
    chart
        .configure_series_labels()
        .border_style(&BLACK)
        .draw()?;

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
