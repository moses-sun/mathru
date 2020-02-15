use mathru::*;
use mathru::algebra::linear::{Vector, Matrix};
use mathru_plot::{Figure, PlottingArea, Plot, Graph, Color};
use mathru::statistics::distrib::{Distribution, Normal};
use mathru::optimization::{Jacobian, LevenbergMarquardt};


///y = a + b * exp(c * t) = f(t)
///
///
pub struct Example
{
	x: Vector<f64>,
	y: Vector<f64>
}

impl Example
{
	pub fn new(x: Vector<f64>, y: Vector<f64>) -> Example
	{
		Example
		{
			x: x,
			y: y
		}
	}

	pub fn function(x: f64, beta: &Vector<f64>) -> f64
	{
		let beta_0: f64 = *beta.get(0);
		let beta_1: f64 = *beta.get(1);
		let beta_2: f64 = *beta.get(2);
		let f_x: f64 = beta_0 + beta_1 * (beta_2 * x).exp();

		return f_x;
	}
}

impl Jacobian<f64> for Example
{
	// y(x_i) - f(x_i)
	fn eval(self: &Self, beta: &Vector<f64>) -> Vector<f64>
	{
		let f_x = self.x.clone().apply(&|x: &f64| Example::function(*x, beta));
		let r: Vector<f64> = &self.y - &f_x;
		return vector![r.dotp(&r)]
	}

	fn jacobian(self: &Self, beta: &Vector<f64>) -> Matrix<f64>
	{
		let (x_m, _x_n) = self.x.dim();
		let (beta_m, _beta_n) = beta.dim();

		let mut jacobian_f: Matrix<f64> = Matrix::zero(x_m, beta_m);

		let f_x = self.x.clone().apply(&|x: &f64| Example::function(*x, beta));
		let residual: Vector<f64> = &self.y - &f_x;

		for i in 0..x_m
		{
			//let beta_0: f64 = *beta.get(0);
			let beta_1: f64 = *beta.get(1);
			let beta_2: f64 = *beta.get(2);

			let x_i: f64 = *self.x.get(i);

			*jacobian_f.get_mut(i, 0) = 1.0;
			*jacobian_f.get_mut(i, 1) = (beta_2 * x_i).exp();
			*jacobian_f.get_mut(i, 2) = beta_1 * x_i * (beta_2 * x_i).exp();

		}

		let jacobian: Matrix<f64> = (residual.transpose() * jacobian_f * -2.0).into();
		return jacobian;
	}
}


fn main()
{
	let num_samples: usize = 100;

	let noise: Normal = Normal::new(0.0, 0.05);

	let mut t_vec: Vec<f64> = Vec::with_capacity(num_samples);
	// Start time
	let t_0 = 0.0f64;
	// End time
	let t_1 = 5.0f64;

	let mut y_vec: Vec<f64> = Vec::with_capacity(num_samples);

	// True function parameters
	let beta: Vector<f64> = vector![0.5; 5.0; -1.0];

	for i in 0..num_samples
	{
		let t_i: f64 = (t_1 - t_0) / (num_samples as f64) * (i as f64);

		//Add some noise
		y_vec.push(Example::function(t_i, &beta) + noise.random());

		t_vec.push(t_i);
	}

	let t: Vector<f64> = Vector::new_column(num_samples, t_vec.clone());
	let y: Vector<f64> = Vector::new_column(num_samples, y_vec.clone());

	let example_function = Example::new(t, y);

	let optim: LevenbergMarquardt<f64> = LevenbergMarquardt::new(100, 0.3, 0.95);
	let beta_opt: Vector<f64> = optim.minimize(&example_function, &vector![-1.5; 1.0; -2.0]).arg();


  	let mut graph_y: Vec<(f64, f64)> = Vec::new();

    for i in 0..num_samples
    {
        graph_y.push((t_vec[i], y_vec[i]));
    }

    let mut graph_y_hat: Vec<(f64, f64)> = Vec::new();

    for i in 0..num_samples
    {
    	let t_i: f64 = t_vec[i];
        let y_hat_i = Example::function(t_i, &beta_opt);
        graph_y_hat.push((t_i, y_hat_i));
    }

    println!("{}", beta_opt);

	//Create chart
    let mut figure: Figure = Figure::new(1024, 768);
    let area: &mut PlottingArea = figure.get_plottingarea();
    let plot: &mut Plot = area.get_plot();

    plot.set_x_range(t_0, t_1);
    plot.set_y_range(-0.5f64, 6.0f64);

    plot.add_graph(Graph::new(graph_y, Color::Black));
    plot.add_graph(Graph::new(graph_y_hat, Color::Red));

    plot.set_x_axis_designator("Time t");
    plot.set_y_axis_designator("x(t)");
    plot.set_title("Fitting with Levenberg-Marquardt");

    figure.save("../figure/fit.png").unwrap();
}