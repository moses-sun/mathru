extern crate mathru;

#[cfg(test)]
mod poissondistrib
{
    use mathru::stats::distrib::{Discrete, Poisson};

    #[test]
    fn pmf0()
    {
        let gamma: f64 = 1.0;
        let x: u32 = 0;
        let distrib : Poisson = Poisson::new(&gamma);
        let prob: f64 = distrib.pmf(x);

        assert_eq!(0.36787944117144233, prob);
    }

    #[test]
    fn pmf1()
    {
        let gamma: f64 = 3.0;
        let x: u32 = 5;
        let distrib : Poisson = Poisson::new(&gamma);
        let prob: f64 = distrib.pmf(x);

        assert_eq!(0.10081881344492448, prob);
    }

    #[test]
    fn cdf1()
    {
        let gamma: f64 = 5.0;
        let x: u32 = 5;
        let distrib : Poisson = Poisson::new(&gamma);
        let prob: f64 = distrib.cdf(x);

			assert_eq!(0.6159606548330621, prob);
    }

//// Does not work all the time, because the used function random is not mocked.
////    #[test]
////    fn random()
////    {
////        let mean_1 : f64 = 0.0;
////        let variance_1: f64 = 1.0;
////        let distrib_1 : NormalDistrib = NormalDistrib::new(&mean_1, &variance_1);
////        let mut data: Vec<f64> = Vec::new();
////
////        for _i in 0..10000
////        {
////            data.push(distrib_1.random());
////        }
////
////        let distrib_2: NormalDistrib = NormalDistrib::from_data(&data);
////        println!("{}", distrib_2.variance());
////        assert!(distrib_2.mean() < 0.01);
////        assert!(distrib_2.mean() > -0.01);
////        assert!(distrib_2.variance() < 1.02 * variance_1);
////        assert!(distrib_2.variance() > 0.98 * variance_1);
////    }
//

//
//    #[test]
//    fn quantile0()
//    {
//        let mean: f64 = 0.0;
//        let variance: f64 = 1.0;
//        let distrib: Normal = Normal::new(&mean, &variance);
//
//        assert_eq!(1.2815515655446006, distrib.quantile(0.9));
//    }
//
//   	#[test]
//    fn quantile1()
//    {
//        let mean: f64 = 1.0;
//        let variance: f64 = 0.5;
//        let distrib: Normal = Normal::new(&mean, &variance);
//
//        assert_eq!(1.0, distrib.quantile(0.5));
//    }
}