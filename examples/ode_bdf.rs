use mathru::{
    algebra::linear::{Matrix, Vector},
    analysis::differential_equation::ordinary::{
        solver::implicit::BDF, ImplicitInitialValueProblemBuilder,
    },
    elementary::Trigonometry,
    {matrix, vector},
};
use plotters::prelude::*;

fn main() {
    fn ode(x: &f64, y: &Vector<f64>) -> Vector<f64> {
        let i1 = 0.5;
        let i2 = 2.0;
        let i3 = 3.0;

        let a = (i2 - i3) / i1;
        let b = (i3 - i1) / i2;
        let c = (i1 - i2) / i3;

        let y_1s = a * (y[1] * y[2]);
        let y_2s = b * (y[2] * y[0]);

        let f = if *x >= 3.0 * f64::pi() && *x <= 4.0 * f64::pi() {
            0.25 * x.sin() * x.sin()
        } else {
            0.0
        };

        let y_3s = c * (y[0] * y[1]) + f;
        vector![y_1s; y_2s; y_3s]
    }

    fn jacobian(_x: &f64, y: &Vector<f64>) -> Matrix<f64> {
        let i1 = 0.5;
        let i2 = 2.0;
        let i3 = 3.0;

        let a = (i2 - i3) / i1;
        let b = (i3 - i1) / i2;
        let c = (i1 - i2) / i3;

        matrix![0.0, a * y[2], a * y[1];
                        b * y[2], 0.0, b * y[0];
                        c * y[1], c * y[0], 0.0]
    }

    let x_start = 0.0f64;
    let x_end = 20.0f64;

    let problem =
        ImplicitInitialValueProblemBuilder::new(&ode, &jacobian, 0.0f64, vector![1.0; 0.0; 0.9])
            .t_end(x_end)
            .build();

    let step_size: f64 = 0.0001;
    let solver: BDF<f64> = BDF::new(6, step_size);

    let (x, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem).unwrap();

    //Create chart
    let mut graph_x1: Vec<(f64, f64)> = Vec::with_capacity(x.len());
    let mut graph_x2: Vec<(f64, f64)> = Vec::with_capacity(x.len());
    let mut graph_x3: Vec<(f64, f64)> = Vec::with_capacity(x.len());

    for i in 0..y.len() {
        let x_i = x[i];

        graph_x1.push((x_i, y[i][0]));
        graph_x2.push((x_i, y[i][1]));
        graph_x3.push((x_i, y[i][2]));
    }

    let root_area = BitMapBackend::new("./figures/ode_bdf.png", (1200, 800)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .margin(20)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("ODE solved with BDF", ("Arial", 40))
        .build_cartesian_2d(x_start..x_end, -1.0f64..1.5f64)
        .unwrap();

    ctx.configure_mesh()
        .x_desc("Time t")
        .axis_desc_style(("sans-serif", 25).into_font())
        .draw()
        .unwrap();

    ctx.draw_series(LineSeries::new(graph_x1, &BLACK)).unwrap();

    ctx.draw_series(LineSeries::new(graph_x2, &RED)).unwrap();

    ctx.draw_series(LineSeries::new(graph_x3, &BLUE)).unwrap();
}
