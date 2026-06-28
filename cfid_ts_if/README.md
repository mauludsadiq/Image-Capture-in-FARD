# IFReceipt (TypeScript)

IF-Protocol-1.0.0 image identity in TypeScript.

Zero runtime dependencies (node:crypto for SHA-256). This is the fourth
language port of IF-Protocol, after Swift, Kotlin, and Go, conformant
with SPEC-IF-1.0.md.

## Build and test

    npm install
    npx tsc
    node --test test/index.test.mjs

9/9 tests pass including the canonical SPEC-IF-1.0.md Appendix A.1
pixel_digest vector.

## Status

Part of Image Capture in FARD, Roadmap Phase B.4. Useful for
browser/extension-based capture surfaces (Phase E).
