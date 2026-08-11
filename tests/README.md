# Tests — Deterministic Signing Episodes

The `tests/` directory contains all deterministic test suites validating the core guarantees of the system.  
Each test ensures that execution, reconstruction, and audit behavior remain **bit‑for‑bit reproducible** across runs, machines, and environments.

These tests are essential for maintaining the strict determinism required by multi‑agent signing workflows.

---

## 📌 Test Suites Overview

### audit_consistency.rs
Validates the deterministic audit trail model.

This test ensures:
- reproducible event sequencing  
- deterministic hashing of audit entries  
- consistent audit logs across repeated runs  
- no nondeterministic branching in audit generation  

Related documentation:  
- **Audit**  
- **Audit Consistency**  

---

### episode_determinism.rs
Validates the determinism of episode execution.

This test ensures:
- no randomness  
- no hidden state  
- no global state  
- identical outputs for identical inputs  
- deterministic branching and execution flow  

Related documentation:  
- **Episode Determinism**  
- **Architecture**  

---

### shamir_reconstruction.rs
Validates deterministic Shamir Secret Sharing (SSS) reconstruction.

This test ensures:
- deterministic polynomial generation  
- reproducible threshold reconstruction  
- consistent share validation  
- strict zeroization of intermediate values  

Related documentation:  
- **Shamir**  
- **Shamir Reconstruction**  

---

## ▶️ Running the Tests

To execute the full deterministic test suite:

```bash
cargo test
```

All tests must pass with **identical results** across environments.  
Any nondeterministic behavior is treated as a critical failure.

---

## 🔁 Deterministic Guarantees Validated by Tests

The test suite collectively verifies:

- **strict determinism**  
- **reproducible auditability**  
- **deterministic threshold approval**  
- **ephemeral secret reconstruction**  
- **zero residual state**  
- **cross‑run consistency**  

These guarantees are foundational for regulatory, cross‑organizational, and high‑assurance signing workflows.

---

## 📚 Relationship to Core Modules

Tests directly correspond to modules in:

```
core/
 ├── agent/
 ├── audit/
 ├── shamir/
 └── zeroize/
```

Documentation alignment ensures that every deterministic rule is both **specified** and **validated**.

---

## 📄 Licensing

All tests are licensed under **GPL‑3.0** as part of the deterministic‑signing‑episodes project.

---
