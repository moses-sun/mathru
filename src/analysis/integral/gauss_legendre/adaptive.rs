use crate::algebra::abstr::Real;

use super::GaussLegendre;

pub struct Adaptive<T> {
    gl_n: GaussLegendre<T>,
    gl_2n_1: GaussLegendre<T>,
    tol: T,
}

impl<T> Adaptive<T>
where
    T: Real,
{
    pub fn new(n: u8, tol: T) -> Adaptive<T> {
        Adaptive {
            gl_n: GaussLegendre::new(n),
            gl_2n_1: GaussLegendre::new(2 * n + 1),
            tol,
        }
    }

    pub fn integrate<F>(&self, f: &F, a: T, b: T) -> T
    where
        F: Fn(T) -> T,
    {
        let g_2n_1 = self.gl_2n_1.integrate(f, a, b);
        let g_n = self.gl_n.integrate(f, a, b);

        if (g_2n_1 - g_n).abs() > self.tol {
            let lim = (a + b) / T::from_u8(2);
            let i_1 = self.integrate(f, a, lim);
            let i_2 = self.integrate(f, lim, b);
            i_1 + i_2
        } else {
            g_n
        }
    }
}
