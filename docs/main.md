## Runtime Entrypoint (`src/main.rs`)

The `src/main.rs` file provides the **minimal deterministic runtime entrypoint** for the project.  
It is intentionally simple and contains **no business logic**, in accordance with the architectural principles of Phase I.

---

## 1. Purpose

The runtime entrypoint is responsible for:

- initializing a deterministic `EpisodeInput`  
- invoking the deterministic signing episode  
- printing deterministic output  
- demonstrating the execution flow for Phase I  

It does **not**:

- perform CLI parsing  
- perform RPC handling  
- manage distributed agents  
- manage secrets  
- perform cryptographic operations  

These responsibilities are introduced in Phase II and Phase III.

---

## 2. Deterministic Behavior

The runtime must remain:

- deterministic  
- reproducible  
- environment‑independent  
- free of randomness  
- free of global state  

This ensures that Phase I remains a **frozen technical artifact**.

---

## 3. Phase I Behavior

In Phase I:

- inputs are static  
- shares are placeholders  
- reconstruction is deterministic but non‑cryptographic  
- signature generation is deterministic but non‑cryptographic  
- audit records are minimal  
- zeroization is performed inside the episode  

The runtime simply demonstrates the deterministic execution model.

---

## 4. Phase II Behavior

In Phase II:

- runtime may expose a CLI interface  
- runtime may accept JSON or binary inputs  
- runtime may integrate with distributed agents  
- runtime may serialize deterministic episode outputs  
- runtime may provide deterministic RPC endpoints  

The entrypoint becomes part of the distributed trust workflow.

---

## 5. Phase III Behavior

In Phase III:

- runtime becomes part of the Distributed Trust Protocol  
- runtime may integrate with SeedTools  
- runtime may enforce threshold approval policies  
- runtime may produce cryptographically bindable audit logs  
- runtime may interact with secure memory regions  

---

## 6. Integration Points

### Episode Module  
The runtime calls:

- `Episode::execute()`

to perform deterministic signing.

### Shamir Module  
The runtime constructs:

- `Share` objects

for deterministic reconstruction.

### Audit Module  
The runtime prints:

- deterministic audit records

### Zeroize Module  
Zeroization is performed inside the episode.

---

## 7. Status

The runtime entrypoint is:

- stable  
- deterministic  
- Phase I‑compliant  
- ready for Phase II extensions  
- foundational for Phase III distributed trust  

---
