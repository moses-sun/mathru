//! Often used ODEs
use mathru::{
    algebra::linear::Vector,
    analysis::differential_equation::ordinary::{
        ExplicitInitialValueProblem, ExplicitInitialValueProblemBuilder,
    },
};
use std::f64;

/// Define ODE
/// $x^{'}(t) = x^2 + 1 = f(t, x) $
/// $x(t) = tan(c+t)$
pub fn explicit_ode() -> ExplicitInitialValueProblem<'static, f64> {
    ExplicitInitialValueProblemBuilder::new(
        &|_t, x| {
            x.clone().apply(&|e: &f64| -> f64 {
                return e * e + 1.0;
            })
        },
        0.0,
        vector![0.0],
    )
    .t_end(1.4)
    .build()
}

/// Define ODE
/// $x^{'}(t) = 2x(t) = f(t, x)$
/// $x(t) = C e^{at}$
/// $x^'(t) = a x(t) $
/// $x(t_{s}) = C e^{at_s} => C = \frac{y(t_s)}{e^{at_s}}$
pub fn explicit_ode1() -> ExplicitInitialValueProblem<'static, f64> {
    ExplicitInitialValueProblemBuilder::new(
        &|_t, x| {
            return x * &2.0f64;
        },
        0.0,
        vector![0.5; 2.0],
    )
    .t_end(2.0)
    .build()
}
