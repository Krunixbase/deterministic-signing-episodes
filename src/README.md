# Source Code — Deterministic Signing Episodes

The `src/` directory contains the main executable entry point for deterministic‑signing‑episodes.  
This layer connects the deterministic core modules with the runtime environment and provides the top‑level orchestration for episode execution.

---

## 📌 Files Overview

### main.rs
The primary entry point of the application.

Responsibilities:
- initializing deterministic signing episodes  
- invoking the agent execution engine  
- coordinating threshold logic  
- producing reproducible audit output  
- enforcing zeroization after execution  

This file ties together the modules located in:

```
 src
 └──core/
    ├── agent/
    ├── audit/
    ├── shamir/
    └── zeroize/
```

---

## 🔁 Deterministic Execution Flow

The runtime pipeline executed from `main.rs` follows:

1. **Episode initialization**  
2. **Deterministic Shamir reconstruction**  
3. **Agent‑level deterministic execution**  
4. **Audit trail generation**  
5. **Zeroization of sensitive data**  
6. **Reproducible output**  

Every run with identical inputs produces **bit‑for‑bit identical results**.

---

## 🧪 Testing

Tests validating the behavior of the runtime pipeline are located in:

```
tests/
 ├── episode_determinism.rs
 ├── audit_consistency.rs
 └── shamir_reconstruction.rs
```

These tests ensure that the logic invoked from `main.rs` remains strictly deterministic.

---

## 📚 Related Documentation

- **Architecture**  
- **Episode Determinism**  
- **Audit Model**  
- **Shamir Reconstruction**  

These documents describe the deterministic model implemented in the runtime.

---

## 📄 Licensing

The core module is licensed under **MIT**, consistent with the deterministic‑signing‑episodes project.

---