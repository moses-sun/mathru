use mathru::*;
use mathru::algebra::linear::{Vector};
use mathru::analysis::ode::{Solver, RK4};
use mathru_plot::{Figure, PlottingArea, Plot, Graph, Color};

/// Define ODE
///
/// $`\frac{\partial y}{\partial x} = \frac{5x^{2} - y}{exp{x + y}} `$
fn main()
{

    let f = |x: &f64, y: &Vector<f64> | -> Vector<f64> { return vector!((5.0 * x * x - *y.get(0)) / (x + *y.get(0)).exp()); };

    // Initial condition
    // y(0) = 1.0
    let init: Vector<f64> = vector![1.0];

    // Create solver instance
    let solver: RK4<f64> = RK4::new(0.01);

    // solve ODE f with initial conditions init in range 0.0, 10.0
    let (x, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(f, init, 0.0, 10.0).unwrap();
    let len = x.len();

    let mut graph: Vec<(f64, f64)> = Vec::new();

    for i in 0..len
    {
        let x_i = x[i];
        let y_i = *y[i].get(0);
        graph.push((x_i, y_i));
    }

    //Create chart
    let mut figure: Figure = Figure::new(1024, 768);
    let area: &mut PlottingArea = figure.get_plottingarea();
    let plot: &mut Plot = area.get_plot();

    plot.set_x_range(0.0f64, 10.0f64).set_y_range(0.5f64, 2.5f64);

    plot.add_graph(Graph::new(graph, Color::Black));

    plot.set_x_axis_designator("Time t");
    plot.set_y_axis_designator("x(t)");
    plot.set_title("y'=(5x^2 - y)/(e^(x + y))");

    figure.save("../figure/ode_simple3.png").unwrap();
}