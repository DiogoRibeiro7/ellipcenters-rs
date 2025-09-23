//! Bracket + Bisection to solve φ(t) = f(x - t g) - f(x) = 0 with φ'(0) = -||g||² < 0.
//! This guarantees a crossing for strongly convex objectives.

use ndarray::Array1;

/// Settings for the scalar root-finding `f(x - t g) = f(x)`.
#[derive(Clone, Copy, Debug)]
pub struct BracketBisectionCfg {
    /// Initial trial step size (seconds). Must be > 0.
    pub t0: f64,
    /// Growth factor while bracketing (> 1.0).
    pub growth: f64,
    /// Maximum number of expansions while bracketing.
    pub max_expand: usize,
    /// Bisection tolerance on t.
    pub tol_t: f64,
    /// Maximum bisection iters.
    pub max_bisect: usize,
}

impl Default for BracketBisectionCfg {
    fn default() -> Self {
        Self {
            t0: 1e-3,
            growth: 2.0,
            max_expand: 40,
            tol_t: 1e-12,
            max_bisect: 80,
        }
    }
}

/// Solve for t > 0 such that f(x - t g) = f(x0).
///
/// # Arguments
/// - `f0`: f(x_k)
/// - `x`: x_k
/// - `g`: ∇f(x_k)
/// - `eval_f_at`: closure evaluating f at a candidate point
///
/// # Returns
/// - Ok(t) if found
/// - Err(&'static str) on failure (no bracket or numerical issue)
pub fn find_levelset_crossing<F>(
    cfg: BracketBisectionCfg,
    f0: f64,
    x: &Array1<f64>,
    g: &Array1<f64>,
    mut eval_f_at: F,
) -> Result<f64, &'static str>
where
    F: FnMut(&Array1<f64>) -> f64,
{
    assert!(cfg.t0 > 0.0 && cfg.growth > 1.0 && cfg.max_expand > 0);

    // φ(t) = f(x - t g) - f0
    let mut t_lo = 0.0;
    let mut t_hi = cfg.t0;

    // Ensure descent at very small t: φ'(0) = -||g||² < 0.
    // Expand t_hi until φ(t_hi) >= 0 to bracket a root in (t_lo, t_hi).
    let mut phi_hi = {
        let x_trial = x - &(g * t_hi);
        eval_f_at(&x_trial) - f0
    };
    let mut expand_count = 0;

    while phi_hi < 0.0 && expand_count < cfg.max_expand {
        t_lo = t_hi;
        t_hi *= cfg.growth;
        let x_trial = x - &(g * t_hi);
        phi_hi = eval_f_at(&x_trial) - f0;
        expand_count += 1;
    }

    if phi_hi < 0.0 {
        return Err("Failed to bracket root within max expansions");
    }
    if t_lo == 0.0 && phi_hi == 0.0 {
        // Rare exact hit
        return Ok(t_hi);
    }

    // Bisection on (t_lo, t_hi) for φ(t) = 0
    let mut lo = t_lo;
    let mut hi = t_hi;

    for _ in 0..cfg.max_bisect {
        let mid = 0.5 * (lo + hi);
        let x_mid = x - &(g * mid);
        let phi_mid = eval_f_at(&x_mid) - f0;

        if phi_mid.abs() == 0.0 || (hi - lo).abs() <= cfg.tol_t {
            return Ok(mid);
        }

        // φ(lo) < 0 by construction; if φ(mid) < 0, move lo up; else hi down.
        if phi_mid < 0.0 {
            lo = mid;
        } else {
            hi = mid;
        }
    }

    Ok(0.5 * (lo + hi))
}

