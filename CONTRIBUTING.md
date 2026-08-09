# Contributing Guidelines  
Deterministic Signing Episodes for Distributed Trust

## 1. Introduction  
Thank you for your interest in contributing to the Deterministic Signing Episodes project.  
This system is built around **strict determinism**, **auditability**, and **reproducibility**, and all contributions must preserve these guarantees.

Before contributing, please review the core documentation:

- Architecture  
- Phase_I  
- Phase_II  
- Phase_III  
- Verification

These documents define the execution model that all contributions must follow.

---

## 2. Contribution Philosophy  
This project prioritizes:

- **deterministic execution**  
- **explicit inputs only**  
- **no randomness**  
- **no hidden or global state**  
- **ephemeral secret handling**  
- **complete auditability**  
- **reproducible tests**  

Any contribution that introduces nondeterminism will be rejected.

---

## 3. Types of Contributions  
You may contribute in the following areas:

### Code Contributions  
- deterministic episode improvements  
- audit record enhancements  
- deterministic Shamir reconstruction logic  
- zeroization mechanisms  
- protocol definitions  
- multi‑agent coordination logic  

### Documentation Contributions  
- improving clarity  
- adding examples  
- refining technical descriptions  
- extending Phase II / Phase III guidelines  

### Testing Contributions  
- deterministic test cases  
- reproducibility validation  
- audit consistency checks  
- threshold reconstruction tests  

Relevant docs:  
- episode_determinism  
- audit_consistency  
- shamir_reconstruction

---

## 4. Requirements for Code Contributions

### Determinism Requirements  
All code **must**:

- avoid randomness  
- avoid system time  
- avoid global state  
- avoid environment‑dependent behavior  
- avoid nondeterministic ordering  
- produce identical outputs for identical inputs  

### Security Requirements  
All code **must**:

- use ephemeral secret handling  
- zeroize sensitive material  
- avoid persistent secrets  
- avoid side effects  

Relevant docs:  
- zeroize  
- audit

### Testing Requirements  
All new code **must** include deterministic tests.  
Tests must be reproducible across machines and environments.

---

## 5. Contribution Process

### Step 1 — Fork the Repository  
Create your own fork and clone it locally.

### Step 2 — Create a Feature Branch  
Use a clear, descriptive name:

```
feature/deterministic-episode-enhancement
fix/audit-record-structure
docs/phase-iii-clarification
```

### Step 3 — Implement Changes  
Ensure your changes follow:

- deterministic execution rules  
- auditability rules  
- zeroization rules  
- reproducibility rules  

### Step 4 — Add Deterministic Tests  
Every change must include tests in the `tests/` directory.

### Step 5 — Submit a Pull Request  
Your PR must include:

- a clear description  
- justification for the change  
- confirmation that determinism is preserved  
- confirmation that tests pass  

---

## 6. Review Process  
All contributions undergo deterministic review:

- execution determinism is validated  
- audit record consistency is checked  
- zeroization behavior is inspected  
- tests must pass reproducibly  
- documentation must remain consistent  

If a contribution introduces nondeterminism, it will be rejected.

---

## 7. Phase‑Specific Contribution Rules

### Phase I  
Frozen.  
No modifications allowed.

### Phase II  
Contributions may extend:

- deterministic SSS  
- threshold approval  
- multi‑agent coordination  
- protocol definitions  

### Phase III  
Contributions may extend:

- interoperability  
- reference integrations  
- adoption guidelines  
- standardization artifacts  

Relevant docs:  
- roadmap

---

## 8. Code Style  
Follow Rust best practices:

- explicit types  
- no implicit behavior  
- clear error handling  
- deterministic ordering  
- no hidden side effects  

---

## 9. Licensing  
By contributing, you agree that your contributions may be incorporated under the project’s licensing model described in:

- legal.md

---

## 10. Contact  
For questions or coordination, please refer to:

- team.md

---
