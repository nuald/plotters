use itertools::Itertools;
use plotters::data::fitting_range;
use plotters::prelude::*;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, prelude::*, BufReader};

fn read_data<BR: BufRead>(reader: BR) -> HashMap<(String, String), Vec<f64>> {
    let mut ds = HashMap::new();
    for l in reader.lines() {
        let line = l.unwrap();
        let tuple: Vec<&str> = line.split('\t').collect();
        if tuple.len() == 3 {
            let key = (String::from(tuple[0]), String::from(tuple[1]));
            let entry = ds.entry(key).or_insert_with(Vec::new);
            entry.push(tuple[2].parse::<f64>().unwrap());
        }
    }
    ds
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = SVGBackend::new("plotters-doc-data/boxplot.svg", (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let (upper, lower) = root.split_vertically(512);

    let args: Vec<String> = env::args().collect();

    let ds = if args.len() < 2 {
        read_data(io::Cursor::new(get_data()))
    } else {
        let file = fs::File::open(&args[1])?;
        read_data(BufReader::new(file))
    };
    let dataset: Vec<(String, String, Quartiles)> = ds
        .iter()
        .map(|(k, v)| (k.0.clone(), k.1.clone(), Quartiles::new(&v)))
        .collect();

    let category = Category::new(
        "Host",
        dataset
            .iter()
            .unique_by(|x| x.0.clone())
            .sorted_by(|a, b| b.2.median().partial_cmp(&a.2.median()).unwrap())
            .map(|x| x.0.clone())
            .collect(),
    );

    let mut colors = [BLUE, RED].iter();
    let mut offsets = (-7..).step_by(14);
    let mut series = HashMap::new();
    for x in dataset.iter() {
        let entry = series
            .entry(x.1.clone())
            .or_insert_with(|| (Vec::new(), colors.next().unwrap(), offsets.next().unwrap()));
        entry.0.push((x.0.clone(), &x.2));
    }

    let values: Vec<f32> = dataset
        .iter()
        .map(|x| x.2.values().to_vec())
        .flatten()
        .collect();
    let values_range = fitting_range(values.iter());

    let mut chart = ChartBuilder::on(&upper)
        .x_label_area_size(40)
        .y_label_area_size(120)
        .caption("Ping Boxplot", ("sans-serif", 20).into_font())
        .build_ranged(0.0..values_range.end + 1.0, category.range())?;

    chart
        .configure_mesh()
        .x_desc("Ping, ms")
        .y_desc(category.name())
        .y_labels(category.len())
        .line_style_2(&WHITE)
        .draw()?;

    for (label, (values, style, offset)) in &series {
        let style_copy = *style;
        chart
            .draw_series(values.iter().map(|x| {
                Boxplot::new_horizontal(category.get(&x.0).unwrap(), &x.1)
                    .width(10)
                    .whisker_width(0.5)
                    .style(*style)
                    .offset(*offset)
            }))?
            .label(label)
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], style_copy));
    }
    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .background_style(WHITE.filled())
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

fn get_data() -> String {
    String::from(
        "
 1.1.1.1	wireless	41.6
 1.1.1.1	wireless	32.5
 1.1.1.1	wireless	33.1
 1.1.1.1	wireless	32.3
 1.1.1.1	wireless	36.7
 1.1.1.1	wireless	32.0
 1.1.1.1	wireless	33.1
 1.1.1.1	wireless	32.0
 1.1.1.1	wireless	32.9
 1.1.1.1	wireless	32.7
 1.1.1.1	wireless	34.5
 1.1.1.1	wireless	36.5
 1.1.1.1	wireless	31.9
 1.1.1.1	wireless	33.7
 1.1.1.1	wireless	32.6
 1.1.1.1	wireless	35.1
 8.8.8.8	wireless	42.3
 8.8.8.8	wireless	32.9
 8.8.8.8	wireless	32.9
 8.8.8.8	wireless	34.3
 8.8.8.8	wireless	32.0
 8.8.8.8	wireless	33.3
 8.8.8.8	wireless	31.5
 8.8.8.8	wireless	33.1
 8.8.8.8	wireless	33.2
 8.8.8.8	wireless	35.9
 8.8.8.8	wireless	42.3
 8.8.8.8	wireless	34.1
 8.8.8.8	wireless	34.2
 8.8.8.8	wireless	34.2
 8.8.8.8	wireless	32.4
 8.8.8.8	wireless	33.0
 1.1.1.1	wired	31.8
 1.1.1.1	wired	28.6
 1.1.1.1	wired	29.4
 1.1.1.1	wired	28.8
 1.1.1.1	wired	28.2
 1.1.1.1	wired	28.8
 1.1.1.1	wired	28.4
 1.1.1.1	wired	28.6
 1.1.1.1	wired	28.3
 1.1.1.1	wired	28.5
 1.1.1.1	wired	28.5
 1.1.1.1	wired	28.5
 1.1.1.1	wired	28.4
 1.1.1.1	wired	28.6
 1.1.1.1	wired	28.4
 1.1.1.1	wired	28.9
 8.8.8.8	wired	33.3
 8.8.8.8	wired	28.4
 8.8.8.8	wired	28.7
 8.8.8.8	wired	29.1
 8.8.8.8	wired	29.6
 8.8.8.8	wired	28.9
 8.8.8.8	wired	28.6
 8.8.8.8	wired	29.3
 8.8.8.8	wired	28.6
 8.8.8.8	wired	29.1
 8.8.8.8	wired	28.7
 8.8.8.8	wired	28.3
 8.8.8.8	wired	28.3
 8.8.8.8	wired	28.6
 8.8.8.8	wired	29.4
 8.8.8.8	wired	33.1
",
    )
}
