use special::gamma;


/// Hypergeometrical function
///
///
/// # Arguments
///
/// * `a` > 0.0
/// * `b` > 0.0
/// * `c` > 0.0
/// * `z` < 1.0
///
/// # Panics
///
/// if the argument conditions are not fulfilled
///
/// Fore more information:
/// <a href="https://en.wikipedia.org/wiki/Hypergeometric_function">https://en.wikipedia.org/wiki/Hypergeometric_function</a>
///
/// The implementation follows the suggested algorithm in the follow:
/// Numerical methods for the computation of the confluent and Gauss hypergeometric functions
/// Pearson et al. 20
/// Taylor series, Method a)
/// <a href="https://arxiv.org/abs/1407.7786">https://arxiv.org/abs/1407.7786</a>
/// https://arxiv.org/abs/1407.7786
///
/// https://cran.r-project.org/web/packages/hypergeo/vignettes/hypergeometric.pdf
/// http://people.maths.ox.ac.uk/porterm/research/pearson_final.pdf
pub fn f21(a: f64, b: f64, c: f64, z: f64) -> f64
{
	if a <= 0.0 || b <= 0.0 || c <= 0.0 || z > 1.0
	{
		panic!();
	}

	if z == 1.0
	{
		if c - a  - b  < 0.0
		{
			return gamma::gamma(c) * gamma::gamma(a + b -c) / (gamma::gamma(a) * gamma::gamma(b));
		}
		else
		{
			if c - a - b == 0.0
			{
				return gamma::gamma(c) / (gamma::gamma(a) * gamma::gamma(b))
			}
			else
			{
				return gamma::gamma(c) * gamma::gamma(c - a - b) / (gamma::gamma(c - a) * gamma::gamma(c - b));
			}
		}
	}


	let f: f64;
	if -std::f64::INFINITY < z && z < -1.0
	{
		let l1: f64 = (1.0 - z).powf(-a) * gamma::gamma(c) * gamma::gamma(b - a) / (gamma::gamma(b) * gamma::gamma(c -
		a));

		let l2: f64 = (1.0 - z).powf(-b) * gamma::gamma(c) * gamma::gamma(a - b) / (gamma::gamma(a) * gamma::gamma(c -
		b));

		let f1: f64 = f21_norm(a, c - b, a - b + 1.0, 1.0 / (1.0 - z));
		println!("f1: {}", f1);
		let f2: f64 = f21_norm(b, c - a, b - a + 1.0, 1.0 / (1.0 - z));

		f = l1 * f1 + l2 * f2;
	}
	else
	{
		if -1.0 <= z && z < 0.0
		{
			f = f21_norm(a, c - b, c, z / (z - 1.0)) * (1.0 - z).powf(-a);
		}
		else
		{
			if 0.0 <= z && z <= 0.5
			{
				f = f21_norm(a, b, c, z);
			}
			else
			{
				if 0.5 < z && z <= 1.0
				{
					let l1: f64 = gamma::gamma(c) * gamma::gamma(c-a-b) / (gamma::gamma(c - a) * gamma::gamma(c - b));
					let l2: f64 = (1.0 - z).powf(c - a - b) * gamma::gamma(c) * gamma::gamma(a + b -c) / (gamma::gamma
					(a) *
					gamma::gamma(b));
					let f1: f64 = f21_norm(a, b, a + b - c + 1.0, 1.0 - z);
					let f2: f64 = f21_norm(c - a, c - b, c - a - b + 1.0, 1.0 - z);

					f = l1 * f1 + l2 * f2;
				}
				else
				{
					if 1.0 < z && z <= 2.0
					{
						//complex numbers are not supported
//						let l1: f64 = gamma::gamma(c) * gamma::gamma(c - a - b) / (gamma::gamma(c - a) * gamma::gamma
//						(c - b)) * z.powf(-a);
//						println!("{}", l1);
//						let l2: f64 = (1.0 - z).powf(c - a - b) * z.powf(a - c) * gamma::gamma(c) * gamma::gamma(a +
//						b - c) / (gamma::gamma(a) * gamma::gamma(b));
//						println!("{}", l2);
//						let f1: f64 = f21_norm(a, a - c + 1.0, a + b - c + 1.0, 1.0 - 1.0 / z);
//						println!("{}", f1);
//						let f2: f64 = f21_norm(c - a, 1.0 - a, c - a - b + 1.0, 1.0 - 1.0 / z);
//						println!("{}", f2);
//
//						f = l1 * f1 + l2 * f2;
						f = 0.0;
					}
					// 2.0 < z && z < std::f64::INFINITY
					else
					{
						//complex numbers are not supported
//						let l1: f64 = gamma::gamma(c) * gamma::gamma(b - a) / (gamma::gamma(b) * gamma::gamma
//						(c - b)) * (-a * z.ln()).exp();
//						println!("l1: {}", l1);
//						let l2: f64 = gamma::gamma(c) * gamma::gamma(a - b) / (gamma::gamma(a) * gamma::gamma(c - b))
//						 * (-b * z.ln()).exp();
//						 println!("l2: {}", l2);
//						let f1: f64 = f21_norm(a, 1.0 - c + a, 1.0 - b + a, 1.0 / z);
//						let f2: f64 = f21_norm(b, 1.0 - c + b, 1.0 - a + b, 1.0 / z);
//
//						f = l1 * f1 + l2 * f2;
						f = 0.0;
					}
				}
			}
		}
	}

	return f;
}

fn f21_norm(a: f64, b: f64, c: f64, z: f64) -> f64
{

	let mut c_i: f64 = 1.0;
	let mut s_i: f64 = c_i;
	let mut s_i_p: f64 = s_i; 	//s_{i-1}
	let tolerance: f64 = 0.0000000000000002;
	let mut j: f64 = 0.0;

	while c_i.abs()/s_i_p.abs() > tolerance
	{
		let k: f64 = (a + j) * (b + j) / (c + j);
		let l: f64 = z / (j + 1.0);
		c_i = c_i * k * l;
		s_i_p = s_i;
		s_i += c_i;

		j += 1.0;
	}
	return s_i
}

