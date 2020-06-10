use crate::{
    algebra::abstr::Real,
    statistics::{
        distrib::{ChiSquared as ChiSquaredDistrib, Continuous},
        test::Test,
    },
};

/// Chi-Squared Test
///
/// Fore more information:
/// <a href="https://en.wikipedia.org/wiki/Chi-squared_test">https://en.wikipedia.org/wiki/Chi-squared_test</a>
pub struct ChiSquared<T>
{
    df: u32,
    chi_squared: T,
}

impl<T> ChiSquared<T> where T: Real
{
    ///
    /// alpha: significance level
    pub fn test_vector(x: &Vec<T>, y: &Vec<T>) -> ChiSquared<T>
    {
        if x.len() != y.len()
        {
            panic!();
        }

        let df: u32 = (y.len() - 1) as u32;

        let mut sum_x: T = T::zero();
        for n_i in x.iter()
        {
            sum_x += *n_i;
        }

        let mut sum_y: T = T::zero();
        for n_j in y.iter()
        {
            sum_y += *n_j;
        }

        let n: T = sum_x + sum_y;

        let mut chi_squared: T = T::zero();
        let m: usize = x.len();
        for j in 0..m
        {
            for k in 0..2
            {
                let n_jk: T;
                let mut n_k: T = T::zero();
                if k == 0
                {
                    n_jk = x[j];
                    for l in 0..m
                    {
                        n_k += x[l];
                    }
                }
                else
                {
                    n_jk = y[j];
                    for l in 0..m
                    {
                        n_k += y[l];
                    }
                }

                let n_j_: T = x[j] + y[j];

                let n_jks: T = (n_k * n_j_) / (n);
                chi_squared += (n_jk - n_jks).pow(&T::from_f64(2.0)) / n_jks
            }
        }

        ChiSquared { chi_squared, df }
    }
}

impl<T> Test<T> for ChiSquared<T> where T: Real
{
    fn df(self: &Self) -> u32
    {
        self.df
    }

    fn value(self: &Self) -> T
    {
        self.chi_squared
    }

    fn p_value(self: &Self) -> T
    {
        let distrib: ChiSquaredDistrib<T> = ChiSquaredDistrib::new(self.df);
        T::one() - distrib.cdf(self.chi_squared)
    }
}
