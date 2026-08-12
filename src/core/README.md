# Core Module — Deterministic Signing Episodes

The `src/core/` directory contains the fundamental execution logic for deterministic signing episodes.  
All modules in this directory follow strict reproducibility rules: **no randomness, no hidden state, no global state, no nondeterministic branching**.

This is the heart of the system.

---

## 📌 Module Overview

### agent/
Implements the **deterministic episode engine**.

Key responsibilities:
- orchestrating episode execution  
- reconstructing ephemeral secrets deterministically  
- enforcing threshold approval  
- zeroizing sensitive data after use  
- producing reproducible execution traces  

Main file:
- `episode.rs` — core deterministic execution unit

---

### audit/
Implements the **deterministic audit trail**.

Key responsibilities:
- structured audit record generation  
- reproducible event sequencing  
- consistency guarantees across runs  
- deterministic hashing of audit entries  

Main file:
- `mod.rs` — audit subsystem root

---

### shamir/
Implements **deterministic Shamir Secret Sharing (SSS)**.

Key responsibilities:
- threshold reconstruction without probabilistic behavior  
- deterministic polynomial generation  
- reproducible share validation  
- strict zeroization of intermediate values  

Main file:
- `mod.rs` — deterministic SSS implementation

---

### zeroize/
Implements **deterministic zeroization** routines.

Key responsibilities:
- secure memory wiping  
- deterministic cleanup semantics  
- ensuring no residual state remains after episode execution  

Main file:
- `mod.rs` — zeroization primitives

---

## 🔁 Deterministic Execution Flow

The core module guarantees the following deterministic pipeline:

1. **Episode initialization**  
2. **Ephemeral secret reconstruction**  
3. **Deterministic Shamir threshold logic**  
4. **Agent‑level deterministic execution**  
5. **Audit trail generation**  
6. **Zeroization of all sensitive data**  
7. **Reproducible output**

Every run of the same episode with the same inputs produces **bit‑for‑bit identical results**.

---

## 🧪 Testing

Deterministic behavior is validated through the tests in:

```
tests/
 ├── audit_consistency.rs
 ├── episode_determinism.rs
 └── shamir_reconstruction.rs
```

These tests ensure:
- reproducible audit logs  
- deterministic episode execution  
- deterministic Shamir reconstruction  

---

## 📚 Related Documentation

- **Episode Determinism** — detailed determinism guarantees  
- **Audit Consistency** — reproducible audit model  
- **Shamir Reconstruction** — deterministic threshold logic  
- **Architecture** — system‑level design  

---

## 🔐 Security Guarantees

The core module enforces:

- deterministic secret handling  
- zero residual state  
- reproducible auditability  
- threshold approval without randomness  
- strict memory zeroization  

These guarantees make the system suitable for:
- regulatory environments  
- cross‑organizational signing workflows  
- high‑assurance cryptographic operations  

---

## 📄 Licensing

The core module is licensed under **MIT**, consistent with the deterministic‑signing‑episodes project.


---
