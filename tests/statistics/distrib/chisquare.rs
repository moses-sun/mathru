use mathru::statistics::distrib::{ChiSquare, Continuous};

#[test]
fn pdf0()
{
    let df: u32 = 4;
    let distrib: ChiSquare<f64> = ChiSquare::new(df);
    let x: f64 = 0.0;
    let prob: f64 = distrib.pdf(x);

    assert_relative_eq!(0.0, prob);
}

#[test]
fn pdf1()
{
    let df: u32 = 4;
    let distrib: ChiSquare<f64> = ChiSquare::new(df);
    let x: f64 = 1.0;
    let prob: f64 = distrib.pdf(x);

    assert_relative_eq!(0.1516326649281583, prob);
}

#[test]
fn cdf0()
{
    let df: u32 = 4;
    let distrib: ChiSquare<f64> = ChiSquare::new(df);

    assert_relative_eq!(0.09020401043104986, distrib.cdf(1.0))
}

#[test]
fn cdf1()
{
    let df: u32 = 4;
    let distrib: ChiSquare<f64> = ChiSquare::new(df);

    assert_relative_eq!(0.001209104274250028, distrib.cdf(0.1))
}

#[test]
fn cdf2()
{
    let df: u32 = 4;
    let distrib: ChiSquare<f64> = ChiSquare::new(df);

    assert_relative_eq!(0.7689217620241717, distrib.cdf(5.6))
}

//Very inaccurate
#[test]
fn cdf3()
{
    let df: u32 = 3;

    let distrib: ChiSquare<f64> = ChiSquare::new(df);

    assert_relative_eq!(0.08110858834532447, distrib.cdf(0.5), epsilon=3.0 * f64::EPSILON);
}

//Very inaccurate
#[test]
fn cdf4()
{
    let df: u32 = 3;

    let distrib: ChiSquare<f64> = ChiSquare::new(df);

    assert_relative_eq!(0.9539882943107686, distrib.cdf(8.0), epsilon=3.0 * f64::EPSILON);
}

#[test]
fn quantile()
{
    let df: u32 = 4;

    let distrib: ChiSquare<f64> = ChiSquare::new(df);

    assert_relative_eq!(7.711853942311415, distrib.quantile(0.9))
}

#[test]
fn quantile1()
{
    let df: u32 = 4;

    let distrib: ChiSquare<f64> = ChiSquare::new(df);

    assert_relative_eq!(0.7107, distrib.quantile(0.05))
}

#[test]
fn quantile_4()
{
    let df: u32 = 10;

    let distrib: ChiSquare<f64> = ChiSquare::new(df);

    assert_relative_eq!(18.307, distrib.quantile(0.95))
}

#[test]
fn quantile_2()
{
    let df: u32 = 7;

    let distrib: ChiSquare<f64> = ChiSquare::new(df);

    assert_relative_eq!(14.07, distrib.quantile(0.95), max_relative=0.00001)
}


#[test]
fn quantile_3()
{
    let df: u32 = 7;

    let distrib: ChiSquare<f64> = ChiSquare::new(df);

    assert_relative_eq!(2.1673, distrib.quantile(0.05), max_relative=0.00001)
}
