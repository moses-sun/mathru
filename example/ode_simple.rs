use mathru::*;
use mathru::algebra::linear::{Vector, Matrix};
use mathru::analysis::ode::{Solver, RK4};
use mathru_plot::{Figure, PlottingArea, Plot, Graph, Color};

fn main()
{
    // Define ODE
    // y' = 2x = f(x, y)
    let f = |x: &f64, _y: &Vector<f64> | -> Vector<f64> { return Vector::new_row(1, vec![1.0]) * (x * &2.0f64); };

    // Initial condition
    // t(0) = 1.0
    let init: Vector<f64> = vector![1.0];

    // Create solver instance
    let solver: RK4<f64> = RK4::new(0.01);

    // solve ODE f with initial conditions init in range 0.0, 2.0
    let (x, y): (Vector<f64>, Matrix<f64>) = solver.solve(f, init, 0.0, 2.0);
    let (x_m, _x_n) = x.dim();

    let mut graph: Vec<(f64, f64)> = Vec::new();

    for i in 0..x_m
    {
        let x_i = *x.get(i);
        let y_i = *y.get(i, 0);
        graph.push((x_i, y_i));
    }

    //Create chart
    let mut figure: Figure = Figure::new(1024, 768);
    let area: &mut PlottingArea = figure.get_plottingarea();
    let plot: &mut Plot = area.get_plot();

    plot.set_x_range(0.0f64, 2.0f64).set_y_range(1.0f64, 5.0f64);

    plot.add_graph(Graph::new(graph, Color::Black));

    plot.set_x_axis_designator("Time t");
    plot.set_y_axis_designator("x(t)");
    plot.set_title("y' = 2x");

    figure.save("../figure/ode_simple.png").unwrap();
}