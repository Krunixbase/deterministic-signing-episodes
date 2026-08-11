# SeedTools Branding — Technical Specification

## Purpose
This document defines the deterministic structure, operational rules, and asset organization for the entire SeedTools branding module located in `assets/branding`.  
It ensures consistent usage of colors, typography, badges, guidelines, and logo assets across all SeedTools repositories.

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
The `badges/` directory contains deterministic SVG badges used for documentation, repositories, and UI components.  
All badges must remain in SVG format to ensure resolution independence.

### 2. Colors
The `colors/` directory defines the SeedTools color palette.  
The palette is authoritative and must be referenced by all UI, documentation, and branding components.

### 3. Guidelines
The `guidelines/` directory contains the official branding rules.  
These rules define:

- color usage  
- spacing  
- logo placement  
- variant selection  
- typography rules  
- prohibited modifications  

All contributors must follow these guidelines.

### 4. Logo
The `logo/` directory contains the full deterministic logo pipeline:

- authoritative SVG sources  
- structured SVG variants  
- deterministic PNG exports  

See `logo/README.md` for the full technical specification.

### 5. Typography
The `typography/` directory defines font usage rules and typographic constraints.  
Fonts must remain consistent across all SeedTools materials.

---

## Deterministic Rules

### Authoritative Assets
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
- Any change to the branding pipeline must be reflected in this README and in `logo/README.md`.

---

## Related Documents
- `assets/branding/logo/README.md`
- `assets/branding/guidelines/branding_guidelines.md`
- `assets/branding/colors/palette.md`
- `assets/branding/typography/fonts.md`

---

## Notes
This specification ensures deterministic, reproducible branding across all SeedTools repositories.  
All contributors must adhere to this structure and workflow to maintain consistency and integrity of visual assets.

---
