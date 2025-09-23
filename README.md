# ellipcenters-rs

Rust implementation of the **Method of Ellipcenters (ME)** for unconstrained optimization.

> Behling, Aquines, Zanatta, Guigues (2025). “Introducing the Method of Ellipcenters, a new first order technique for unconstrained optimization”.

[![CI](https://github.com/your-org/ellipcenters-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/your-org/ellipcenters-rs/actions/workflows/ci.yml)
[![License: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%20or%20Apache--2.0-blue.svg)](#license)
[![Crates.io](https://img.shields.io/crates/v/ellipcenters.svg)](https://crates.io/crates/ellipcenters)
[![Docs](https://docs.rs/ellipcenters/badge.svg)](https://docs.rs/ellipcenters)

---

## Overview

The **Method of Ellipcenters (ME)** is a first-order optimization method. Each iteration builds an ellipse inside the 2D affine subspace spanned by two gradients and moves to the **center** of that ellipse. For strongly convex quadratics, ME enjoys **linear convergence** and is competitive with Barzilai–Borwein and Conjugate Gradient, especially on ill‑conditioned problems.

This crate provides:

* A typed `Objective` trait for smooth objectives (value and gradient).
* A robust 1‑D solver to enforce the level‑set condition `f(x - t g) = f(x)`.
* A stable 2×2 solve to compute the ellipse center, with numeric fallbacks.
* A quadratic fast‑path (`A v`) that matches the paper’s closed forms.

> **Per‑iteration cost:** 2 gradient evaluations + level‑set scalar root‑finding + tiny 2×2 linear solve.

---

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
ellipcenters = { version = "0.1", package = "ellipcenters" }
```

From source:

```bash
git clone https://github.com/your-org/ellipcenters-rs
cd ellipcenters-rs
cargo run --bin demo
```

> The crate uses `ndarray`. BLAS is optional. If OpenBLAS linking is inconvenient, keep the pure‑`ndarray` path.

---

## Quick Start

```rust
use ellipcenters::{EllipCenters, Objective};
use ndarray::Array1;

struct Rosenbrock;
impl Objective for Rosenbrock {
    fn value(&self, x: &Array1<f64>) -> f64 {
        let (x1, x2) = (x[0], x[1]);
        100.0 * (x2 - x1 * x1).powi(2) + (1.0 - x1).powi(2)
    }
    fn grad(&self, x: &Array1<f64>) -> Array1<f64> {
        let (x1, x2) = (x[0], x[1]);
        Array1::from(vec![
            -400.0 * x1 * (x2 - x1 * x1) - 2.0 * (1.0 - x1),
             200.0 * (x2 - x1 * x1),
        ])
    }
}

let solver = EllipCenters::default();
let x0 = Array1::from(vec![-1.2, 1.0]);
let res = solver.solve(&Rosenbrock, &x0).unwrap();
println!("status: {:?}, iters: {}, f: {}, ||g||: {}",
         res.stop, res.iters, res.f, res.grad_norm);
```

---

## API

### Traits

* `Objective`

  * `fn value(&self, x: &Array1<f64>) -> f64`
  * `fn grad(&self, x: &Array1<f64>) -> Array1<f64>`
  * `fn apply_hess_to_vec(&self, x: &Array1<f64>, v: &Array1<f64>) -> Option<Array1<f64>>`

    * Optional. For quadratics, return `A v` to enable the fast step.

### Solver

* `EllipCenters` / `EllipCentersCfg` / `EllipCentersResult` / `StopReason`

```rust
let solver = EllipCenters { cfg: EllipCentersCfg { ..Default::default() } };
let out = solver.solve(&my_objective, &x0)?;
```

Key config fields:

* `tol_grad`: stopping tolerance on ||∇f(x)||₂.
* `max_iters`: maximum iterations.
* `collinear_eps`: threshold for collinearity (cosine proximity to 1).
* `det_eps`: determinant safeguard for 2×2 solve.
* `ls`: bracketing + bisection settings for the level‑set crossing.

---

## Algorithm sketch

At iterate `x_k`:

1. Compute `g_x = ∇f(x_k)`.
2. Find `t_k > 0` such that `f(x_k - t_k g_x) = f(x_k)`.

   * Quadratic fast‑path: `t_k = 2‖g_x‖² / (g_xᵀ A g_x)`.
   * General objectives: bracket + bisection.
3. Set `y_k = x_k - t_k g_x` and `g_y = ∇f(y_k)`.
4. If `g_x` and `g_y` are (near) collinear, take midpoint `x_{k+1} = 0.5 (x_k + y_k)`.
5. Else solve the 2×2 system for `(α, β)` to get
   `x_{k+1} = x_k + α g_x + β g_y`.

Numerical safeguards: determinant checks, midpoint fallbacks, conservative line‑search failure mode.

---

## Examples

### Ill‑conditioned quadratic (demo binary)

```bash
cargo run --bin demo
```

### Custom quadratic with `A v` fast‑path

```rust
use ellipcenters::{Objective, QuadraticOracle, EllipCenters};
use ndarray::{Array1, Array2};

let mut a = Array2::<f64>::zeros((2, 2));
a[(0,0)] = 1.0; a[(1,1)] = 1000.0;
let b = Array1::from(vec![1.0, 1.0]);
let quad = QuadraticOracle { a, b, c: 0.0 };

let x0 = Array1::from(vec![10.0, -10.0]);
let res = EllipCenters::default().solve(&quad, &x0).unwrap();
```

---

## Performance notes

* Two gradients per iteration; scalar root‑finding typically converges quickly.
* For quadratics with `A v`, the method mirrors the paper exactly and is notably strong on ill‑conditioned cases.
* Preconditioning support is planned.

---

## Limitations

* Current theoretical guarantees are strongest for **quadratic strongly convex** problems (matching the paper).
* For general smooth objectives, the solver uses an identity‑metric fallback for the 2×2 system; this generalization is pragmatic but not yet analyzed in theory.

---

## Roadmap

* Preconditioners and user‑provided metrics.
* Benchmarks vs GD / BB / CG on diagonals and dense instances.
* Optional `no_std` / BLAS‑free builds.
* Python bindings via PyO3.

---

## Contributing & Governance

Contributions are welcome—see [CONTRIBUTING.md](CONTRIBUTING.md) and [CODE\_OF\_CONDUCT.md](CODE_OF_CONDUCT.md).

For security issues, see [SECURITY.md](SECURITY.md).

---

## Citing

If you use this software, please cite **this crate** and the **ME paper**. See [CITATION.cff](CITATION.cff).

---

## License

* MIT License ([LICENSE‑MIT](LICENSE-MIT))


---

## Acknowledgments

This crate implements the algorithmic ideas introduced by Roger Behling, Ramyro Corrêa Aquines, Eduarda Ferreira Zanatta, and Vincent Guigues (2025).
