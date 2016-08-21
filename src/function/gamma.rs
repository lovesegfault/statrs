use std::f64;
use consts;
use error::StatsError;
use prec;

/// Auxiliary variable when evaluating the gamma_ln function
const GAMMA_R: f64 = 10.900511;

/// Polynomial coefficients for approximating the gamma_ln function
const GAMMA_DK: &'static [f64] = &[2.48574089138753565546e-5,
                                   1.05142378581721974210,
                                   -3.45687097222016235469,
                                   4.51227709466894823700,
                                   -2.98285225323576655721,
                                   1.05639711577126713077,
                                   -1.95428773191645869583e-1,
                                   1.70970543404441224307e-2,
                                   -5.71926117404305781283e-4,
                                   4.63399473359905636708e-6,
                                   -2.71994908488607703910e-9];

/// Computes the logarithm of the gamma function
/// with an accuracy of 16 floating point digits.
/// The implementation is derived from 
/// "An Analysis of the Lanczos Gamma Approximation",
/// Glendon Ralph Pugh, 2004 p. 116
pub fn ln_gamma(x: f64) -> f64 {
    if x < 0.5 {
        let s = GAMMA_DK.iter()
                        .enumerate()
                        .skip(1)
                        .fold(GAMMA_DK[0], |s, t| s + t.1 / (t.0 as f64 - x));

        consts::LN_PI - (f64::consts::PI * x).sin().ln() - s.ln() - consts::LN_2_SQRT_E_OVER_PI -
        (0.5 - x) * ((0.5 - x + GAMMA_R) / f64::consts::E).ln()
    } else {
        let s = GAMMA_DK.iter()
                        .enumerate()
                        .skip(1)
                        .fold(GAMMA_DK[0], |s, t| s + t.1 / (x + t.0 as f64 - 1.0));

        s.ln() + consts::LN_2_SQRT_E_OVER_PI +
        (x - 0.5) * ((x - 0.5 + GAMMA_R) / f64::consts::E).ln()
    }
}

/// Computes the gamma function with an accuracy
/// of 16 floating point digits. The implementation
/// is derived from "An Analysis of the Lanczos Gamma Approximation",
/// Glendon Ralph Pugh, 2004 p. 116
pub fn gamma(x: f64) -> f64 {
    if x < 0.5 {
        let s = GAMMA_DK.iter()
                        .enumerate()
                        .skip(1)
                        .fold(GAMMA_DK[0], |s, t| s + t.1 / (t.0 as f64 - x));

        f64::consts::PI /
        ((f64::consts::PI * x).sin() * s * consts::TWO_SQRT_E_OVER_PI *
         ((0.5 - x + GAMMA_R) / f64::consts::E).powf(0.5 - x))
    } else {
        let s = GAMMA_DK.iter()
                        .enumerate()
                        .skip(1)
                        .fold(GAMMA_DK[0], |s, t| s + t.1 / (x + t.0 as f64 - 1.0));

        s * consts::TWO_SQRT_E_OVER_PI * ((x - 0.5 + GAMMA_R) / f64::consts::E).powf(x - 0.5)
    }
}

/// Computes the upper incomplete gamma function
/// `Gamma(a,x) = int(exp(-t)t^(a-1), t=0..x) for a > 0, x > 0`
/// where `a` is the argument for the gamma function and
/// `x` is the lower intergral limit.
///
/// # Panics 
/// 
/// if `a` or `x` are less than `0.0`
pub fn gamma_ui(a: f64, x: f64) -> f64 {
    gamma_ur(a, x) * gamma(a)
}

/// Computes the lower incomplete gamma function
/// `gamma(a,x) = int(exp(-t)t^(a-1), t=0..x) for a > 0, x > 0`
/// where `a` is the argument for the gamma function and `x`
/// is the upper integral limit.
///
/// 
/// # Panics 
/// 
/// if `a` or `x` are less than `0.0`
pub fn gamma_li(a: f64, x: f64) -> f64 {
    gamma_lr(a, x) * gamma(a)
}

/// Computes the upper incomplete regularized gamma function
/// `Q(a,x) = 1 / Gamma(a) * int(exp(-t)t^(a-1), t=0..x) for a > 0, x > 0`
/// where `a` is the argument for the gamma function and
/// `x` is the lower integral limit.
/// 
/// # Panics
/// 
/// if `a` or `x` are less than `0.0`
pub fn gamma_ur(a: f64, x: f64) -> f64 {
    let eps = 0.000000000000001;
    let big = 4503599627370496.0;
    let big_inv = 2.22044604925031308085e-16;

    if x < 1.0 || x <= a {
        return 1.0 - gamma_lr(a, x);
    }

    let mut ax = a * x.ln() - x - ln_gamma(a);
    if ax < -709.78271289338399 {
        return if a < x {
            0.0
        } else {
            1.0
        };
    }

    ax = ax.exp();
    let mut y = 1.0 - a;
    let mut z = x + y + 1.0;
    let mut c = 0.0;
    let mut pkm2 = 1.0;
    let mut qkm2 = x;
    let mut pkm1 = x + 1.0;
    let mut qkm1 = z * x;
    let mut ans = pkm1 / qkm1;
    loop {
        y += 1.0;
        z += 2.0;
        c += 1.0;
        let yc = y * c;
        let pk = pkm1 * z - pkm2 * yc;
        let qk = qkm1 * z - qkm2 * yc;

        pkm2 = pkm1;
        pkm1 = pk;
        qkm2 = qkm1;
        qkm1 = qk;

        if pk.abs() > big {
            pkm2 *= big_inv;
            pkm1 *= big_inv;
            qkm2 *= big_inv;
            qkm1 *= big_inv;
        }

        if qk != 0.0 {
            let r = pk / qk;
            let t = ((ans - r) / r).abs();
            ans = r;

            if t <= eps {
                break;
            }
        }
    }
    ans * ax
}

/// Computes the lower incomplete regularized gamma function
/// `P(a,x) = 1 / Gamma(a) * int(exp(-t)t^(a-1), t=0..x) for real a > 0, x > 0`
/// where `a` is the argument for the gamma function and `x` is the upper integral limit.
/// 
///
/// # Panics
///
/// if `a` or `x` are less than 0.0
pub fn gamma_lr(a: f64, x: f64) -> f64 {
    assert!(a >= 0.0, format!("{}", StatsError::ArgNotNegative("a")));
    assert!(x >= 0.0, format!("{}", StatsError::ArgNotNegative("x")));

    let eps = 0.000000000000001;
    let big = 4503599627370496.0;
    let big_inv = 2.22044604925031308085e-16;

    if prec::almost_eq(a, 0.0, prec::DEFAULT_F64_ACC) {
        return 1.0;
    }
    if prec::almost_eq(x, 0.0, prec::DEFAULT_F64_ACC) {
        return 0.0;
    }

    let ax = a * x.ln() - x - ln_gamma(a);
    if ax < -709.78271289338399 {
        if a < x {
            return 1.0;
        }
        return 0.0;
    }
    if x <= 1.0 || x <= a {
        let mut r2 = a;
        let mut c2 = 1.0;
        let mut ans2 = 1.0;
        loop {
            r2 += 1.0;
            c2 *= x / r2;
            ans2 += c2;

            if c2 / ans2 <= eps {
                break;
            }
        }
        return ax.exp() * ans2 / a;
    }

    let mut y = 1.0 - a;
    let mut z = x + y + 1.0;
    let mut c = 0;

    let mut p3 = 1.0;
    let mut q3 = x;
    let mut p2 = x + 1.0;
    let mut q2 = z * x;
    let mut ans = p2 / q2;

    loop {
        y += 1.0;
        z += 2.0;
        c += 1;
        let yc = y * c as f64;

        let p = p2 * z - p3 * yc;
        let q = q2 * z - q3 * yc;

        p3 = p2;
        p2 = p;
        q3 = q2;
        q2 = q;

        if p.abs() > big {
            p3 *= big_inv;
            p2 *= big_inv;
            q3 *= big_inv;
            q2 *= big_inv;
        }

        if q != 0.0 {
            let nextans = p / q;
            let error = ((ans - nextans) / nextans).abs();
            ans = nextans;

            if error <= eps {
                break;
            }
        }
    }
    1.0 - ax.exp() * ans
}


/// Computes the Digamma function which is defined as the derivative of
/// the gamma function. The implementation is based on
/// "Algorithm AS 103", Jose Bernardo, Applied Statistics, Volume 25, Number 3
/// 1976, pages 315 - 317
pub fn digamma(x: f64) -> f64 {
    let c = 12.0;
    let d1 = -0.57721566490153286;
    let d2 = 1.6449340668482264365;
    let s = 1e-6;
    let s3 = 1.0 / 12.0;
    let s4 = 1.0 / 120.0;
    let s5 = 1.0 / 252.0;
    let s6 = 1.0 / 240.0;
    let s7 = 1.0 / 132.0;

    if x == f64::NEG_INFINITY || x.is_nan() {
        return f64::NAN;
    }
    if x <= 0.0 && x.floor() == x {
        return f64::NEG_INFINITY;
    }
    if x < 0.0 {
        return digamma(1.0 - x) + f64::consts::PI / (-f64::consts::PI * x).tan();
    }
    if x <= s {
        return d1 - 1.0 / x + d2 * x;
    }

    let mut result = 0.0;
    let mut z = x;
    while z < c {
        result -= 1.0 / z;
        z += 1.0;
    }

    if z >= c {
        let mut r = 1.0 / z;
        result += z.ln() - 0.5 * r;
        r *= r;

        result -= r * (s3 - (r * (s4 - (r * (s5 - (r * (s6 - (r * s7))))))));
    }
    result
}

/// Computes the inverse Digamma function
///
/// # Remarks
///
/// This is the inverse of the logarithm of the gamma function. This function
/// only returns solutions that are positive. The implementation is based on
/// the bisection method
pub fn inv_digamma(p: f64) -> f64 {
    if p.is_nan() {
        return f64::NAN;
    }
    match p {
        f64::NEG_INFINITY => 0.0,
        f64::INFINITY => f64::INFINITY,
        _ => {
            let x = p.exp();
            let d = 1.0;
            for d > 1e-15 {
                let v = p - digamma(x)
                if v.is_sign_positive() {
                    x += d
                } else if v != 0 {
                    x += -d
                }
            }
            x
        }
    }
}
