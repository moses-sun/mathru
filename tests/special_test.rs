extern crate mathru;

#[cfg(test)]
mod special_test
{
    use mathru::special;



    #[test]
    fn beta0()
    {
        let x: f64 = 5.0;
        let y: f64 = 6.5;

        let beta: f64 = special::beta::beta(x, y);

        assert_eq!(0.0005806371131448529, beta);
    }

    #[test]
    fn gamma0()
    {
        let x: f64 = 0.5;

        let gamma: f64 = special::gamma::gamma(x);

        assert_eq!(1.7724538509055163, gamma);
    }

    #[test]
    fn digamma0()
    {
        let x: f64 = 0.1;

        let gamma: f64 = special::gamma::digamma(x);

        assert_eq!(-10.423754940411138, gamma);
    }

    #[test]
    fn digamma1()
    {
        let x: f64 = 1.7;

        let gamma: f64 = special::gamma::digamma(x);

        assert_eq!(0.20854787487338688, gamma);
    }

      #[test]
    fn gamma_ur0()
    {
        let x: f64 = 1.5;
        let a: f64 = 1.0;

        let gamma: f64 = special::gamma::gamma_ur(a, x);

        assert_eq!(0.2231301601484299, gamma);
    }

    #[test]
    fn gamma_lr0()
    {
        let x: f64 = 1.5;
        let a: f64 = 1.0;

        let gamma: f64 = special::gamma::gamma_lr(a, x);

        assert_eq!(0.7768698398515701, gamma);
    }


}