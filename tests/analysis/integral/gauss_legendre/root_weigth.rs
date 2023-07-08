use mathru::analysis::integral::gauss_legendre::RootWeight;

#[test]
fn root_weight_bessel() {
    assert_relative_eq!(2.40482555769577276862163187933f64, RootWeight::bessel_0(1));
    assert_relative_eq!(5.520078110286311, RootWeight::bessel_0(2));
    assert_relative_eq!(62.0484691902271698828525002646f64, RootWeight::bessel_0(20));
    assert_relative_eq!(65.18996479978959, RootWeight::bessel_0(21));
}

#[test]
fn root_weight_n_eq_1() {
    let rw = RootWeight::<f64>::new(1);
    let roots_ref: Vec<f64> = vec![0.0];
    let weights_ref: Vec<f64> = vec![2.0];

    roots_ref
        .iter()
        .zip(rw.roots.iter())
        .for_each(|(r_ref, r)| {
            assert_relative_eq!(*r_ref, *r, epsilon = 1.0e-10f64);
        });

    weights_ref
        .iter()
        .zip(rw.weights.iter())
        .for_each(|(w_ref, w)| {
            assert_relative_eq!(*w_ref, *w, epsilon = 1.0e-10f64);
        });
}

#[test]
fn root_weight_n_eq_2() {
    let rw = RootWeight::<f64>::new(2);
    let roots_ref: Vec<f64> = vec![1.0 / 3.0f64.sqrt()];
    let weights_ref: Vec<f64> = vec![1.0];

    roots_ref
        .iter()
        .zip(rw.roots.iter())
        .for_each(|(r_ref, r)| {
            assert_relative_eq!(*r_ref, *r, epsilon = 1.0e-10f64);
        });

    weights_ref
        .iter()
        .zip(rw.weights.iter())
        .for_each(|(w_ref, w)| {
            assert_relative_eq!(*w_ref, *w, epsilon = 1.0e-10f64);
        });
}

#[test]
fn root_weight_n_eq_3() {
    let rw = RootWeight::<f64>::new(3);
    let roots_ref: Vec<f64> = vec![0.0, (3.0f64 / 5.0f64).sqrt()];
    let weights_ref: Vec<f64> = vec![8.0 / 9.0, 5.0 / 9.0];

    roots_ref
        .iter()
        .zip(rw.roots.iter())
        .for_each(|(r_ref, r)| {
            assert_relative_eq!(*r_ref, *r, epsilon = 1.0e-10f64);
        });

    weights_ref
        .iter()
        .zip(rw.weights.iter())
        .for_each(|(w_ref, w)| {
            assert_relative_eq!(*w_ref, *w, epsilon = 1.0e-10f64);
        });
}

#[test]
fn root_weight_n_eq_4() {
    let rw = RootWeight::<f64>::new(4);
    let roots_ref: Vec<f64> = vec![0.3399810435848562648027, 0.861136311594052575224];
    let weights_ref: Vec<f64> = vec![0.6521451548625461426269, 0.3478548451374538573731];

    roots_ref
        .iter()
        .zip(rw.roots.iter())
        .for_each(|(r_ref, r)| {
            assert_relative_eq!(*r_ref, *r, epsilon = 1.0e-10f64);
        });

    weights_ref
        .iter()
        .zip(rw.weights.iter())
        .for_each(|(w_ref, w)| {
            assert_relative_eq!(*w_ref, *w, epsilon = 1.0e-10f64);
        });
}

// https://keisan.casio.com/exec/system/1330940731
#[test]
fn root_weight_n_eq_10() {
    let rw = RootWeight::<f64>::new(10);
    let roots_ref: Vec<f64> = vec![
        0.1488743389816312108848,
        0.4333953941292471907993,
        0.6794095682990244062343,
        0.8650633666889845107321,
        0.973906528517171720078,
    ];

    let weights_ref: Vec<f64> = vec![
        0.295524224714752870174,
        0.269266719309996355091,
        0.2190863625159820439955,
        0.1494513491505805931458,
        0.0666713443086881375936,
    ];

    roots_ref
        .iter()
        .zip(rw.roots.iter())
        .for_each(|(r_ref, r)| {
            assert_relative_eq!(*r_ref, *r, epsilon = 1.0e-3f64);
        });

    weights_ref
        .iter()
        .zip(rw.weights.iter())
        .for_each(|(w_ref, w)| {
            assert_relative_eq!(*w_ref, *w, epsilon = 1.0e-4f64);
        });
}
