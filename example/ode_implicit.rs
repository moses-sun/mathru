use mathru::algebra::linear::{Vector};
use mathru::analysis::ode::{ImplicitODE, ImplicitEuler};
use mathru::analysis::ode::problem::VanDerPolOsc;
use plotters::prelude::*;

fn main()
{
	// Create an ODE instance
    let problem: VanDerPolOsc<f64> = VanDerPolOsc::default();

	// Create a ODE solver instance
    let solver: ImplicitEuler<f64> = ImplicitEuler::new(0.0005);

	// Solve ODE
	let (t, x): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem).unwrap();

	//Create chart
    let mut graph_x1: Vec<(f64, f64)> = Vec::with_capacity(x.len());
	let mut graph_x2: Vec<(f64, f64)> = Vec::with_capacity(x.len());

    for i in 0..x.len()
    {
        let t_i = t[i];
        graph_x1.push((t_i, *x[i].get(0)));
       	graph_x2.push((t_i, *x[i].get(1)));
    }

	let root_area = BitMapBackend::new("../figure/ode_implicit.png", (1200, 800))
        .into_drawing_area();
    root_area.fill(&WHITE).unwrap();

	let (t_start, t_end) = problem.time_span();

    let mut ctx = ChartBuilder::on(&root_area)
		.margin(20)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("ODE solved with implicit Euler", ("Arial", 40))
        .build_ranged(t_start..t_end, -2.0f64..2.0f64)
        .unwrap();

  	ctx.configure_mesh()
  		.x_desc("Time t")
		.y_desc("x, x'")
  		.axis_desc_style(("sans-serif", 25).into_font())
       	.draw().unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(
        LineSeries::new(graph_x1, &BLACK)
    ).unwrap();

    ctx.draw_series(
        LineSeries::new(graph_x2, &RED)
    ).unwrap();
}