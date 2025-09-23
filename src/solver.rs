use crate::line_search::{find_levelset_crossing, BracketBisectionCfg};
use crate::obj::Objective;
use crate::small_linalg::{cos_angle, solve_2x2};

use ndarray::{arr1, Array1};

use thiserror::Error;

/// Reasons the solver stops.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StopReason {
    GradNormBelowTol,
    MaxItersReached,
    LineSearchFailure,
    NumericalIssue,
}

/// Configuration for the Ellipcenters optimizer.
#[derive(Debug, Clone)]
pub struct EllipCentersCfg {
    /// Gradient norm tolerance (L2).
    pub tol_grad: f64,
    /// Maximum iterations.
    pub max_iters: usize,
    /// Collinearity threshold on |cos θ| ≥ 1 - eps ⇒ treat as collinear.
    pub collinear_eps: f64,
    /// Determinant safeguard for 2×2 solve.
    pub det_eps: f64,
    /// Line-search config for t_k.
    pub ls: BracketBisectionCfg,
}

impl Default for EllipCentersCfg {
    fn default() -> Self {
        Self {
            tol_grad: 1e-8,
            max_iters: 1_000,
            collinear_eps: 1e-8,
            det_eps: 1e-14,
            ls: BracketBisectionCfg::default(),
        }
    }
}

/// Optimization output.
#[derive(Debug, Clone)]
pub struct EllipCentersResult {
    pub x: Array1<f64>,
    pub f: f64,
    pub grad_norm: f64,
    pub iters: usize,
    pub stop: StopReason,
}

#[derive(Debug, Error)]
pub enum EllipCentersError {
    #[error("dimension mismatch between initial x and objective gradient")]
    DimMismatch,
}

/// Ellipcenters optimizer (stateless wrapper with a `solve` method).
pub struct EllipCenters {
    pub cfg: EllipCentersCfg,
}

impl Default for EllipCenters {
    fn default() -> Self {
        Self { cfg: EllipCentersCfg::default() }
    }
}

impl EllipCenters {
    /// Run ME starting from `x0`.
    pub fn solve<O: Objective>(
        &self,
        obj: &O,
        x0: &Array1<f64>,
    ) -> Result<EllipCentersResult, EllipCentersError> {
        let mut x = x0.clone();

        // Initial evals
        let mut f_x = obj.value(&x);
        let mut g_x = obj.grad(&x);
        if g_x.len() != x.len() {
            return Err(EllipCentersError::DimMismatch);
        }

        // Main loop
        for k in 0..self.cfg.max_iters {
            let gnorm = g_x.dot(&g_x).sqrt();
            if gnorm <= self.cfg.tol_grad {
                return Ok(EllipCentersResult {
                    x, f: f_x, grad_norm: gnorm, iters: k, stop: StopReason::GradNormBelowTol
                });
            }

            // === Step 1: find t_k s.t. f(x - t g) = f(x) ===
            let t_k = if let Some(ag) = obj.apply_hess_to_vec(&x, &g_x) {
                // Quadratic fast path: t_k = 2 ||g||^2 / (gᵀ A g)
                let num = gnorm * gnorm;
                let denom = g_x.dot(&ag);
                // guard against pathological A or zero denom
                if denom.abs() > 0.0 {
                    2.0 * num / denom
                } else {
                    // fallback to robust line search
                    find_levelset_crossing(
                        self.cfg.ls, f_x, &x, &g_x, |p| obj.value(p)
                    ).map_err(|_| EllipCentersError::DimMismatch /* reuse */)
                    .map_err(|_| EllipCentersError::DimMismatch)? // convert to anyhow if desired
                }
            } else {
                // General objective: bracket + bisection
                match find_levelset_crossing(self.cfg.ls, f_x, &x, &g_x, |p| obj.value(p)) {
                    Ok(t) => t,
                    Err(_) => {
                        // Could not bracket: treat as line-search failure.
                        return Ok(EllipCentersResult {
                            x, f: f_x, grad_norm: gnorm, iters: k, stop: StopReason::LineSearchFailure
                        });
                    }
                }
            };

            // Build y_k = x_k - t_k g_k
            let y = &x - &( &g_x * t_k );
            let g_y = obj.grad(&y);

            // === Step 2: decide collinearity ===
            let col_ok = cos_angle(&g_x, &g_y, 1e-16)
                .map(|c| (1.0 - c.abs()) <= self.cfg.collinear_eps)
                .unwrap_or(true); // if any gradient ~0, treat as collinear and take midpoint.

            let x_next = if col_ok {
                // Midpoint step (equivalent to optimal GD step for quadratics when collinear).
                0.5 * (&x + &y)
            } else {
                // === Step 3: solve 2×2 system for (α, β) ===
                // Notation shortcuts:
                let gx = &g_x;    // ∇f(x_k)
                let gy = &g_y;    // ∇f(y_k)

                // We need inner products with A for quadratic case; for general f, emulate via secant-like A?
                // The paper's exact 2×2 uses ⟨·, A·⟩. For general f (no Hessian), we approximate with identity metric,
                // which still defines a center in the 2D plane. This is a pragmatic generalization.
                //
                // If apply_hess_to_vec exists, use it to get accurate A-geometry; else use I-geometry.
                let (gx_A_gx, gx_A_gy, gy_A_gy) = if let Some(agx) = obj.apply_hess_to_vec(&x, gx) {
                    // Use A from objective for accurate geometry
                    let agy = obj.apply_hess_to_vec(&y, gy).unwrap_or_else(|| gy.clone());
                    (gx.dot(&agx), gx.dot(&agy), gy.dot(&agy))
                } else {
                    // Identity metric fallback
                    (gx.dot(gx), gx.dot(gy), gy.dot(gy))
                };

                // Gram-like 2×2 (see paper Lemma 2.2) in the chosen metric:
                let m11 = gx_A_gx;
                let m12 = gx_A_gy;
                let m21 = gx_A_gy;
                let m22 = gy_A_gy;

                // Right-hand side q = [-||gx||^2, -⟨gx, gy⟩] in Euclidean geometry
                let q1 = -gx.dot(gx);
                let q2 = -gx.dot(gy);

                if let Some((alpha, beta)) = solve_2x2(m11, m12, m21, m22, q1, q2, self.cfg.det_eps) {
                    // x_{k+1} = x_k + α ∇f(x_k) + β ∇f(y_k)
                    &x + &(gx * alpha) + &(gy * beta)
                } else {
                    // Nearly singular: fallback to midpoint for safety
                    0.5 * (&x + &y)
                }
            };

            // Prepare next loop
            x = x_next;
            f_x = obj.value(&x);
            g_x = obj.grad(&x);
        }

        // Max iters
        let gnorm = g_x.dot(&g_x).sqrt();
        Ok(EllipCentersResult {
            x, f: f_x, grad_norm: gnorm, iters: self.cfg.max_iters, stop: StopReason::MaxItersReached
        })
    }
}

