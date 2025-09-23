//! Tiny linear algebra helpers specialized for 2×2 systems and vector geometry.

use ndarray::Array1;

/// Solve the 2×2 linear system M [a, b]^T = q.
///
/// M = [[m11, m12],
///      [m21, m22]]
///
/// Returns (a, b) or None if near-singular.
pub fn solve_2x2(
    m11: f64, m12: f64,
    m21: f64, m22: f64,
    q1: f64,  q2: f64,
    det_eps: f64,
) -> Option<(f64, f64)> {
    let det = m11 * m22 - m12 * m21;
    if det.abs() <= det_eps {
        return None;
    }
    let inv11 =  m22 / det;
    let inv12 = -m12 / det;
    let inv21 = -m21 / det;
    let inv22 =  m11 / det;
    let a = inv11 * q1 + inv12 * q2;
    let b = inv21 * q1 + inv22 * q2;
    Some((a, b))
}

/// Cosine of angle between u and v. Returns None if either is near zero.
pub fn cos_angle(u: &Array1<f64>, v: &Array1<f64>, eps: f64) -> Option<f64> {
    let nu = u.dot(u).sqrt();
    let nv = v.dot(v).sqrt();
    if nu <= eps || nv <= eps {
        return None;
    }
    Some(u.dot(v) / (nu * nv))
}

