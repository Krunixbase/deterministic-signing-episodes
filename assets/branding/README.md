<<<<<<< HEAD
# SeedTools Branding — Deterministic Specification

## Purpose
The `assets/branding` module defines the **deterministic structure**, **operational rules**, and **asset organization** for all SeedTools branding materials.  
It ensures consistent usage of colors, typography, badges, guidelines, and logo assets across every SeedTools repository.

Branding in SeedTools follows the same principles as the software itself:

- determinism  
- reproducibility  
- auditability  
- strict separation of authoritative vs. derived assets  
=======
# SeedTools Branding — Technical Specification

## Purpose
This document defines the deterministic structure, operational rules, and asset organization for the entire SeedTools branding module located in `assets/branding`.  
It ensures consistent usage of colors, typography, badges, guidelines, and logo assets across all SeedTools repositories.
>>>>>>> 8eb8e34 (Refactor: migrate core modules to src/, update structure, add Cargo.lock)

---

## Directory Structure
```
assets/branding
│
├── badges/             # Deterministic badges (SVG only)
│   ├── deterministic_badge.svg
│   ├── deterministic_badge_dark.svg
│   └── deterministic_badge_light.svg
│
├── colors/             # Color palette definitions
│   └── palette.md
│
├── guidelines/         # Branding rules and usage guidelines
│   └── branding_guidelines.md
│
├── logo/               # Logo assets (SVG source + deterministic PNG exports)
│   ├── png/
│   ├── svg/
│   └── variants/
│
└── typography/         # Font usage and typographic rules
    └── fonts.md
```

---

## Module Overview

### 1. Badges
<<<<<<< HEAD
The `badges/` directory contains deterministic SVG badges used across documentation, repositories, and UI components.  
All badges must remain **SVG-only** to preserve resolution independence and deterministic rendering.

### 2. Colors
The `colors/` directory defines the authoritative SeedTools color palette.  
All UI components, documentation, and branding assets must reference this palette.

### 3. Guidelines
The `guidelines/` directory contains the official branding rules, including:
=======
The `badges/` directory contains deterministic SVG badges used for documentation, repositories, and UI components.  
All badges must remain in SVG format to ensure resolution independence.

### 2. Colors
The `colors/` directory defines the SeedTools color palette.  
The palette is authoritative and must be referenced by all UI, documentation, and branding components.

### 3. Guidelines
The `guidelines/` directory contains the official branding rules.  
These rules define:
>>>>>>> 8eb8e34 (Refactor: migrate core modules to src/, update structure, add Cargo.lock)

- color usage  
- spacing  
- logo placement  
- variant selection  
- typography rules  
- prohibited modifications  

<<<<<<< HEAD
These rules are mandatory for all contributors.
=======
All contributors must follow these guidelines.
>>>>>>> 8eb8e34 (Refactor: migrate core modules to src/, update structure, add Cargo.lock)

### 4. Logo
The `logo/` directory contains the full deterministic logo pipeline:

- authoritative SVG sources  
- structured SVG variants  
<<<<<<< HEAD
- deterministic PNG exports (generated via Inkscape CLI)

See `logo/README.md` for the complete technical specification.
=======
- deterministic PNG exports  

See `logo/README.md` for the full technical specification.
>>>>>>> 8eb8e34 (Refactor: migrate core modules to src/, update structure, add Cargo.lock)

### 5. Typography
The `typography/` directory defines font usage rules and typographic constraints.  
Fonts must remain consistent across all SeedTools materials.

---

## Deterministic Rules

### Authoritative Assets
<<<<<<< HEAD
- **SVG files are authoritative** for badges, logos, and variants.  
- **PNG files are non-authoritative** and must be generated deterministically from SVG sources.

### Rendering Requirements
- All raster exports must use **Inkscape CLI**.  
- No manual edits to PNG files are allowed.  
- No alternative rendering tools may be used.

### Repository Hygiene
- Do not store temporary or experimental assets.  
- Do not store redundant PNG sizes.  
- Maintain strict separation between directories (`badges`, `colors`, `guidelines`, `logo`, `typography`).  

### Consistency Requirements
- All contributors must follow the same deterministic workflow.  
- Any new branding asset must be placed in the correct directory.  
=======
- **SVG files are authoritative** for badges, logos, and variants.
- **PNG files are non-authoritative** and must be generated deterministically.

### Rendering Requirements
- All raster exports must use **Inkscape CLI**.
- No manual edits to PNG files are allowed.
- No alternative rendering tools may be used.

### Repository Hygiene
- Do not store temporary or experimental assets.
- Do not store redundant PNG sizes.
- Maintain strict separation between directories (`badges`, `colors`, `guidelines`, `logo`, `typography`).

### Consistency Requirements
- All contributors must follow the same deterministic workflow.
- Any new branding asset must be placed in the correct directory.
>>>>>>> 8eb8e34 (Refactor: migrate core modules to src/, update structure, add Cargo.lock)
- Any change to the branding pipeline must be reflected in this README and in `logo/README.md`.

---

<<<<<<< HEAD
## Deterministic Pipeline

All branding assets follow a deterministic pipeline:

1. **Authoritative SVG creation**  
2. **Variant generation (SVG only)**  
3. **Deterministic PNG export via Inkscape CLI**  
4. **Placement in correct directory**  
5. **Documentation update**  
6. **Audit of changes (commit-level)**  

This ensures reproducibility across machines, reviewers, and repositories.

---

## Versioning & Change Control

- All branding changes must be atomic and documented.  
- Any modification to authoritative SVG assets requires:  
  - updated PNG exports  
  - updated guidelines (if applicable)  
  - updated version notes  
- No contributor may introduce non-deterministic assets or workflows.

---

## Integration Across Repositories

The `assets/branding` module is designed to be reused across:

- deterministic-signing-episodes  
- SeedTools core repositories  
- SeedTools UI components  
- documentation sites  
- external integrations  

All repositories referencing SeedTools branding must treat this module as the authoritative source.

---

## Related Documents
- `assets/branding/logo/README.md`  
- `assets/branding/guidelines/branding_guidelines.md`  
- `assets/branding/colors/palette.md`  
=======
## Related Documents
- `assets/branding/logo/README.md`
- `assets/branding/guidelines/branding_guidelines.md`
- `assets/branding/colors/palette.md`
>>>>>>> 8eb8e34 (Refactor: migrate core modules to src/, update structure, add Cargo.lock)
- `assets/branding/typography/fonts.md`

---

## Notes
This specification ensures deterministic, reproducible branding across all SeedTools repositories.  
All contributors must adhere to this structure and workflow to maintain consistency and integrity of visual assets.

---
