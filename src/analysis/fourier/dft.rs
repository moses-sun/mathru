use crate::algebra::abstr::{Complex, Real, Zero};

pub struct DiscreteFourierTransformation<T> {
    coefs: Vec<Complex<T>>,
}

impl<T> DiscreteFourierTransformation<T>
where
    T: Real,
{
    pub fn new(length: u8) -> DiscreteFourierTransformation<T> {
        let num_coefs = 2u32.pow(length as u32);

        DiscreteFourierTransformation {
            coefs: Self::calc_coefs(num_coefs),
        }
    }
    pub fn transform(&self, input: &Vec<T>) -> Vec<Complex<T>> {
        let length = self.coefs.len();

        (0..=length - 1)
            .map(|k| -> Complex<T> {
                (0..=(length - 1)).fold(Complex::zero(), |s, n| -> Complex<T> {
                    let idx = (n * k) % (length);
                    s + Complex::new(input[n], T::zero()) * self.coefs[idx]
                })
            })
            .collect::<Vec<Complex<T>>>()
    }

    fn calc_coefs(num_coefs: u32) -> Vec<Complex<T>> {
        (0..num_coefs)
            .map(|n| -> Complex<T> {
                let phi = T::from_u8(2) * T::pi() * T::from_u32(n) / T::from_u32(num_coefs);
                Complex {
                    re: phi.cos(),
                    im: -phi.sin(),
                }
            })
            .collect::<Vec<Complex<T>>>()
    }
}
