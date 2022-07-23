use crate::algebra::abstr::Real;
use crate::algebra::linear::Vector;

#[derive(Clone)]
pub struct ExplicitInitialValueProblem<'a, T>
where
    T: Real,
{
    ode: &'a (dyn Fn(&T, &Vector<T>) -> Vector<T> + 'a),
    t_start: T,
    init_cond: Vector<T>,
    t_end: Option<T>,
    callback: Option<&'a (dyn Fn(&T, &Vector<T>) -> bool + 'a)>,
    //dense_output: Option<Vec<T>>
}

impl<'a, T> ExplicitInitialValueProblem<'a, T>
where
    T: Real,
{
    pub fn ode(&self) -> &'a dyn Fn(&T, &Vector<T>) -> Vector<T> {
        self.ode
    }

    pub fn t_start(&self) -> T {
        self.t_start
    }

    pub fn t_end(&self) -> Option<T> {
        self.t_end
    }

    pub fn callback(&self) -> Option<&'a dyn Fn(&T, &Vector<T>) -> bool> {
        self.callback
    }

    pub fn init_cond(&self) -> Vector<T> {
        self.init_cond.clone()
    }
}

#[derive(Clone)]
pub struct ExplicitInitialValueProblemBuilder<'a, T>
where
    T: Real,
{
    ode: &'a (dyn Fn(&T, &Vector<T>) -> Vector<T> + 'a),
    t_start: T,
    init_cond: Vector<T>,
    t_end: Option<T>,
    callback: Option<&'a (dyn Fn(&T, &Vector<T>) -> bool + 'a)>,
    //dense_output: Option<Vec<T>>
}

impl<'a, T> ExplicitInitialValueProblemBuilder<'a, T>
where
    T: Real,
{
    ///
    pub fn new(
        ode: &'a (dyn Fn(&T, &Vector<T>) -> Vector<T> + 'a),
        t_start: T,
        init_cond: Vector<T>,
    ) -> ExplicitInitialValueProblemBuilder<'a, T> {
        ExplicitInitialValueProblemBuilder {
            ode: ode,
            t_start,
            init_cond,
            callback: None,
            t_end: None,
            //dense_output: None,
        }
    }

    pub fn t_end(&mut self, t: T) -> &mut Self {
        self.t_end = Some(t);
        self
    }

    pub fn callback(&mut self, callback: &'a (dyn Fn(&T, &Vector<T>) -> bool + 'a)) -> &mut Self {
        self.callback = Some(callback);
        self
    }

    ///
    /// # Panics
    ///
    pub fn build(&self) -> ExplicitInitialValueProblem<'a, T> {
        if self.callback.is_none() && self.t_end.is_none() {
            panic!("Either callback or t_end has to be set")
        }

        if !self.t_end.is_none() {
            let t_start = self.t_start;
            let t_end_l = self.t_end.unwrap();

            if t_start > t_end_l {
                panic!("The beginning of the time span is greater that the end");
            }
        }

        ExplicitInitialValueProblem {
            ode: self.ode,
            t_start: self.t_start,
            init_cond: self.init_cond.clone(),
            t_end: self.t_end,
            callback: self.callback,
            //dense_output: self.dense_output.clone()
        }
    }
}
