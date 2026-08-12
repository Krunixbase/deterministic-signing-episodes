## Shamir Module — Deterministic Secret Reconstruction  
The `src/core/shamir` module provides the deterministic secret reconstruction mechanism used by the signing episode.  
In **Phase I**, this module contains a **placeholder deterministic implementation**.  
In **Phase II**, it becomes a full deterministic Shamir Secret Sharing (SSS) engine.

---

## 1. Purpose  
The module is responsible for:

- representing secret shares  
- reconstructing an ephemeral signing secret  
- ensuring deterministic behavior  
- providing reproducible output for the signing episode  
- preparing the foundation for Phase II distributed trust

---

## 2. Module Structure

```
src
└──core/
   └──shamir/
      └──mod.rs
```

The module defines:

- `Share` — deterministic representation of a secret share  
- `Secret` — deterministic representation of the reconstructed secret  
- `reconstruct()` — deterministic reconstruction function  

---

## 3. Deterministic Properties

The module guarantees:

- **no randomness**  
- **no global state**  
- **no environment‑dependent behavior**  
- **no persistent secrets**  
- **reproducible reconstruction**  
- **serializable secret representation**  

These properties are required for Phase I determinism and Phase II distributed trust.

---

## 4. Phase I Behavior (Placeholder)

In Phase I:

- shares are simple `(id, value)` pairs  
- reconstruction concatenates share values deterministically  
- the resulting `Secret` is a deterministic string representation  
- no cryptographic operations are performed  
- no threshold logic is implemented  
- no multi‑agent coordination is required  

This placeholder allows:

- deterministic testing  
- auditability  
- reproducibility  
- stable API for Phase II upgrades  

---

## 5. Phase II Behavior (Deterministic SSS)

In Phase II:

- `Share` becomes a real Shamir share  
- `Secret` becomes a real reconstructed key  
- reconstruction uses deterministic Shamir Secret Sharing  
- multi‑agent coordination is introduced  
- audit records include share metadata  
- serialization becomes part of the distributed protocol  

The API remains the same — only the internal logic changes.

---

## 6. Reconstruction Function

### Phase I  
Deterministic placeholder:

```
id:value;id:value;id:value;
```

### Phase II  
Deterministic SSS reconstruction:

- polynomial interpolation  
- deterministic ordering  
- deterministic field operations  
- deterministic share validation  
- deterministic error handling  

---

## 7. Error Handling

Errors are:

- deterministic  
- explicit  
- serializable  
- auditable  

Examples:

- no shares provided  
- invalid share structure  
- reconstruction failure (Phase II)  

---

## 8. Integration Points

### Episode Module  
Used by:

- `src/core/agent/episode.rs`

to reconstruct the ephemeral signing secret.

### Audit Module  
Provides metadata for:

- `src/core/audit/mod.rs`

### Zeroize Module  
Secret is wiped by:

- `src/core/zeroize/mod.rs`

---

## 9. Testing

Test file:

```
tests/shamir_reconstruction.rs
```

Validates:

- deterministic reconstruction  
- reproducibility  
- correct ordering  
- correct error handling  

Phase II adds:

- multi‑agent determinism tests  
- threshold reconstruction tests  

---

## 10. Status

The module is:

- stable  
- deterministic  
- Phase I‑compliant  
- ready for Phase II upgrade  
- foundational for distributed trust  

---
