# SeedTools Branding — Typography (Technical Specification)

## Purpose
This document defines the typographic rules, font usage, and deterministic constraints for all text-based visual assets within the SeedTools ecosystem.  
Typography ensures consistent visual identity across documentation, UI components, diagrams, and branding materials.

---

## Directory Structure
```
assets/branding/typography
│
└── fonts.md            # Authoritative typography specification
```

---

## Typography Overview

Typography in SeedTools is designed to be:

- **deterministic** — consistent across all repositories and platforms,
- **minimalistic** — focused on clarity and readability,
- **functional** — optimized for technical documentation and cryptographic workflows,
- **modular** — adaptable to both dark and light themes.

All typographic rules are defined in `fonts.md`, which acts as the authoritative source.

---

## Font Requirements

### Primary Typeface
The primary typeface is defined in `fonts.md`.  
It is used for:

- documentation headings,
- UI components,
- diagrams,
- branding materials.

### Secondary Typeface
If a secondary typeface is defined, it is used for:

- code annotations,
- inline technical notes,
- diagrams requiring contrast.

### Deterministic Constraints
- Fonts must not be substituted automatically by rendering tools.
- Only fonts defined in `fonts.md` may be used.
- No additional typefaces may be introduced without updating this specification.

---

## Usage Rules

### Headings
- Must follow the hierarchy defined in `fonts.md`.
- Must maintain consistent spacing and weight.
- Must not use decorative or non-technical styles.

### Body Text
- Must prioritize readability.
- Must maintain consistent line height and spacing.
- Must avoid stylistic variations unless defined in the guidelines.

### Code & Technical Elements
- Code blocks must use the designated monospaced typeface.
- Inline code must follow the same typeface and weight.
- Diagrams must use the same monospaced typeface for labels.

---

## Rendering Rules

### Deterministic Rendering
- Typography must render identically across platforms.
- No browser-specific or OS-specific font substitutions are allowed.
- SVG diagrams must embed or reference the correct typeface.

### Repository Hygiene
- Do not store font files unless explicitly required.
- Do not store temporary or experimental typographic assets.
- All typographic rules must remain centralized in `fonts.md`.

---

## Consistency Requirements
- All contributors must follow the typographic rules defined in this README and in `fonts.md`.
- Any new documentation or UI component must adhere to the established hierarchy.
- Any change to typography must be reflected in this README and in `fonts.md`.

---

## Related Documents
- `assets/branding/guidelines/branding_guidelines.md`
- `assets/branding/colors/palette.md`
- `assets/branding/logo/README.md`
- `assets/branding/badges/README.md`

---

## Notes
This specification ensures deterministic, reproducible typography across all SeedTools repositories.  
All contributors must adhere to these rules to maintain consistency and integrity of SeedTools visual identity.

---
