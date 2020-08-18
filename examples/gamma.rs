use mathru::special::gamma;
use plotters::prelude::*;

fn main()
{
    let x_start: f64 = -3.50;
    let x_end: f64 = 5.0;
    let length: usize = 2000;

    let mut graph_1: Vec<(f64, f64)> = Vec::with_capacity(length);
    for i in 0..length
    {
        let x: f64 = (x_end - x_start) / (length as f64) * (i as f64) + x_start;
        graph_1.push((x, gamma::gamma(x)));
    }

    let root_area =
        BitMapBackend::new("./figure/gamma.png", (1200, 800)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .margin(20)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption(
            "gamma(x)",
            ("Arial", 40),
        )
        .build_ranged(x_start..x_end, -10.0f64..25.0f64)
        .unwrap();

    ctx.configure_mesh()
        .x_desc("x")
        .axis_desc_style(("sans-serif", 25).into_font())
        .draw()
        .unwrap();

    //ctx.draw_series(
      //  graph_1.iter()
       // .map(|(x, y)| Circle::new((*x, *y), 1, BLUE.filled()))).unwrap();
    ctx.draw_series(LineSeries::new(graph_1, &BLUE)).unwrap();
}
