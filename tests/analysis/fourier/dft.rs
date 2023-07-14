use mathru::algebra::abstr::Complex;
use mathru::analysis::fourier::DiscreteFourierTransformation;

#[test]
fn dft() {
    let dft: DiscreteFourierTransformation<f64> = DiscreteFourierTransformation::new(2);

    let td = vec![1.0, -2.0, 1.5, 8.0];

    let fd = dft.transform(&td);

    let fd_ref = vec![
        Complex::new(8.5, 0.0),
        Complex::new(-0.5, 10.0),
        Complex::new(-3.5, 0.0),
        Complex::new(-0.5, -10.0),
    ];

    fd_ref.iter().zip(fd.iter()).for_each(|(fd_ref_i, fd_i)| {
        assert_abs_diff_eq!(
            *fd_ref_i,
            *fd_i,
            epsilon = Complex::new(10.0 * f64::EPSILON, 10.0f64 * f64::EPSILON)
        )
    });
}
