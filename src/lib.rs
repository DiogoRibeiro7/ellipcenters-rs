//! Method of Ellipcenters (ME) for unconstrained optimization.
//!
//! This crate provides a reference implementation of the algorithm introduced in
//! “Introducing the method of ellipcenters” (Behling et al., 2025).
//!
//! # Features
//! - General differentiable objectives via [`Objective`] trait.
//! - Robust 1D bracketing+bisection to find `t_k` such that `f(x_k - t_k g_k) = f(x_k)`.
//! - 2D affine subspace step solving the `(α,β)` 2×2 system with stability guards.
//! - Quadratic fast-path helper (optional) to supply `A g` for closed-form `t_k`.
//! - Collinearity detection with safe fallback to midpoint step.
//!
//! # Example
//! See `src/bin/demo.rs` for a runnable example on a quadratic.

pub mod obj;
pub mod solver;
pub mod line_search;
mod small_linalg;

pub use obj::{Objective, QuadraticOracle};
pub use solver::{EllipCenters, EllipCentersCfg, EllipCentersResult, StopReason};
