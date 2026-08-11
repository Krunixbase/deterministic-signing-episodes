# SeedTools Branding — Guidelines (Technical Specification)

## Purpose
This document defines the authoritative branding guidelines for the SeedTools ecosystem.  
It establishes deterministic rules for logo usage, color application, typography, spacing, and asset consistency across all repositories and visual materials.

All detailed rules are stored in `branding_guidelines.md`, which acts as the single source of truth.

---

## Directory Structure
```
assets/branding/guidelines
│
└── branding_guidelines.md     # Authoritative branding rules
```

---

## Guidelines Overview

SeedTools branding guidelines ensure:

- **deterministic visual identity** across all repositories,
- **consistent asset usage** (logo, badges, colors, typography),
- **strict separation** between authoritative sources and generated artifacts,
- **reproducible rendering** across platforms and environments.

These guidelines apply to:

- documentation,
- UI components,
- diagrams,
- marketing materials,
- repository READMEs,
- technical specifications.

---

## Core Principles

### 1. Determinism
All branding assets must be:

- reproducible,
- consistent,
- free from manual raster edits,
- derived from authoritative sources (SVG, palette, typography rules).

### 2. Minimalism
SeedTools branding prioritizes:

- clarity,
- readability,
- functional design,
- absence of decorative or non-technical elements.

### 3. Modularity
Branding components (logo, badges, colors, typography) must be:

- interchangeable,
- reusable across repositories,
- structured in isolated directories.

### 4. Consistency
All contributors must follow the same rules for:

- spacing,
- color usage,
- logo placement,
- variant selection,
- typography hierarchy.

---

## Usage Rules

### Logo Usage
- Use only SVG sources from `assets/branding/logo/svg`.
- PNG exports must be generated deterministically via Inkscape CLI.
- Do not distort, stretch, or modify logo proportions.
- Use correct variants (mono, outline, solid, premium) based on context.

### Badge Usage
- Badges must remain in SVG format.
- Use dark/light variants depending on background.
- Do not introduce new badge colors outside the palette.

### Color Usage
- All colors must come from `palette.md`.
- No inline or arbitrary colors may be used.
- Diagrams and SVG assets must reference palette colors.

### Typography Usage
- Follow hierarchy defined in `fonts.md`.
- Use designated typefaces for headings, body text, and code.
- Do not introduce additional fonts.

### Spacing & Layout
- Maintain consistent padding around logos and badges.
- Follow spacing rules defined in `branding_guidelines.md`.
- Avoid cramped or overly loose layouts.

---

## Deterministic Rendering Rules

### Authoritative Sources
- SVG files are authoritative.
- PNG files are non-authoritative artifacts.
- No raster editors may be used for branding assets.

### Rendering Requirements
- All exports must use Inkscape CLI.
- No alternative rendering tools are allowed.
- SVG diagrams must embed or reference correct fonts and colors.

### Repository Hygiene
- Do not store temporary or experimental assets.
- Do not store redundant PNG sizes.
- Maintain strict separation between directories.

---

## Consistency Requirements
- All contributors must follow the rules defined in this README and in `branding_guidelines.md`.
- Any new branding asset must adhere to established principles.
- Any change to branding rules must be reflected in this README.

---

## Related Documents
- `assets/branding/logo/README.md`
- `assets/branding/badges/README.md`
- `assets/branding/colors/README.md`
- `assets/branding/typography/README.md`

---

## Notes
This specification ensures deterministic, reproducible branding across all SeedTools repositories.  
All contributors must adhere to these guidelines to maintain consistency and integrity of SeedTools visual identity.

---
