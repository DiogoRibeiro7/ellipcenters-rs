# Contributing to `ellipcenters-rs`

Thanks for your interest in improving the Rust implementation of the **Method of Ellipcenters (ME)**. This guide explains how to propose changes, report bugs, and submit pull requests.

> TL;DR: Open an issue first for non‑trivial changes, keep PRs focused, add tests + docs, and make CI green (`fmt`, `clippy`, `test`).

---

## Code of Conduct

We enforce the [Contributor Covenant v2.1](CODE_OF_CONDUCT.md). By participating, you agree to uphold this code. If you experience or witness unacceptable behavior, please contact the maintainers privately (see **Security** for email) or open a confidential issue.

---

## How you can contribute

* **Bug reports:** Minimal, reproducible examples and environment details.
* **Feature requests:** Explain motivation, alternatives considered, and proposed API (if applicable).
* **Documentation:** Improve Rustdocs, README, examples, comments.
* **Tests & benchmarks:** Add unit/integration tests, suggest benchmark scenarios (Criterion optional).
* **Maintenance:** Refactors, dependency updates, CI fixes.

Before starting substantial work, please **open an issue** to reach consensus on the approach.

---

## Development setup

1. Install Rust (stable) via [`rustup`](https://rustup.rs/).
2. Clone the repository and build:

   ```bash
   git clone https://github.com/diogoribeiro7/ellipcenters-rs
   cd ellipcenters-rs
   cargo build
   ```
3. Run the demo:

   ```bash
   cargo run --bin demo
   ```
4. Run tests with backtraces enabled:

   ```bash
   RUST_BACKTRACE=1 cargo test --all
   ```

> **Tip:** If linking BLAS is inconvenient, keep the pure-`ndarray` path (no external BLAS). The library does not require `unsafe` for core functionality.

---

## Project layout

```
src/
  lib.rs           # crate entry, re-exports
  obj.rs           # Objective trait & QuadraticOracle
  solver.rs        # EllipCenters solver
  line_search.rs   # bracketing + bisection for level-set condition
  small_linalg.rs  # 2×2 solver and geometry helpers
src/bin/demo.rs    # example binary
```

---

## Coding standards

* **Style & formatting:**

  ```bash
  rustup component add rustfmt clippy
  cargo fmt --all -- --check
  cargo clippy --all-targets -- -D warnings
  ```
* **Docs:** Public items **must** have rustdoc comments and, when relevant, a small example. Prefer clear, concise explanations.
* **Errors:** Avoid panics in library code. Prefer `Result<_, Error>` with `thiserror` for typed errors.
* **Safety:** Avoid `unsafe`. If absolutely necessary, encapsulate it, add thorough comments, and justify invariants.
* **Dependencies:** Keep the dependency set minimal; justify any new crates.
* **Performance:** Measure before optimizing. Add micro-benchmarks when proposing perf changes.

---

## Testing policy

* **Unit tests:** Co-located with modules (`mod tests { ... }`).
* **Integration tests:** Put in `tests/` when exercising public APIs end-to-end.
* **Determinism:** Tests must be deterministic; seed RNGs when needed.
* **Coverage:** Prefer tests for edge cases: collinearity path, near-singular 2×2 system, line-search failures, and the quadratic fast‑path.

Optional (not required in CI):

```bash
# Criterion benchmarks (if added)
cargo bench
```

---

## Commit & branch conventions

* **Commit messages:** Imperative mood, e.g., `Add quadratic fast-path`.
* **Conventional Commits (recommended):** `feat:`, `fix:`, `docs:`, `refactor:`, `perf:`, `test:`, `build:`, `ci:`, `chore:`.
* **Branches:**

  * `main` is protected by CI.
  * Feature work should be on topic branches (`feat/…`, `fix/…`).

Update `CHANGELOG.md` under **Unreleased** for user-visible changes.

---

## Pull request checklist

Before requesting review, please ensure:

* [ ] Tests added/updated; all tests pass (`cargo test`).
* [ ] Lints & formatting pass (`cargo fmt`, `cargo clippy -D warnings`).
* [ ] Public APIs documented (rustdoc + examples where useful).
* [ ] `CHANGELOG.md` updated (Unreleased section).
* [ ] No gratuitous API breakage; if breaking, call out clearly and motivate.

PRs should be **small and focused**. Large refactors are harder to review—consider splitting them.

---

## Issue triage labels (suggested)

* `bug`, `regression`
* `enhancement`
* `documentation`
* `good first issue`, `help wanted`
* `performance`
* `blocked`, `needs reproduction`

---

## Release process (maintainers)

1. Ensure `main` is green; update `CHANGELOG.md`.
2. Bump `version` in `Cargo.toml` following SemVer.
3. Tag the release: `git tag -a vX.Y.Z -m "vX.Y.Z" && git push --tags`.
4. Publish to crates.io:

   ```bash
   cargo publish
   ```
5. Draft GitHub Release notes (or use Release Drafter) and link to the tag.

---

## Security

Please **do not** file public issues for security vulnerabilities. Email **[security@your-org.example](mailto:security@your-org.example)** with details (impact, reproduction steps, version/commit). See [SECURITY.md](SECURITY.md).

---

## License

By contributing, you agree that your contributions will be dual-licensed under the terms of the **MIT** license and **Apache 2.0** license, at the repository’s choice (see `LICENSE-MIT` and `LICENSE-APACHE`).

---

## Acknowledgments

We implement algorithmic ideas by Behling, Aquines, Zanatta, and Guigues (2025). Thanks to all contributors who help make the crate robust, documented, and fast.

