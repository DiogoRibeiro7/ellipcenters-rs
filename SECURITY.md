# Security Policy

## Supported Versions

We support the latest released version of `ellipcenters-rs`. Security fixes are generally applied only to the most recent release.

## Reporting a Vulnerability

Please **do not** file public GitHub issues for security vulnerabilities.

Email **[security@your-org.example](mailto:security@your-org.example)** with the following:

* A detailed description of the issue and potential impact
* Steps to reproduce or a proof of concept
* Affected version/commit and environment details
* Your contact information

We aim to acknowledge receipt within **5 business days**. We will work with you to reproduce and assess the issue, determine scope, and prepare a fix and coordinated disclosure timeline.

## Disclosure Policy

* We prefer **coordinated disclosure**. Once a fix is available, we will publish a security advisory and a patch release.
* Credit will be given to reporters who responsibly disclose vulnerabilities, unless you request anonymity.

## Hardening Guidance

* Use the latest stable Rust toolchain.
* Enable CI checks (`clippy`, `fmt`, tests\`) to catch regressions.
* Consider running in minimal‑privilege environments and validating untrusted inputs if you wrap this library in services.

Thank you for helping keep the community safe.
