use mathru::analysis::integral::gauss_kronrod::RootWeight;

#[test]
fn root_weight_0() {
    let roots = RootWeight::<f64>::new();
    let mut it = roots.iter();

    assert_eq!(it.next(), Some((-0.991455371120813, 0.022935322010529)));
    assert_eq!(it.next(), Some((-0.949107912342759, 0.063092092629979)));
    it.next();
    it.next();
    it.next();
    it.next();
    it.next();
    assert_eq!(it.next(), Some((0.0, 0.209482141084728)));
    assert_eq!(it.next(), Some((0.207784955007898, 0.204432940075298)));
    it.next();
    it.next();
    it.next();
    it.next();
    it.next();
    assert_eq!(it.next(), Some((0.991455371120813, 0.022935322010529)));
    assert_eq!(it.next(), None);
}
