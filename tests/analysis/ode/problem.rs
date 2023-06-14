//! Often used ODEs
use mathru::{
    algebra::linear::{General, Vector},
    analysis::differential_equation::ordinary::{
        ExplicitInitialValueProblem, ExplicitInitialValueProblemBuilder, ExplicitODE,
        ImplicitInitialValueProblem, ImplicitInitialValueProblemBuilder, ImplicitODE,
    },
};
use std::f64;

/// Define ODE
/// $x^{'}(t) = x^2 + 1 = f(t, x) $
/// $x(t) = tan(c+t)$
pub struct TestOde {}

impl Default for TestOde {
    fn default() -> TestOde {
        TestOde {}
    }
}

impl ExplicitODE<f64> for TestOde {
    fn ode(&self, _x: &f64, y: &Vector<f64>) -> Vector<f64> {
        y.clone().apply(&|e: &f64| -> f64 {
            return e * e + 1.0;
        })
    }
}

impl ImplicitODE<f64> for TestOde {
    fn ode(&self, _x: &f64, y: &Vector<f64>) -> Vector<f64> {
        y.clone().apply(&|e: &f64| -> f64 {
            return e * e + 1.0;
        })
    }

    fn jacobian(&self, x: &f64, y: &Vector<f64>) -> General<f64> {
        General::new(
            1,
            1,
            y.clone()
                .apply(&|_e: &f64| -> f64 { x * &2.0 })
                .convert_to_vec(),
        )
    }
}

static TESTODE: TestOde = TestOde {};

pub fn explicit_ode() -> ExplicitInitialValueProblem<'static, f64, TestOde> {
    ExplicitInitialValueProblemBuilder::new(&TESTODE, 0.0, vector![0.0])
        .t_end(1.4)
        .build()
}

pub fn implicit_ode() -> ImplicitInitialValueProblem<'static, f64, TestOde> {
    ImplicitInitialValueProblemBuilder::new(&TESTODE, 0.0, vector![0.0])
        .t_end(1.4)
        .build()
}

/// Define ODE
/// $x^{'}(t) = 2x(t) = f(t, x)$
/// $x(t) = C e^{at}$
/// $x^'(t) = a x(t) $
/// $x(t_{s}) = C e^{at_s} => C = \frac{y(t_s)}{e^{at_s}}$
///
pub struct ExplicitOde1 {}

impl Default for ExplicitOde1 {
    fn default() -> ExplicitOde1 {
        ExplicitOde1 {}
    }
}

impl ExplicitODE<f64> for ExplicitOde1 {
    fn ode(&self, _x: &f64, y: &Vector<f64>) -> Vector<f64> {
        y * &2.0f64
    }
}

static EXPLICITODE1: ExplicitOde1 = ExplicitOde1 {};

pub fn explicit_ode1() -> ExplicitInitialValueProblem<'static, f64, ExplicitOde1> {
    ExplicitInitialValueProblemBuilder::new(&EXPLICITODE1, 0.0, vector![0.5; 2.0])
        .t_end(2.0)
        .build()
}
