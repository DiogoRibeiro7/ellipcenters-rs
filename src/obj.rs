use ndarray::{Array1, Array2};

/// Differentiable objective: value and gradient at x.
///
/// Implementors should ensure:
/// - `value(x)` is finite for `x` in the domain
/// - `grad(x)` returns a vector of the same length as `x`
pub trait Objective {
    /// Evaluate f(x).
    fn value(&self, x: &Array1<f64>) -> f64;

    /// Evaluate ∇f(x).
    fn grad(&self, x: &Array1<f64>) -> Array1<f64>;

    /// Optional: For quadratics, provide A·v to enable closed-form t_k.
    /// Default returns None (general objective).
    fn apply_hess_to_vec(&self, _x: &Array1<f64>, _v: &Array1<f64>) -> Option<Array1<f64>> {
        None
    }
}

/// Convenience quadratic oracle: f(x) = 0.5 xᵀ A x − bᵀ x + c, with A ≻ 0.
/// Provides exact gradient and A·v.
pub struct QuadraticOracle {
    pub a: Array2<f64>,
    pub b: Array1<f64>,
    pub c: f64,
}

impl Objective for QuadraticOracle {
    fn value(&self, x: &Array1<f64>) -> f64 {
        // f(x) = 0.5 xᵀ A x − bᵀ x + c
        let ax = self.a.dot(x);
        0.5 * x.dot(&ax) - self.b.dot(x) + self.c
    }

    fn grad(&self, x: &Array1<f64>) -> Array1<f64> {
        // ∇f(x) = A x − b
        self.a.dot(x) - &self.b
    }

    fn apply_hess_to_vec(&self, _x: &Array1<f64>, v: &Array1<f64>) -> Option<Array1<f64>> {
        Some(self.a.dot(v))
    }
}

