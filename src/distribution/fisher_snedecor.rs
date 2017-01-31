use std::f64;
use rand::Rng;
use rand::distributions::{Sample, IndependentSample};
use function::beta;
use statistics::*;
use distribution::{Univariate, Continuous, Distribution};
use {Result, StatsError};

/// Implements the [Fisher-Snedecor](https://en.wikipedia.org/wiki/F-distribution) distribution
/// also commonly known as the F-distribution
///
/// # Examples
///
/// ```
/// use statrs::distribution::{FisherSnedecor, Continuous};
/// use statrs::statistics::Mean;
/// use statrs::prec;
///
/// let n = FisherSnedecor::new(3.0, 3.0).unwrap();
/// assert_eq!(n.mean(), 3.0);
/// assert!(prec::almost_eq(n.pdf(1.0), 0.318309886183790671538, 1e-15));
/// ```
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FisherSnedecor {
    freedom_1: f64,
    freedom_2: f64,
}

impl FisherSnedecor {
    /// Constructs a new fisher-snedecor distribution with
    /// degrees of freedom `freedom_1` and `freedom_2`
    ///
    /// # Errors
    ///
    /// Returns an error if `freedom_1` or `freedom_2` are `NaN`.
    /// Also returns an error if `freedom_1 <= 0.0` or `freedom_2 <= 0.0`
    ///
    /// # Examples
    ///
    /// ```
    /// use statrs::distribution::FisherSnedecor;
    ///
    /// let mut result = FisherSnedecor::new(1.0, 1.0);
    /// assert!(result.is_ok());
    ///
    /// result = FisherSnedecor::new(0.0, 0.0);
    /// assert!(result.is_err());
    /// ```
    pub fn new(freedom_1: f64, freedom_2: f64) -> Result<FisherSnedecor> {
        if freedom_1.is_nan() || freedom_2.is_nan() {
            Err(StatsError::BadParams)
        } else if freedom_1 <= 0.0 || freedom_2 <= 0.0 {
            Err(StatsError::BadParams)
        } else {
            Ok(FisherSnedecor {
                freedom_1: freedom_1,
                freedom_2: freedom_2,
            })
        }
    }

    /// Returns the first degree of freedom for the
    /// fisher-snedecor distribution
    ///
    /// # Examples
    ///
    /// ```
    /// use statrs::distribution::FisherSnedecor;
    ///
    /// let n = FisherSnedecor::new(2.0, 3.0).unwrap();
    /// assert_eq!(n.freedom_1(), 2.0);
    /// ```
    pub fn freedom_1(&self) -> f64 {
        self.freedom_1
    }

    /// Returns the second degree of freedom for the
    /// fisher-snedecor distribution
    ///
    /// # Examples
    ///
    /// ```
    /// use statrs::distribution::FisherSnedecor;
    ///
    /// let n = FisherSnedecor::new(2.0, 3.0).unwrap();
    /// assert_eq!(n.freedom_2(), 3.0);
    /// ```
    pub fn freedom_2(&self) -> f64 {
        self.freedom_2
    }
}

impl Sample<f64> for FisherSnedecor {
    /// Generate a random sample from a fisher-snedecor distribution
    /// using `r` as the source of randomness.
    /// Refer [here](#method.sample-1) for implementation details.
    fn sample<R: Rng>(&mut self, r: &mut R) -> f64 {
        super::Distribution::sample(self, r)
    }
}

impl IndependentSample<f64> for FisherSnedecor {
    /// Generate a random independent sample from a fisher-snedecor distribution
    /// using `r` as the source of randomness.
    /// Refer [here](#method.sample-1) for implementation details.
    fn ind_sample<R: Rng>(&self, r: &mut R) -> f64 {
        super::Distribution::sample(self, r)
    }
}

impl Distribution<f64> for FisherSnedecor {
    /// Generate a random sample from a fisher-snedecor distribution using
    /// `r` as the source of randomness.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate rand;
    /// # extern crate statrs;
    /// use rand::StdRng;
    /// use statrs::distribution::{FisherSnedecor, Distribution};
    ///
    /// # fn main() {
    /// let mut r = rand::StdRng::new().unwrap();
    /// let n = FisherSnedecor::new(2.0, 2.0).unwrap();
    /// print!("{}", n.sample::<StdRng>(&mut r));
    /// # }
    /// ```
    fn sample<R: Rng>(&self, r: &mut R) -> f64 {
        (super::gamma::sample_unchecked(r, self.freedom_1 / 2.0, 0.5) * self.freedom_2) /
        (super::gamma::sample_unchecked(r, self.freedom_2 / 2.0, 0.5) * self.freedom_1)
    }
}

impl Univariate<f64, f64> for FisherSnedecor {
    /// Calculates the cumulative distribution function for the fisher-snedecor distribution
    /// at `x`
    ///
    /// # Remarks
    ///
    /// Returns `NaN` if `freedom_1`, `freedom_2` is `INF`, or `x` is `+INF` or `-INF`
    ///
    /// # Formula
    ///
    /// ```ignore
    /// I_((d1 * x) / (d1 * x + d2))(d1 / 2, d2 / 2)
    /// ```
    ///
    /// where `d1` is the first degree of freedom, `d2` is
    /// the second degree of freedom, and `I` is the regularized incomplete
    /// beta function
    fn cdf(&self, x: f64) -> f64 {
        if self.freedom_1 == f64::INFINITY || self.freedom_2 == f64::INFINITY || x.is_infinite() {
            f64::NAN
        } else {
            beta::beta_reg(self.freedom_1 / 2.0,
                           self.freedom_2 / 2.0,
                           self.freedom_1 * x / (self.freedom_1 * x + self.freedom_2))
        }
    }
}

impl Min<f64> for FisherSnedecor {
    /// Returns the minimum value in the domain of the
    /// fisher-snedecor distribution representable by a double precision
    /// float
    ///
    /// # Formula
    ///
    /// ```ignore
    /// 0
    /// ```
    fn min(&self) -> f64 {
        0.0
    }
}

impl Max<f64> for FisherSnedecor {
    /// Returns the maximum value in the domain of the
    /// fisher-snedecor distribution representable by a double precision
    /// float
    ///
    /// # Formula
    ///
    /// ```ignore
    /// INF
    /// ```
    fn max(&self) -> f64 {
        f64::INFINITY
    }
}

impl Mean<f64> for FisherSnedecor {
    /// Returns the mean of the fisher-snedecor distribution
    ///
    /// # Panics
    ///
    /// If `freedom_2 <= 2.0`
    ///
    /// # Remarks
    ///
    /// Returns `NaN` if `freedom_2` is `INF`
    ///
    /// # Formula
    ///
    /// ```ignore
    /// d2 / (d2 - 2)
    /// ```
    ///
    /// where `d2` is the second degree of freedom
    fn mean(&self) -> f64 {
        assert!(self.freedom_2 > 2.0, StatsError::ArgGt("freedom_2", 2.0));
        self.freedom_2 / (self.freedom_2 - 2.0)
    }
}

impl Variance<f64> for FisherSnedecor {
    /// Returns the variance of the fisher-snedecor distribution
    ///
    /// # Panics
    ///
    /// If `freedom_2 <= 4.0`
    ///
    /// # Remarks
    ///
    /// Returns `NaN` if `freedom_1` or `freedom_2` is `INF`
    ///
    /// # Formula
    ///
    /// ```ignore
    /// (2 * d2^2 * (d1 + d2 - 2)) / (d1 * (d2 - 2)^2 * (d2 - 4))
    /// ```
    ///
    /// where `d1` is the first degree of freedom and `d2` is
    /// the second degree of freedom
    fn variance(&self) -> f64 {
        assert!(self.freedom_2 > 4.0, StatsError::ArgGt("freedom_2", 4.0));
        (2.0 * self.freedom_2 * self.freedom_2 * (self.freedom_1 + self.freedom_2 - 2.0)) /
        (self.freedom_1 * (self.freedom_2 - 2.0) * (self.freedom_2 - 2.0) * (self.freedom_2 - 4.0))
    }

    /// Returns the standard deviation of the fisher-snedecor distribution
    ///
    /// # Panics
    ///
    /// If `freedom_2 <= 4.0`
    ///
    /// # Remarks
    ///
    /// Returns `NaN` if `freedom_1` or `freedom_2` is `INF`
    ///
    /// # Formula
    ///
    /// ```ignore
    /// sqrt((2 * d2^2 * (d1 + d2 - 2)) / (d1 * (d2 - 2)^2 * (d2 - 4)))
    /// ```
    ///
    /// where `d1` is the first degree of freedom and `d2` is
    /// the second degree of freedom
    fn std_dev(&self) -> f64 {
        self.variance().sqrt()
    }
}

impl Skewness<f64> for FisherSnedecor {
    /// Returns the skewness of the fisher-snedecor distribution
    ///
    /// # Panics
    ///
    /// If `freedom_2 <= 6.0`
    ///
    /// # Remarks
    ///
    /// Returns `NaN` if `freedom_1` or `freedom_2` is `INF`
    ///
    /// # Formula
    ///
    /// ```ignore
    /// ((2d1 + d2 - 2) * sqrt(8 * (d2 - 4))) / ((d2 - 6) * sqrt(d1 * (d1 + d2 - 2)))
    /// ```
    ///
    /// where `d1` is the first degree of freedom and `d2` is
    /// the second degree of freedom
    fn skewness(&self) -> f64 {
        assert!(self.freedom_2 > 6.0, StatsError::ArgGt("freedom_2", 6.0));
        ((2.0 * self.freedom_1 + self.freedom_2 - 2.0) * (8.0 * (self.freedom_2 - 4.0)).sqrt()) /
        ((self.freedom_2 - 6.0) * (self.freedom_1 * (self.freedom_1 + self.freedom_2 - 2.0)).sqrt())
    }
}

impl Mode<f64> for FisherSnedecor {
    /// Returns the mode for the fisher-snedecor distribution
    ///
    /// # Panics
    ///
    /// If `freedom_1 <= 2.0`
    ///
    /// # Remarks
    ///
    /// Returns `NaN` if `freedom_1` or `freedom_2` is `INF`
    ///
    /// # Formula
    ///
    /// ```ignore
    /// ((d1 - 2) / d1) * (d2 / (d2 + 2))
    /// ```
    ///
    /// where `d1` is the first degree of freedom and `d2` is
    /// the second degree of freedom
    fn mode(&self) -> f64 {
        assert!(self.freedom_1 > 2.0, StatsError::ArgGt("freedom_1", 2.0));
        (self.freedom_2 * (self.freedom_1 - 2.0)) / (self.freedom_1 * (self.freedom_2 + 2.0))
    }
}

impl Continuous<f64, f64> for FisherSnedecor {
    /// Calculates the probability density function for the fisher-snedecor distribution
    /// at `x`
    ///
    /// # Remarks
    ///
    /// Returns `NaN` if `freedom_1`, `freedom_2` is `INF`, or `x` is `+INF` or `-INF`
    ///
    /// # Formula
    ///
    /// ```ignore
    /// sqrt(((d1 * x) ^ d1 * d2 ^ d2) / (d1 * x + d2) ^ (d1 + d2)) / (x * β(d1 / 2, d2 / 2))
    /// ```
    ///
    /// where `d1` is the first degree of freedom, `d2` is
    /// the second degree of freedom, and `β` is the beta function
    fn pdf(&self, x: f64) -> f64 {
        ((self.freedom_1 * x).powf(self.freedom_1) * self.freedom_2.powf(self.freedom_2) /
         (self.freedom_1 * x + self.freedom_2).powf(self.freedom_1 + self.freedom_2))
            .sqrt() / (x * beta::beta(self.freedom_1 / 2.0, self.freedom_2 / 2.0))
    }

    /// Calculates the log probability density function for the fisher-snedecor distribution
    /// at `x`
    ///
    /// # Remarks
    ///
    /// Returns `NaN` if `freedom_1`, `freedom_2` is `INF`, or `x` is `+INF` or `-INF`
    ///
    /// # Formula
    ///
    /// ```ignore
    /// ln(sqrt(((d1 * x) ^ d1 * d2 ^ d2) / (d1 * x + d2) ^ (d1 + d2)) / (x * β(d1 / 2, d2 / 2)))
    /// ```
    ///
    /// where `d1` is the first degree of freedom, `d2` is
    /// the second degree of freedom, and `β` is the beta function
    fn ln_pdf(&self, x: f64) -> f64 {
        self.pdf(x).ln()
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
#[cfg(test)]
mod test {
    use std::f64;
    use statistics::*;
    use distribution::{Univariate, Continuous, FisherSnedecor};

    fn try_create(freedom_1: f64, freedom_2: f64) -> FisherSnedecor {
        let n = FisherSnedecor::new(freedom_1, freedom_2);
        assert!(n.is_ok());
        n.unwrap()
    }

    fn create_case(freedom_1: f64, freedom_2: f64) {
        let n = try_create(freedom_1, freedom_2);
        assert_eq!(freedom_1, n.freedom_1());
        assert_eq!(freedom_2, n.freedom_2());
    }

    fn bad_create_case(freedom_1: f64, freedom_2: f64) {
        let n = FisherSnedecor::new(freedom_1, freedom_2);
        assert!(n.is_err());
    }

     fn get_value<F>(freedom_1: f64, freedom_2: f64, eval: F) -> f64
        where F: Fn(FisherSnedecor) -> f64
    {
        let n = try_create(freedom_1, freedom_2);
        eval(n)
    }

    fn test_case<F>(freedom_1: f64, freedom_2: f64, expected: f64, eval: F)
        where F: Fn(FisherSnedecor) -> f64
    {
        let x = get_value(freedom_1, freedom_2, eval);
        assert_eq!(expected, x);
    }

    fn test_almost<F>(freedom_1: f64, freedom_2: f64, expected: f64, acc: f64, eval: F)
        where F: Fn(FisherSnedecor) -> f64
    {
        let x = get_value(freedom_1, freedom_2, eval);
        assert_almost_eq!(expected, x, acc);
    }

    fn test_is_nan<F>(freedom_1: f64, freedom_2: f64, eval: F)
        where F: Fn(FisherSnedecor) -> f64
    {
        assert!(get_value(freedom_1, freedom_2, eval).is_nan())
    }

    #[test]
    fn test_create() {
        create_case(0.1, 0.1);
        create_case(1.0, 0.1);
        create_case(10.0, 0.1);
        create_case(f64::INFINITY, 0.1);
        create_case(0.1, 1.0);
        create_case(1.0, 1.0);
        create_case(10.0, 1.0);
        create_case(f64::INFINITY, 10.0);
        create_case(0.1, f64::INFINITY);
        create_case(1.0, f64::INFINITY);
        create_case(10.0, f64::INFINITY);
        create_case(f64::INFINITY, f64::INFINITY);
    }

    #[test]
    fn test_bad_create() {
        bad_create_case(f64::NAN, f64::NAN);
        bad_create_case(0.0, f64::NAN);
        bad_create_case(-1.0, f64::NAN);
        bad_create_case(-10.0, f64::NAN);
        bad_create_case(f64::NAN, 0.0);
        bad_create_case(0.0, 0.0);
        bad_create_case(-1.0, 0.0);
        bad_create_case(-10.0, 0.0);
        bad_create_case(f64::NAN, -1.0);
        bad_create_case(0.0, -1.0);
        bad_create_case(-1.0, -1.0);
        bad_create_case(-10.0, -1.0);
        bad_create_case(f64::NAN, -10.0);
        bad_create_case(0.0, -10.0);
        bad_create_case(-1.0, -10.0);
        bad_create_case(-10.0, -10.0);
    }

    #[test]
    #[should_panic]
    fn test_mean_with_low_d2() {
        get_value(0.1, 0.1, |x| x.mean());
    }

    #[test]
    fn test_mean() {
        test_case(0.1, 10.0, 1.25, |x| x.mean());
        test_case(1.0, 10.0, 1.25, |x| x.mean());
        test_case(10.0, 10.0, 1.25, |x| x.mean());
        test_is_nan(0.1, f64::INFINITY, |x| x.mean());
        test_is_nan(1.0, f64::INFINITY, |x| x.mean());
        test_is_nan(10.0, f64::INFINITY, |x| x.mean());
        test_is_nan(f64::INFINITY, f64::INFINITY, |x| x.mode());
    }

    #[test]
    #[should_panic]
    fn test_variance_with_low_d2() {
        get_value(0.1, 0.1, |x| x.variance());
    }

    #[test]
    fn test_variance() {
        test_almost(0.1, 10.0, 42.1875, 1e-14, |x| x.variance());
        test_case(1.0, 10.0, 4.6875, |x| x.variance());
        test_case(10.0, 10.0, 0.9375, |x| x.variance());
        test_is_nan(0.1, f64::INFINITY, |x| x.variance());
        test_is_nan(1.0, f64::INFINITY, |x| x.variance());
        test_is_nan(10.0, f64::INFINITY, |x| x.variance());
        test_is_nan(f64::INFINITY, 10.0, |x| x.variance());
        test_is_nan(f64::INFINITY, f64::INFINITY, |x| x.mode());
    }

    #[test]
    #[should_panic]
    fn test_std_dev_with_low_d2() {
        get_value(0.1, 0.1, |x| x.std_dev());
    }

    #[test]
    fn test_std_dev() {
        test_almost(0.1, 10.0, 42.1875f64.sqrt(), 1e-14, |x| x.std_dev());
        test_case(1.0, 10.0, 4.6875f64.sqrt(), |x| x.std_dev());
        test_case(10.0, 10.0, 0.9375f64.sqrt(), |x| x.std_dev());
        test_is_nan(0.1, f64::INFINITY, |x| x.std_dev());
        test_is_nan(1.0, f64::INFINITY, |x| x.std_dev());
        test_is_nan(10.0, f64::INFINITY, |x| x.std_dev());
        test_is_nan(f64::INFINITY, 10.0, |x| x.std_dev());
        test_is_nan(f64::INFINITY, f64::INFINITY, |x| x.mode());
    }

    #[test]
    #[should_panic]
    fn test_skewness_with_low_d2() {
        get_value(0.1, 0.1, |x| x.skewness());
    }

    #[test]
    fn test_skewness() {
        test_almost(0.1, 10.0, 15.78090735784977089658, 1e-14, |x| x.skewness());
        test_case(1.0, 10.0, 5.773502691896257645091, |x| x.skewness());
        test_case(10.0, 10.0, 3.614784456460255759501, |x| x.skewness());
        test_is_nan(0.1, f64::INFINITY, |x| x.skewness());
        test_is_nan(1.0, f64::INFINITY, |x| x.skewness());
        test_is_nan(10.0, f64::INFINITY, |x| x.skewness());
        test_is_nan(f64::INFINITY, 10.0, |x| x.skewness());
        test_is_nan(f64::INFINITY, f64::INFINITY, |x| x.mode());
    }

    #[test]
    #[should_panic]
    fn test_mode_with_low_d1() {
        get_value(0.1, 0.1, |x| x.mode());
    }

    #[test]
    fn test_mode() {
        test_case(10.0, 0.1, 0.0380952380952380952381, |x| x.mode());
        test_case(10.0, 1.0, 4.0 / 15.0, |x| x.mode());
        test_case(10.0, 10.0, 2.0 / 3.0, |x| x.mode());
        test_is_nan(10.0, f64::INFINITY, |x| x.mode());
        test_is_nan(f64::INFINITY, 0.1, |x| x.mode());
        test_is_nan(f64::INFINITY, 1.0, |x| x.mode());
        test_is_nan(f64::INFINITY, 10.0, |x| x.mode());
        test_is_nan(f64::INFINITY, f64::INFINITY, |x| x.mode());
    }

    #[test]
    fn test_min_max() {
        test_case(1.0, 1.0, 0.0, |x| x.min());
        test_case(1.0, 1.0, f64::INFINITY, |x| x.max());
    }

    #[test]
    fn test_pdf() {
        test_almost(0.1, 0.1, 0.0234154207226588982471, 1e-16, |x| x.pdf(1.0));
        test_almost(1.0, 0.1, 0.0396064560910663979961, 1e-16, |x| x.pdf(1.0));
        test_almost(10.0, 0.1, 0.0418440630400545297349, 1e-14, |x| x.pdf(1.0));
        test_is_nan(f64::INFINITY, 0.1, |x| x.pdf(1.0));
        test_almost(0.1, 1.0, 0.0396064560910663979961, 1e-16, |x| x.pdf(1.0));
        test_almost(1.0, 1.0, 0.1591549430918953357689, 1e-16, |x| x.pdf(1.0));
        test_almost(10.0, 1.0, 0.230361989229138647108, 1e-16, |x| x.pdf(1.0));
        test_is_nan(f64::INFINITY, 1.0, |x| x.pdf(1.0));
        test_is_nan(0.1, f64::INFINITY, |x| x.pdf(1.0));
        test_is_nan(1.0, f64::INFINITY, |x| x.pdf(1.0));
        test_is_nan(10.0, f64::INFINITY, |x| x.pdf(1.0));
        test_is_nan(f64::INFINITY, f64::INFINITY, |x| x.pdf(1.0));
        test_almost(0.1, 0.1, 0.00221546909694001013517, 1e-18, |x| x.pdf(10.0));
        test_almost(1.0, 0.1, 0.00369960370387922619592, 1e-17, |x| x.pdf(10.0));
        test_almost(10.0, 0.1, 0.00390179721174142927402, 1e-15, |x| x.pdf(10.0));
        test_is_nan(f64::INFINITY, 0.1, |x| x.pdf(10.0));
        test_almost(0.1, 1.0, 0.00319864073359931548273, 1e-17, |x| x.pdf(10.0));
        test_almost(1.0, 1.0, 0.009150765837179460915678, 1e-17, |x| x.pdf(10.0));
        test_almost(10.0, 1.0, 0.0116493859171442148446, 1e-17, |x| x.pdf(10.0));
        test_is_nan(f64::INFINITY, 1.0, |x| x.pdf(10.0));
        test_almost(0.1, 10.0, 0.00305087016058573989694, 1e-15, |x| x.pdf(10.0));
        test_almost(1.0, 10.0, 0.00271897749113479577864, 1e-17, |x| x.pdf(10.0));
        test_almost(10.0, 10.0, 2.4289227234060500084E-4, 1e-18, |x| x.pdf(10.0));
        test_is_nan(f64::INFINITY, 10.0, |x| x.pdf(10.0));
        test_is_nan(f64::INFINITY, 1.0, |x| x.pdf(10.0));
        test_is_nan(0.1, f64::INFINITY, |x| x.pdf(10.0));
        test_is_nan(1.0, f64::INFINITY, |x| x.pdf(10.0));
        test_is_nan(10.0, f64::INFINITY, |x| x.pdf(10.0));
        test_is_nan(f64::INFINITY, f64::INFINITY, |x| x.pdf(10.0));
        test_is_nan(0.1, 0.1, |x| x.pdf(f64::INFINITY));
        test_is_nan(0.1, 0.1, |x| x.pdf(f64::NEG_INFINITY));
        test_is_nan(f64::INFINITY, 0.1, |x| x.pdf(f64::INFINITY));
        test_is_nan(0.1, f64::INFINITY, |x| x.pdf(f64::INFINITY));
        test_is_nan(f64::INFINITY, f64::INFINITY, |x| x.pdf(f64::INFINITY));
        test_is_nan(f64::INFINITY, 0.1, |x| x.pdf(f64::NEG_INFINITY));
        test_is_nan(0.1, f64::INFINITY, |x| x.pdf(f64::NEG_INFINITY));
        test_is_nan(f64::INFINITY, f64::INFINITY, |x| x.pdf(f64::NEG_INFINITY));
    }

    #[test]
    fn test_ln_pdf() {
        test_almost(0.1, 0.1, 0.0234154207226588982471f64.ln(), 1e-15, |x| x.ln_pdf(1.0));
        test_almost(1.0, 0.1, 0.0396064560910663979961f64.ln(), 1e-15, |x| x.ln_pdf(1.0));
        test_almost(10.0, 0.1, 0.0418440630400545297349f64.ln(), 1e-13, |x| x.ln_pdf(1.0));
        test_is_nan(f64::INFINITY, 0.1, |x| x.ln_pdf(1.0));
        test_almost(0.1, 1.0, 0.0396064560910663979961f64.ln(), 1e-15, |x| x.ln_pdf(1.0));
        test_almost(1.0, 1.0, 0.1591549430918953357689f64.ln(), 1e-15, |x| x.ln_pdf(1.0));
        test_almost(10.0, 1.0, 0.230361989229138647108f64.ln(), 1e-15, |x| x.ln_pdf(1.0));
        test_is_nan(f64::INFINITY, 1.0, |x| x.ln_pdf(1.0));
        test_is_nan(0.1, f64::INFINITY, |x| x.ln_pdf(1.0));
        test_is_nan(1.0, f64::INFINITY, |x| x.ln_pdf(1.0));
        test_is_nan(10.0, f64::INFINITY, |x| x.ln_pdf(1.0));
        test_is_nan(f64::INFINITY, f64::INFINITY, |x| x.ln_pdf(1.0));
        test_case(0.1, 0.1, 0.00221546909694001013517f64.ln(), |x| x.ln_pdf(10.0));
        test_almost(1.0, 0.1, 0.00369960370387922619592f64.ln(), 1e-15, |x| x.ln_pdf(10.0));
        test_almost(10.0, 0.1, 0.00390179721174142927402f64.ln(), 1e-13, |x| x.ln_pdf(10.0));
        test_is_nan(f64::INFINITY, 0.1, |x| x.ln_pdf(10.0));
        test_almost(0.1, 1.0, 0.00319864073359931548273f64.ln(), 1e-15, |x| x.ln_pdf(10.0));
        test_almost(1.0, 1.0, 0.009150765837179460915678f64.ln(), 1e-15, |x| x.ln_pdf(10.0));
        test_case(10.0, 1.0, 0.0116493859171442148446f64.ln(), |x| x.ln_pdf(10.0));
        test_is_nan(f64::INFINITY, 1.0, |x| x.ln_pdf(10.0));
        test_almost(0.1, 10.0, 0.00305087016058573989694f64.ln(), 1e-13, |x| x.ln_pdf(10.0));
        test_case(1.0, 10.0, 0.00271897749113479577864f64.ln(), |x| x.ln_pdf(10.0));
        test_almost(10.0, 10.0, 2.4289227234060500084E-4f64.ln(), 1e-14, |x| x.ln_pdf(10.0));
        test_is_nan(f64::INFINITY, 10.0, |x| x.ln_pdf(10.0));
        test_is_nan(0.1, f64::INFINITY, |x| x.ln_pdf(10.0));
        test_is_nan(1.0, f64::INFINITY, |x| x.ln_pdf(10.0));
        test_is_nan(10.0, f64::INFINITY, |x| x.ln_pdf(10.0));
        test_is_nan(f64::INFINITY, f64::INFINITY, |x| x.ln_pdf(10.0));
        test_is_nan(0.1, 0.1, |x| x.ln_pdf(f64::INFINITY));
        test_is_nan(0.1, 0.1, |x| x.ln_pdf(f64::NEG_INFINITY));
        test_is_nan(f64::INFINITY, 0.1, |x| x.ln_pdf(f64::INFINITY));
        test_is_nan(0.1, f64::INFINITY, |x| x.ln_pdf(f64::INFINITY));
        test_is_nan(f64::INFINITY, f64::INFINITY, |x| x.ln_pdf(f64::INFINITY));
        test_is_nan(f64::INFINITY, 0.1, |x| x.ln_pdf(f64::NEG_INFINITY));
        test_is_nan(0.1, f64::INFINITY, |x| x.ln_pdf(f64::NEG_INFINITY));
        test_is_nan(f64::INFINITY, f64::INFINITY, |x| x.ln_pdf(f64::NEG_INFINITY));
    }

    #[test]
    fn test_cdf() {
        test_almost(0.1, 0.1, 0.44712986033425140335, 1e-15, |x| x.cdf(0.1));
        test_almost(1.0, 0.1, 0.08156522095104674015, 1e-15, |x| x.cdf(0.1));
        test_almost(10.0, 0.1, 0.033184005716276536322, 1e-13, |x| x.cdf(0.1));
        test_is_nan(f64::INFINITY, 0.1, |x| x.cdf(0.1));
        test_almost(0.1, 1.0, 0.74378710917986379989, 1e-15, |x| x.cdf(0.1));
        test_almost(1.0, 1.0, 0.1949822290421366451595, 1e-16, |x| x.cdf(0.1));
        test_almost(10.0, 1.0, 0.0101195597354337146205, 1e-17, |x| x.cdf(0.1));
        test_is_nan(f64::INFINITY, 1.0, |x| x.cdf(0.1));
        test_is_nan(0.1, f64::INFINITY, |x| x.cdf(0.1));
        test_is_nan(1.0, f64::INFINITY, |x| x.cdf(0.1));
        test_is_nan(10.0, f64::INFINITY, |x| x.cdf(0.1));
        test_is_nan(f64::INFINITY, f64::INFINITY, |x| x.cdf(0.1));
        test_almost(0.1, 0.1, 0.5, 1e-15, |x| x.cdf(1.0));
        test_almost(1.0, 0.1, 0.16734351500944271141, 1e-14, |x| x.cdf(1.0));
        test_almost(10.0, 0.1, 0.12207560664741704938, 1e-13, |x| x.cdf(1.0));
        test_is_nan(f64::INFINITY, 0.1, |x| x.cdf(1.0));
        test_almost(0.1, 1.0, 0.83265648499055728859, 1e-15, |x| x.cdf(1.0));
        test_almost(1.0, 1.0, 0.5, 1e-15, |x| x.cdf(1.0));
        test_almost(10.0, 1.0, 0.340893132302059872675, 1e-15, |x| x.cdf(1.0));
        test_is_nan(f64::INFINITY, 1.0, |x| x.cdf(1.0));
        test_is_nan(0.1, f64::INFINITY, |x| x.cdf(1.0));
        test_is_nan(1.0, f64::INFINITY, |x| x.cdf(1.0));
        test_is_nan(10.0, f64::INFINITY, |x| x.cdf(1.0));
        test_is_nan(f64::INFINITY, f64::INFINITY, |x| x.cdf(1.0));
        test_is_nan(0.1, 0.1, |x| x.cdf(f64::INFINITY));
        test_is_nan(0.1, 0.1, |x| x.cdf(f64::NEG_INFINITY));
        test_is_nan(f64::INFINITY, 0.1, |x| x.cdf(f64::INFINITY));
        test_is_nan(0.1, f64::INFINITY, |x| x.cdf(f64::INFINITY));
        test_is_nan(f64::INFINITY, f64::INFINITY, |x| x.cdf(f64::INFINITY));
        test_is_nan(f64::INFINITY, 0.1, |x| x.cdf(f64::NEG_INFINITY));
        test_is_nan(0.1, f64::INFINITY, |x| x.cdf(f64::NEG_INFINITY));
        test_is_nan(f64::INFINITY, f64::INFINITY, |x| x.cdf(f64::NEG_INFINITY));
    }
}