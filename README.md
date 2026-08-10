<p align="center">
  <img src="assets/branding/logo/png/deterministic_logo_512.png"
       alt="Deterministic Signing Episodes Logo"
       width="180" />
</p>

<h1 align="center">deterministic-signing-episodes</h1>

<p align="center">
Deterministic • Reproducible • Auditable Signing Workflows
</p>

<p align="center">
  <img src="https://img.shields.io/github/license/Krunixbase/deterministic-signing-episodes" />
  <img src="https://img.shields.io/github/repo-size/Krunixbase/deterministic-signing-episodes" />
  <img src="https://img.shields.io/github/issues/Krunixbase/deterministic-signing-episodes" />
  <img src="https://img.shields.io/github/stars/Krunixbase/deterministic-signing-episodes" />
  <img src="https://img.shields.io/badge/rust-1.70%2B-orange" />
  <img src="https://img.shields.io/badge/security-policy-blue" />
</p>

---

## Overview

This repository implements **deterministic signing episodes** — isolated, reproducible execution units that reconstruct trust.

The system is designed for **high‑assurance, multi‑agent signing workflows**, where transparency, reproducibility, and auditability are critical.

The project evolves through three phases:

- **Phase I** — deterministic execution core  
- **Phase II** — deterministic distributed trust  
- **Phase III** — adoption, interoperability, and standardization  

Documentation for each phase is available in the `docs/` directory.

---

## Key Guarantees

- **Strict determinism** — no randomness, no hidden state, no global state  
- **Ephemeral secret handling** — secrets reconstructed only during execution and zeroized afterward  
- **Complete auditability** — every step is recorded in a structured, reproducible audit record  
- **Threshold approval** — deterministic multi‑agent authorization  
- **Deterministic Shamir Secret Sharing (SSS)** — threshold reconstruction without probabilistic behavior  

These guarantees make the system suitable for regulatory, cross‑organizational, and high‑assurance environments.

---


## Overview  
This repository implements **deterministic signing episodes** — isolated, reproducible execution units that reconstruct ephemeral signing material, produce deterministic signatures, and emit complete audit records.

The system is designed for **high‑assurance, multi‑agent signing workflows**, where transparency, reproducibility, and auditability are mandatory.

The project evolves through three phases:

- **Phase I** — deterministic execution core  
- **Phase II** — deterministic distributed trust  
- **Phase III** — adoption, interoperability, and standardization  

Documentation for each phase is available in the `docs/` directory.

---

## Key Guarantees

- **Strict determinism** — no randomness, no hidden state, no global state.  
- **Ephemeral secret handling** — secrets reconstructed only during execution and zeroized afterward.  
- **Complete auditability** — every step is recorded in a structured, reproducible audit record.  
- **Threshold approval** — deterministic multi‑agent authorization.  
- **Deterministic Shamir Secret Sharing (SSS)** — threshold reconstruction without probabilistic behavior.

These guarantees make the system suitable for regulatory, cross‑organizational, and high‑assurance environments.

---

## Repository Structure

```
|   CHANGELOG.md
|   CODE_OF_CONDUCT.md
|   CONTRIBUTING.md
|   LICENSE
|   README.md
|   SECURITY.md
|
+---core
|   +---agent
|   |       episode.rs
|   |
|   +---audit
|   |       mod.rs
|   |
|   +---shamir
|   |       mod.rs
|   |
|   \---zeroize
|           mod.rs
|
+---docs
|       agent_episode.md
|       architecture.md
|       audit.md
|       audit_consistency.md
|       episode_determinism.md
|       legal.md
|       main.md
|       phase_I.md
|       phase_II.md
|       phase_III.md
|       roadmap.md
|       shamir.md
|       shamir_reconstruction.md
|       team.md
|       verification.md
|       zeroize.md
|
+---src
|       main.rs
|
\---tests
        audit_consistency.rs
        episode_determinism.rs
        shamir_reconstruction.rs
```

---

## Core Modules

### `core/agent/episode.rs`  
Deterministic signing episode implementation.  
Validates inputs, reconstructs ephemeral material, produces signatures, emits audit records, and zeroizes secrets.

### `core/audit/`  
Structured audit record generation.  
Deterministic, reproducible, environment‑independent.

### `core/shamir/`  
Deterministic Shamir Secret Sharing reconstruction.  
Threshold‑based ephemeral secret recovery.

### `core/zeroize/`  
Deterministic zeroization of sensitive material.

---

## Documentation

Full documentation is available in the `docs/` directory:

- architecture  
- phase_I  
- phase_II  
- phase_III  
- verification  
- roadmap  
- legal  
- team

Module‑specific documentation:

- agent_episode  
- audit  
- audit_consistency  
- episode_determinism  
- shamir  
- shamir_reconstruction  
- zeroize

---

## Tests

Deterministic tests are located in the `tests/` directory:

- `audit_consistency.rs` — verifies reproducible audit records  
- `episode_determinism.rs` — verifies deterministic episode execution  
- `shamir_reconstruction.rs` — verifies deterministic SSS reconstruction  

All tests:

- rely only on explicit inputs  
- are environment‑independent  
- validate Phase I and Phase II guarantees  
- ensure reproducibility across runs and reviewers

---

## Minimal Example

```rust
fn main() {
    let episode = core::agent::episode::SigningEpisode::new();
    let result = episode.execute(/* explicit deterministic inputs */);

    println!("Signature: {:?}", result.signature);
    println!("Audit: {:?}", result.audit_record);
}
```

---

## Security Model

The system enforces:

- no randomness  
- no hidden state  
- no global state  
- no system time  
- no persistent secrets  
- deterministic zeroization  
- deterministic reconstruction  
- deterministic audit records  

This makes the system suitable for:

- high‑assurance cryptographic workflows  
- multi‑agent signing  
- regulatory environments  
- cross‑organizational trust  
- reproducible research and verification

---

## Licensing

The project uses a **flexible licensing model**, supporting:

- open‑source transparency  
- external auditability  
- optional dual‑licensing  
- controlled commercial use  

Details: legal.md

---

## Contributing

During Phase II, the project is developed by a single author.  
Phase III may involve:

- external reviewers  
- integrators  
- standardization bodies  
- third‑party adopters  

Details: CONTRIBUTING.md

---

## Roadmap

- **Phase I** — deterministic execution core  
- **Phase II** — deterministic distributed trust  
- **Phase III** — adoption, interoperability, standardization  

Details: roadmap.md

---
