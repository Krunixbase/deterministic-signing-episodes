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
│   .gitignore
│   Cargo.lock
│   Cargo.toml
│   CHANGELOG.md
│   CODE_OF_CONDUCT.md
│   CONTRIBUTING.md
│   ESP_BUDGET.md
│   ESP_PROPOSAL.md
│   ESP_ROADMAP.md
│   LICENSE
│   README.md
│   SECURITY.md
│   Structure_repo.md
│   WHY_ETHEREUM.md
│
├───assets
│   └───branding
│       │   README.md
│       │
│       ├───badges
│       │       deterministic_badge.svg
│       │       deterministic_badge_dark.svg
│       │       deterministic_badge_light.svg
│       │       README.md
│       │
│       ├───colors
│       │       palette.md
│       │       README.md
│       │
│       ├───guidelines
│       │       branding_guidelines.md
│       │       README.md
│       │
│       ├───logo
│       │   │   README.md
│       │   │
│       │   ├───png
│       │   │       deterministic_logo_1024.png
│       │   │       deterministic_logo_220.png
│       │   │       deterministic_logo_320.png
│       │   │       deterministic_logo_512.png
│       │   │
│       │   ├───svg
│       │   │       deterministic_logo.svg
│       │   │       deterministic_logo_mono.svg
│       │   │       deterministic_logo_outline.svg
│       │   │
│       │   └───variants
│       │       ├───mono
│       │       │       deterministic_logo_mono.svg
│       │       │
│       │       ├───outline
│       │       │       deterministic_logo_outline.svg
│       │       │
│       │       ├───premium
│       │       │       deterministic_logo_premium.svg
│       │       │
│       │       └───solid
│       │               deterministic_logo_solid.svg
│       │
│       └───typography
│               fonts.md
│               README.md
│
├───docs
│       agent_episode.md
│       architecture.md
│       audit.md
│       audit_consistency.md
│       episode_determinism.md
│       legal.md
│       main.md
│       phase_I.md
│       phase_II.md
│       phase_III.md
│       README.md
│       roadmap.md
│       shamir.md
│       shamir_reconstruction.md
│       team.md
│       verification.md
│       zeroize.md
│
├───examples
│       eth_integration.rs
│       README.md
│
├───src
│   │   lib.rs
│   │   main.rs
│   │   README.md
│   │
│   └───core
│       │   mod.rs
│       │   README.md
│       │
│       ├───agent
│       │       episode.rs
│       │       mod.rs
│       │
│       ├───audit
│       │       mod.rs
│       │
│       ├───shamir
│       │       mod.rs
│       │
│       └───zeroize
│               mod.rs
│
└───tests
        audit_consistency.rs
        episode_determinism.rs
        README.md
        shamir_reconstruction.rs
```

---

## Core Modules

### `src/core/agent/episode.rs`  
Deterministic signing episode implementation.  
Validates inputs, reconstructs ephemeral material, produces signatures, emits audit records, and zeroizes secrets.

### `src/core/audit/`  
Structured audit record generation.  
Deterministic, reproducible, environment‑independent.

### `src/core/shamir/`  
Deterministic Shamir Secret Sharing reconstruction.  
Threshold‑based ephemeral secret recovery.

### `src/core/zeroize/`  
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
    let input = core::agent::episode::EpisodeInput {
        artifact_hash: "example_hash".to_string(),
        shares: vec![], // explicit deterministic shares
        metadata: core::agent::episode::EpisodeMetadata {
            episode_id: "ep1".to_string(),
            description: "deterministic test episode".to_string(),
        },
    };

    let result = core::agent::episode::Episode::execute(input);

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
