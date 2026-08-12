# Examples — Deterministic Signing Episodes

The `examples/` directory contains practical, self‑contained examples demonstrating how deterministic signing episodes can be integrated into external systems.  
Each example is minimal, reproducible, and aligned with the deterministic execution guarantees of the core modules.

---

## 📌 Available Examples

### eth_integration.rs
A minimal example showing how deterministic signing episodes can be embedded into an Ethereum‑based workflow.

This example demonstrates:
- deterministic episode initialization  
- ephemeral secret reconstruction  
- threshold approval logic  
- deterministic audit record generation  
- integration with external signing or verification layers  

---

## ▶️ Running the Example

To execute the example:

```bash
cargo run --example eth_integration
```

This will run the deterministic episode pipeline and produce reproducible audit output.

---

## 🧩 What This Example Shows

- how to embed deterministic episodes inside external systems  
- how to call the agent execution engine  
- how deterministic Shamir reconstruction works in practice  
- how audit logs remain consistent across runs  
- how zeroization is enforced after execution  

This example is intentionally minimal and serves as a template for further integrations.

---

## 🔧 Extending Examples

You can create additional examples by adding new files inside:

```
examples/
    your_example.rs
```

Recommended topics:
- multi‑agent threshold signing  
- cross‑chain deterministic workflows  
- deterministic compliance automation  
- reproducible audit pipelines  
- NGO / multi‑organization verification flows  

---

## 📚 Related Documentation

- **Architecture**  
- **Episode Determinism**  
- **Audit Consistency**  
- **Shamir Reconstruction**  

These documents explain the deterministic model used in the examples.

---

## 📄 Licensing

All examples are licensed under **MIT**, consistent with the deterministic‑signing‑episodes core project.

---
