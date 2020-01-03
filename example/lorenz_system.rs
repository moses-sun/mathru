use mathru::*;
use mathru::algebra::linear::{Vector, Matrix};
use mathru::analysis::ode::{Solver, RK4};
use mathru_plot::{Figure, PlottingArea, Plot, Graph, Color};

fn main()
{
    // Define ODE
    //
    // sigma = 10.0
    // rho = 28.0
    // beta = 8.0/3.0

    let f = |_x: &f64, y: &Vector<f64> | -> Vector<f64>
    {
        let sigma: f64 = 10.0;
        let rho: f64 = 28.0;
        let beta = 8.0 / 3.0;
        let mut d_y = y.clone();

        let y_1: f64 = *y.get(0);
        let y_2: f64 = *y.get(1);
        let y_3: f64 = *y.get(2);

        *d_y.get_mut(0) = sigma * (y_2 - y_1);
        *d_y.get_mut(1) = y_1 * (rho - y_3) - y_2;
        *d_y.get_mut(2) = y_1 * y_2 - beta * y_3;

        return d_y;
    };

    // Initial condition
    // t(0) = (1.0 1.0 1.0)^T
    let init: Vector<f64> = vector![1.0; 0.0; 0.0];

    // Create solver instance
    let solver: RK4<f64> = RK4::new(0.01);

    // solve ODE f with initial conditions init in range 0.0, 30.0
    let (x, y): (Vector<f64>, Matrix<f64>) = solver.solve(f, init, 0.0, 30.0);
    let (y_m, _y_n) = y.dim();

    let mut graph_yz: Vec<(f64, f64)> = Vec::new();
    let mut graph_xz: Vec<(f64, f64)> = Vec::new();

    // Convert data
    for i in 0..y_m
    {
        let y_1_i = *y.get(i, 0);
        let y_2_i = *y.get(i, 1);
        let y_3_i = *y.get(i, 2);

        graph_yz.push((y_2_i, y_3_i));
        graph_xz.push((y_1_i, y_3_i));
    }

    //Create chart
    let mut figure: Figure = Figure::new(1024, 768);
    let area: &mut PlottingArea = figure.get_plottingarea();
    let plot: &mut Plot = area.get_plot();

    plot.set_x_range(-28.0f64, 40.0f64).set_y_range(0.0f64, 50.0f64);

    plot.add_graph(Graph::new(graph_yz, Color::Black));

    plot.set_x_axis_designator("y");
    plot.set_y_axis_designator("z");
    plot.set_title("Lorenz system");

    figure.save("../figure/lorenz_system.png").unwrap();;
}