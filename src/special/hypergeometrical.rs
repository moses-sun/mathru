//! Hypergeometrical functions

use crate::special::gamma;
use crate::algebra::abstr::Real;

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
pub fn f21<T>(a: T, b: T, c: T, z: T) -> T
	where T: Real
{
	if a <= T::zero() || b <= T::zero() || c <= T::zero() || z > T::one()
	{
		panic!();
	}

	if z == T::one()
	{
		if c - a  - b  < T::zero()
		{
			return gamma::gamma(c) * gamma::gamma(a + b -c) / (gamma::gamma(a) * gamma::gamma(b));
		}
		else
		{
			if c - a - b == T::zero()
			{
				return gamma::gamma(c) / (gamma::gamma(a) * gamma::gamma(b))
			}
			else
			{
				return gamma::gamma(c) * gamma::gamma(c - a - b) / (gamma::gamma(c - a) * gamma::gamma(c - b));
			}
		}
	}


	let f: T;
	if T::from_f64(-std::f64::INFINITY).unwrap() < z && z < -T::one()
	{
		let l1: T = (T::one() - z).pow(&-a) * gamma::gamma(c) * gamma::gamma(b - a) / (gamma::gamma(b) * gamma::gamma(c -
		a));

		let l2: T = (T::one() - z).pow(&-b) * gamma::gamma(c) * gamma::gamma(a - b) / (gamma::gamma(a) * gamma::gamma(c -
		b));

		let f1: T = f21_norm(a, c - b, a - b + T::one(), T::one() / (T::one() - z));
		let f2: T = f21_norm(b, c - a, b - a + T::one(), T::one() / (T::one() - z));

		f = l1 * f1 + l2 * f2;
	}
	else
	{
		if -T::one() <= z && z < T::zero()
		{
			f = f21_norm(a, c - b, c, z / (z - T::one())) * (T::one() - z).pow(&-a);
		}
		else
		{
			if T::zero() <= z && z <= T::from_f64(0.5).unwrap()
			{
				f = f21_norm(a, b, c, z);
			}
			else
			{
				if T::from_f64(0.5).unwrap() < z && z <= T::one()
				{
					let l1: T = gamma::gamma(c) * gamma::gamma(c-a-b) / (gamma::gamma(c - a) * gamma::gamma(c - b));
					let l2: T = (T::one() - z).pow(&(c - a - b)) * gamma::gamma(c) * gamma::gamma(a + b -c) / (gamma::gamma
					(a) *
					gamma::gamma(b));
					let f1: T = f21_norm(a, b, a + b - c + T::one(), T::one() - z);
					let f2: T = f21_norm(c - a, c - b, c - a - b + T::one(), T::one() - z);

					f = l1 * f1 + l2 * f2;
				}
				else
				{
					if T::one() < z && z <= T::from_f64(2.0).unwrap()
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
						unimplemented!();
						//f = 0.0;
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
						unimplemented!();
						//f = T::zero();
					}
				}
			}
		}
	}

	return f;
}

fn f21_norm<T>(a: T, b: T, c: T, z: T) -> T
	where T: Real
{

	let mut c_i: T = T::one();
	let mut s_i: T = c_i;
	let mut s_i_p: T = s_i; 	//s_{i-1}
	let tolerance: T = T::from_f64(0.0000000000000002).unwrap();
	let mut j: T = T::zero();

	while c_i.abs()/s_i_p.abs() > tolerance
	{
		let k: T = (a + j) * (b + j) / (c + j);
		let l: T = z / (j + T::one());
		c_i = c_i * k * l;
		s_i_p = s_i;
		s_i += c_i;

		j += T::one();
	}
	return s_i
}

