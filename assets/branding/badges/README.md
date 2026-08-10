# SeedTools Branding — Badges (Technical Specification)

## Purpose
This document defines the structure, usage rules, and deterministic requirements for all SeedTools branding badges located in `assets/branding/badges`.  
Badges are used across documentation, repositories, and UI components to visually represent deterministic security, SeedTools identity, and project modules.

---

## Directory Structure
```
assets/branding/badges
│
├── deterministic_badge.svg
├── deterministic_badge_dark.svg
└── deterministic_badge_light.svg
```

---

## Badge Overview

### Deterministic Badge (Primary)
- File: `deterministic_badge.svg`
- Purpose: Represents deterministic security and reproducible cryptographic workflows.
- Usage: Documentation headers, repository READMEs, technical diagrams.

### Dark Variant
- File: `deterministic_badge_dark.svg`
- Purpose: Optimized for dark UI themes or dark backgrounds.
- Usage: Dark-mode documentation, dark UI components.

### Light Variant
- File: `deterministic_badge_light.svg`
- Purpose: Optimized for light UI themes or light backgrounds.
- Usage: Light-mode documentation, light UI components.

---

## Deterministic Rules

### Authoritative Format
- **All badges must remain in SVG format.**
- **No PNG or rasterized versions may be stored in the repository.**
- SVG ensures:
  - resolution independence,
  - deterministic rendering,
  - consistent scaling across platforms.

### Modification Rules
- Badges must not be manually edited in raster editors.
- Any modification must be performed directly on the SVG source.
- Variants must preserve:
  - proportions,
  - padding,
  - stroke thickness,
  - color consistency.

### Repository Hygiene
- Do not store temporary or experimental badge files.
- Do not introduce additional color variants unless defined in `palette.md`.
- Maintain strict separation between badges and logo assets.

---

## Usage Guidelines

### General Usage
- Badges should be used to visually indicate deterministic security or SeedTools identity.
- Badges must not be stretched, skewed, or distorted.
- Minimum recommended display size: **32×32 px**.

### Background Compatibility
- Use the **primary badge** on neutral or mixed backgrounds.
- Use the **dark variant** on dark backgrounds.
- Use the **light variant** on light backgrounds.

### Documentation Integration
Badges may be used in:
- repository READMEs,
- module documentation,
- architecture diagrams,
- security specifications.

Badges must not replace the main SeedTools logo.

---

## Consistency Requirements
- All contributors must use the existing badge variants.
- New badge variants require approval and must follow SeedTools branding guidelines.
- Any change to badge design must be reflected in this README.

---

## Related Documents
- `assets/branding/logo/README.md`
- `assets/branding/guidelines/branding_guidelines.md`
- `assets/branding/colors/palette.md`
- `assets/branding/typography/fonts.md`

---

## Notes
This specification ensures deterministic, reproducible badge usage across all SeedTools repositories.  
All contributors must adhere to these rules to maintain consistency and integrity of SeedTools branding.

---
