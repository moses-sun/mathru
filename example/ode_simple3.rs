use mathru::*;
use mathru::algebra::linear::{Vector};
use mathru::analysis::ode::{DormandPrince54};
use mathru_plot::{Figure, PlottingArea, Plot, Graph, Color};
use mathru::analysis::ode::ExplicitODE;

/// Define ODE
///
/// $`\frac{\partial y}{\partial x} = \frac{5x^{2} - y}{exp{x + y}} `$
pub struct ExplicitODE1
{
	time_span: (f64, f64),
	init_cond: Vector<f64>
}

impl Default for ExplicitODE1
{
	fn default() -> ExplicitODE1
	{
		ExplicitODE1
		{
			time_span: (0.0, 10.0),
			init_cond: vector![1.0],
		}
	}
}

impl ExplicitODE<f64> for ExplicitODE1
{
   	fn func(self: &Self, t: &f64, x: &Vector<f64>) -> Vector<f64>
	{
		return vector!((5.0 * t * t - *x.get(0)) / (t + *x.get(0)).exp());
	}

    fn time_span(self: &Self) -> (f64, f64)
	{
		return self.time_span;
	}

    fn init_cond(self: &Self) -> Vector<f64>
	{
		return self.init_cond.clone();
	}
}

fn main()
{
	let problem: ExplicitODE1 = ExplicitODE1::default();

	let h_0: f64 = 0.001;
	let n_max: u32 = 300;
	let abs_tol: f64 = 0.00000001;

	let solver: DormandPrince54<f64> = DormandPrince54::new(abs_tol, h_0, n_max);

	let (t, x): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem).unwrap();

    let len = t.len();

    let mut graph: Vec<(f64, f64)> = Vec::new();

    for i in 0..len
    {
        let t_i = t[i];
        let x_i = *x[i].get(0);
        graph.push((t_i, x_i));
    }

    //Create chart
    let mut figure: Figure = Figure::new(1024, 768);
    let area: &mut PlottingArea = figure.get_plottingarea();
    let plot: &mut Plot = area.get_plot();

    plot.set_x_range(0.0f64, 10.0f64).set_y_range(0.5f64, 2.5f64);

    plot.add_graph(Graph::new(graph, Color::Black));

    plot.set_x_axis_designator("Time t");
    plot.set_y_axis_designator("x(t)");
    plot.set_title("y'=(5t^2 - x)/(e^(t + x))");

    figure.save("../figure/ode_simple3.png").unwrap();
}