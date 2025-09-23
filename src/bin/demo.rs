use ellipcenters::{EllipCenters, EllipCentersCfg, Objective, QuadraticOracle};
use ndarray::{Array1, Array2};

fn main() {
    // Build a small ill-conditioned quadratic:
    // A = diag(1, 1000), b = [1, 1], c = 0  ⇒  x* = A^{-1} b = [1, 0.001]
    let mut a = Array2::<f64>::zeros((2, 2));
    a[(0, 0)] = 1.0;
    a[(1, 1)] = 1000.0;
    let b = Array1::from(vec![1.0, 1.0]);

    let quad = QuadraticOracle { a, b, c: 0.0 };

    let x0 = Array1::from(vec![10.0, -10.0]);

    let solver = EllipCenters { cfg: EllipCentersCfg {
        tol_grad: 1e-10,
        max_iters: 1000,
        collinear_eps: 1e-10,
        det_eps: 1e-20,
        ls: Default::default(),
    }};

    let out = solver.solve(&quad, &x0).expect("solver failed");
    println!("status:      {:?}", out.stop);
    println!("iters:       {}", out.iters);
    println!("f(x):        {:.12e}", out.f);
    println!("||grad||:    {:.12e}", out.grad_norm);
    println!("x*:          [1.0, 0.001]");
    println!("x (found):   {:?}", out.x);
}

