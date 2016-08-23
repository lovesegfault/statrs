use std::f64;
use rand::Rng;
use rand::distributions::{Sample, IndependentSample};
use result::Result;
use super::*;

/// Implements the [Chi-squared](https://en.wikipedia.org/wiki/Chi-squared_distribution)
/// distribution which is a special case of the [Gamma](https://en.wikipedia.org/wiki/Gamma_distribution) distribution
/// (referenced [Here](./struct.Gamma.html))
///
/// # Examples
///
/// ```
/// use statrs::distribution::{ChiSquared, Mean, Continuous};
/// use statrs::prec;
///
/// let n = ChiSquared::new(3.0).unwrap();
/// assert_eq!(n.mean(), 3.0);
/// assert!(prec::almost_eq(n.pdf(4.0), 0.107981933026376103901, 1e-15));
/// ```
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ChiSquared {
    freedom: f64,
    g: Gamma,
}

impl ChiSquared {
    /// Constructs a new chi-squared distribution with `freedom`
    /// degrees of freedom. This is equivalent to a Gamma distribution
    /// with a shape of `freedom / 2.0` and a rate of `0.5`.
    ///
    /// # Errors
    ///
    /// Returns an error if `freedom` is `NaN` or less than
    /// or equal to `0.0`
    ///
    /// # Examples
    ///
    /// ```
    /// use statrs::distribution::ChiSquared;
    ///
    /// let mut result = ChiSquared::new(3.0);
    /// assert!(result.is_ok());
    ///
    /// result = ChiSquared::new(0.0);
    /// assert!(result.is_err());
    /// ```
    pub fn new(freedom: f64) -> Result<ChiSquared> {
        Gamma::new(freedom / 2.0, 0.5).map(|g| {
            ChiSquared {
                freedom: freedom,
                g: g,
            }
        })
    }

    /// Returns the degrees of freedom of the chi-squared
    /// distribution
    ///
    /// # Examples
    ///
    /// ```
    /// use statrs::distribution::ChiSquared;
    ///
    /// let n = ChiSquared::new(3.0).unwrap();
    /// assert_eq!(n.freedom(), 3.0);
    /// ```
    pub fn freedom(&self) -> f64 {
        self.freedom
    }

    /// Returns the shape of the underlying Gamma distribution
    ///
    /// # Examples
    ///
    /// ```
    /// use statrs::distribution::ChiSquared;
    ///
    /// let n = ChiSquared::new(3.0).unwrap();
    /// assert_eq!(n.shape(), 3.0 / 2.0);
    /// ```
    pub fn shape(&self) -> f64 {
        self.g.shape()
    }

    /// Returns the rate of the underlying Gamma distribution
    ///
    /// # Examples
    ///
    /// ```
    /// use statrs::distribution::ChiSquared;
    ///
    /// let n = ChiSquared::new(3.0).unwrap();
    /// assert_eq!(n.rate(), 0.5);
    /// ```
    pub fn rate(&self) -> f64 {
        self.g.rate()
    }
}

impl Sample<f64> for ChiSquared {
    /// Generate a random sample from a chi-squared
    /// distribution using `r` as the source of randomness.
    /// Refer [here](#method.sample-1) for implementation details
    fn sample<R: Rng>(&mut self, r: &mut R) -> f64 {
        super::Distribution::sample(self, r)
    }
}

impl IndependentSample<f64> for ChiSquared {
    /// Generate a random independent sample from a Chi
    /// distribution using `r` as the source of randomness.
    /// Refer [here](#method.sample-1) for implementation details
    fn ind_sample<R: Rng>(&self, r: &mut R) -> f64 {
        super::Distribution::sample(self, r)
    }
}

impl Distribution<f64> for ChiSquared {
    /// Generate a random sample from the chi-squared distribution
    /// using `r` as the source of randomness
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate rand;
    /// # extern crate statrs;
    /// use rand::StdRng;
    /// use statrs::distribution::{ChiSquared, Distribution};
    ///
    /// # fn main() {
    /// let mut r = rand::StdRng::new().unwrap();
    /// let n = ChiSquared::new(3.0).unwrap();
    /// print!("{}", n.sample::<StdRng>(&mut r));
    /// # }
    /// ```
    fn sample<R: Rng>(&self, r: &mut R) -> f64 {
        self.g.sample(r)
    }
}

impl Univariate<f64, f64> for ChiSquared {
    /// Calculates the cumulative distribution function for the
    /// chi-squared distribution at `x`
    ///
    /// # Panics
    ///
    /// If `x < 0.0`
    ///
    /// # Formula
    ///
    /// ```ignore
    /// (1 / Γ(k / 2)) * γ(k / 2, x / 2)
    /// ```
    ///
    /// where `k` is the degrees of freedom, `Γ` is the gamma function,
    /// and `γ` is the lower incomplete gamma function
    fn cdf(&self, x: f64) -> f64 {
        self.g.cdf(x)
    }

    /// Returns the minimum value in the domain of the
    /// chi-squared distribution representable by a double precision
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

    /// Returns the maximum value in the domain of the
    /// chi-squared distribution representable by a double precision
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

impl Mean<f64, f64> for ChiSquared {
    /// Returns the mean of the chi-squared distribution
    ///
    /// # Formula
    ///
    /// ```ignore
    /// k
    /// ```
    ///
    /// where `k` is the degrees of freedom
    fn mean(&self) -> f64 {
        self.g.mean()
    }
}

impl Variance<f64, f64> for ChiSquared {
    /// Returns the variance of the chi-squared distribution
    ///
    /// # Formula
    ///
    /// ```ignore
    /// 2k
    /// ```
    ///
    /// where `k` is the degrees of freedom
    fn variance(&self) -> f64 {
        self.g.variance()
    }

    /// Returns the standard deviation of the chi-squared distribution
    ///
    /// # Formula
    ///
    /// ```ignore
    /// sqrt(2k)
    /// ```
    ///
    /// where `k` is the degrees of freedom
    fn std_dev(&self) -> f64 {
        self.g.std_dev()
    }
}

impl Entropy<f64> for ChiSquared {
    /// Returns the entropy of the chi-squared distribution
    ///
    /// # Formula
    ///
    /// ```ignore
    /// (k / 2) + ln(2 * Γ(k / 2)) + (1 - (k / 2)) * ψ(k / 2)
    /// ```
    ///
    /// where `k` is the degrees of freedom, `Γ` is the gamma function,
    /// and `ψ` is the digamma function
    fn entropy(&self) -> f64 {
        self.g.entropy()
    }
}

impl Skewness<f64, f64> for ChiSquared {
    /// Returns the skewness of the chi-squared distribution
    ///
    /// # Formula
    ///
    /// ```ignore
    /// sqrt(8 / k)
    /// ```
    ///
    /// where `k` is the degrees of freedom
    fn skewness(&self) -> f64 {
        self.g.skewness()
    }
}

impl Median<f64> for ChiSquared {
    /// Returns the median  of the chi-squared distribution
    ///
    /// # Formula
    ///
    /// ```ignore
    /// k * (1 - (2 / 9k))^3
    /// ```
    fn median(&self) -> f64 {
        if self.freedom < 1.0 {
            // if k is small, calculate using expansion of formula
            self.freedom - 2.0 / 3.0 + 12.0 / (81.0 * self.freedom) -
            8.0 / (729.0 * self.freedom * self.freedom)
        } else {
            // if k is large enough, median heads toward k - 2/3
            self.freedom - 2.0 / 3.0
        }
    }
}

impl Mode<f64, f64> for ChiSquared {
    /// Returns the mode of the chi-squared distribution
    ///
    /// # Formula
    ///
    /// ```ignore
    /// k - 2
    /// ```
    ///
    /// where `k` is the degrees of freedom
    fn mode(&self) -> f64 {
        self.g.mode()
    }
}

impl Continuous<f64, f64> for ChiSquared {
    /// Calculates the probability density function for the chi-squared
    /// distribution at `x`
    ///
    /// # Panics
    ///
    /// If `x < 0.0`
    ///
    /// # Formula
    ///
    /// ```ignore
    /// 1 / (2^(k / 2) * Γ(k / 2)) * x^((k / 2) - 1) * e^(-x / 2)
    /// ```
    ///
    /// where `k` is the degrees of freedom and `Γ` is the gamma function
    fn pdf(&self, x: f64) -> f64 {
        self.g.pdf(x)
    }

    /// Calculates the log probability density function for the chi-squared
    /// distribution at `x`
    ///
    /// # Panics
    ///
    /// If `x < 0.0`
    ///
    /// # Formula
    ///
    /// ```ignore
    /// ln(1 / (2^(k / 2) * Γ(k / 2)) * x^((k / 2) - 1) * e^(-x / 2))
    /// ```
    fn ln_pdf(&self, x: f64) -> f64 {
        self.g.ln_pdf(x)
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
#[cfg(test)]
mod test {
    use std::f64;
    use distribution::*;

    fn try_create(freedom: f64) -> ChiSquared {
        let n = ChiSquared::new(freedom);
        assert!(n.is_ok());
        n.unwrap()
    }

    fn test_case<F>(freedom: f64, expected: f64, eval: F)
        where F: Fn(ChiSquared) -> f64
    {
        let n = try_create(freedom);
        let x = eval(n);
        assert_eq!(expected, x);
    }

    fn test_almost<F>(freedom: f64, expected: f64, acc: f64, eval: F)
        where F: Fn(ChiSquared) -> f64
    {
        let n = try_create(freedom);
        let x = eval(n);
        assert_almost_eq!(expected, x, acc);
    }

    #[test]
    fn test_median() {
        test_almost(0.5, 0.0857338820301783264746, 1e-16, |x| x.median());
        test_case(1.0, 1.0 - 2.0 / 3.0, |x| x.median());
        test_case(2.0, 2.0 - 2.0 / 3.0, |x| x.median());
        test_case(2.5, 2.5 - 2.0 / 3.0, |x| x.median());
        test_case(3.0, 3.0 - 2.0 / 3.0, |x| x.median());
    }
}
