# Deterministic Signing Episodes — Branding Guidelines

These guidelines define the deterministic, minimal, and reproducible visual identity
of the project. All branding elements follow strict geometric rules, consistent
color usage, and a cryptographic design language aligned with the Krunixbase
ecosystem and SeedTools.

---

## 1. Core Principles

### **Determinism**
All visual elements must be reproducible, predictable, and free from randomness.
Shapes, spacing, and colors follow strict rules.

### **Minimalism**
No decorative elements, gradients (except premium variants), or unnecessary
complexity. Geometry and clarity come first.

### **Consistency**
Logos, badges, colors, and typography must remain uniform across all repositories,
documentation, and UI components.

---

## 2. Logo Usage

### **Primary Logo**
- Use the official deterministic hexagon‑triangle icon.
- SVG must remain monochrome (white + deterministic green).
- Do not alter proportions, stroke width, or node positions.

### **Allowed Backgrounds**
- Black (`#000000`)
- White (`#FFFFFF`)
- Dark Grey (`#111111`)
- Light Grey (`#F5F5F5`)

### **Forbidden Modifications**
- No rotations
- No color changes outside the palette
- No added shadows or effects
- No gradients (except premium variant)
- No distortion or stretching

---

## 3. Badge Usage

### **Available Variants**
- Deterministic Badge (default)
- Dark Badge
- Light Badge
- Premium Badge (optional)

### **Rules**
- Badge text must always be “Deterministic”.
- Icon geometry must remain identical to the main logo.
- Background colors must follow the palette.
- Do not add additional text or icons.

---

## 4. Color Guidelines

Use only colors defined in `colors/palette.md`.

### **Primary**
- Black `#000000`
- White `#FFFFFF`

### **Accent**
- Deterministic Green `#00FF88`
- Deterministic Green Light `#00AA55`

### **Neutral**
- Dark Grey `#111111`
- Light Grey `#F5F5F5`

### **Rules**
- Accent green is used for deterministic markers (nodes, connections).
- Light green is used only on light backgrounds.
- No additional colors may be introduced.

---

## 5. Typography

Use fonts defined in `typography/fonts.md`.

### **Primary Font**
- Inter (UI, documentation, badges)

### **Technical Font**
- JetBrains Mono (code, CLI, API examples)

### **Rules**
- No decorative fonts.
- No mixing multiple sans‑serif families.
- Maintain consistent spacing and line height.

---

## 6. File Structure Requirements

Branding assets must follow the deterministic folder structure:

```
assets/
└── branding/
    ├── logo/
    ├── badges/
    ├── colors/
    ├── typography/
    └── guidelines/
```

No additional folders may be added inside `branding/`.

---

## 7. Premium Variant Rules (Optional)

Premium variants may use:
- Minimal gradients
- Slight glow accents
- High‑resolution PNG exports

Premium variants **must not** replace the primary deterministic versions.

---

## 8. Deterministic Compliance Checklist

Before publishing any branding asset, verify:

- Geometry is consistent  
- Colors follow the palette  
- Typography follows the rules  
- No randomness or decorative elements  
- SVG is clean, minimal, and reproducible  
- File is placed in the correct folder  

If all conditions are met, the asset is deterministic.

---

## 9. Licensing

All branding assets are released under the MIT License unless otherwise specified.

---

## 10. Purpose

These guidelines ensure that every visual element of the project reflects its
core philosophy: **deterministic, reproducible, cryptographic precision**.

---
