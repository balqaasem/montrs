# Contributing to MontRS

Thank you for your interest in contributing to MontRS! This document outlines the process for contributing and how we manage development channels.

## Development Channels

MontRS follows a channel-based development model, similar to the Rust toolchain. This allows us to balance stability for production users with the need for rapid experimentation.

### 1. Stable Channel
- **Purpose:** Production-ready code.
- **Branch:** `main`
- **Criteria:** All features must be fully tested, documented, and have stable APIs.
- **Release Cadence:** Major/Minor releases as needed.

### 2. Nightly Channel
- **Purpose:** Experimental features, "bleeding edge" development.
- **Branch:** `develop`
- **Criteria:** Features may be incomplete or have breaking API changes. 
- **Usage:** Users can opt-in via `montrs channel nightly`.

## How to Contribute

### Feature Gating
When working on a new feature that isn't ready for stable, use Rust's feature flags and check the project channel.

```rust
if config.project.channel == Channel::Nightly {
    // Experimental logic
}
```

### Pull Request Process
1. **Target Branch:** All new features and bug fixes should target the `develop` branch first.
2. **Tests:** Ensure all tests pass (`montrs test`).
3. **Documentation:** Update relevant `.md` files in the `docs/` directory.
4. **Code Style:** Follow standard Rust conventions. Run `cargo fmt` before submitting.

## Release Train
We periodically merge `develop` into `main` after a period of "hardening" in the nightly channel. If a feature is found to be unstable during this period, it may be reverted or kept behind a feature gate.
