# SeedTools Branding — Logo Assets (Technical Specification)

## Purpose
This document defines the deterministic structure, generation workflow, and operational rules for all logo assets located in `assets/branding/logo`.  
Its goal is to ensure reproducible rendering, consistent variant management, and strict separation between source assets (SVG) and generated artifacts (PNG).

---

## Directory Structure
```
assets/branding/logo
│
├── png/                # Generated PNG artifacts (non-authoritative)
│   ├── deterministic_logo_220.png
│   ├── deterministic_logo_512.png
│   └── ...
│
├── svg/                # Authoritative SVG sources (single source of truth)
│   ├── deterministic_logo.svg
│   ├── deterministic_logo_mono.svg
│   ├── deterministic_logo_outline.svg
│   └── ...
│
└── variants/           # Structured SVG variants
    ├── mono/
    ├── outline/
    ├── premium/
    └── solid/
```

### Source of Truth
- **SVG files are authoritative.**
- **PNG files are derived artifacts and must never be edited manually.**

---

## Variant Specification

### Master Logo
Primary deterministic logo used for all conversions.
- `svg/deterministic_logo.svg`

### Mono Variant
Single-color variant for high-contrast or minimal UI contexts.
- `variants/mono/deterministic_logo_mono.svg`

### Outline Variant
Stroke-only variant for diagrams or lightweight UI.
- `variants/outline/deterministic_logo_outline.svg`

### Solid Variant
Filled variant for strong visual presence.
- `variants/solid/deterministic_logo_solid.svg`

### Premium Variant
Enhanced variant with gradient or premium visual effects.
- `variants/premium/deterministic_logo_premium.svg`

---

## PNG Generation Workflow (Deterministic)

### Requirements
- Inkscape ≥ 1.4
- Inkscape available in system PATH
- Execution from PowerShell or compatible shell

### Working Directory
All export commands must be executed from:

```
assets/branding/logo/svg
```

### Export Commands

#### 220×220 PNG
```
inkscape ./deterministic_logo.svg \
  --export-type="png" \
  --export-filename="../png/deterministic_logo_220.png" \
  --export-width=220 --export-height=220
```

#### 512×512 PNG
```
inkscape ./deterministic_logo.svg \
  --export-type="png" \
  --export-filename="../png/deterministic_logo_512.png" \
  --export-width=512 --export-height=512
```

### Rendering Rules
1. All exports must use **Inkscape CLI** to ensure deterministic rendering.
2. PNG files must not be manually modified or committed unless generated via the above commands.
3. Temporary or experimental PNGs must not be stored in the repository.
4. Variant exports must reference their respective SVG variant files.

---

## Operational Rules

### Authoritative Assets
- Only SVG files are considered authoritative.
- Any modification to branding must be performed on SVG sources.

### Repository Hygiene
- Do not store redundant PNG sizes.
- Do not store intermediate or test renders.
- Maintain strict separation between `svg/`, `png/`, and `variants/`.

### Consistency Requirements
- All contributors must follow the same export commands.
- Any new variant must be placed in the correct subdirectory under `variants/`.
- Any change to the branding pipeline must be reflected in this README.

---

## Related Documents
- `assets/branding/guidelines/branding_guidelines.md`
- `assets/branding/colors/palette.md`
- `assets/branding/typography/fonts.md`

---

## Notes
This specification ensures deterministic, reproducible branding across all SeedTools repositories.  
All contributors must adhere to this workflow to maintain consistency and integrity of visual assets.

---
