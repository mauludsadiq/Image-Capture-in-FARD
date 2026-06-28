# Image Capture in FARD

**Deterministic image identity and receipted capture.**

Every image has a permanent, verifiable identity: an IF-ID, computed from
its decoded pixels, not its file bytes. JPEG recompression changes bytes
but not pixels; metadata edits change receipts but not pixel identity.
IF Protocol separates these four identities:

    byte_digest    -- exact file identity (SHA-256 of raw file bytes)
    pixel_digest   -- decoded pixel identity (SHA-256 of canonical RGB)
    IF-ID          -- composite identity (pixel + receipt hash)
    CF palette     -- every dominant colour identified by its CF-ID

Built on FARD and CF-Protocol-2.0.0 (Colour in FARD).

## Usage

    fardrun run --program apps/image_explain.fard --out out/run -- photo.jpg

    IF-ID:         IF-8EDC2910BB67-4B902CEE5506
    pixel_digest:  sha256:8edc2910bb67...
    byte_digest:   sha256:e358d4fd...
    format:        png
    colorspace:    sRGB
    dominant palette (k=5):
      1. #cb5236  CF-CB5236-1EF4B185  contrast/white=4.38
      ...

## Specification

SPEC-IF-1.0.md defines IF-ID, canonical pixel sequences, image receipts,
and edit chains.

## Tests

    fardrun test --program tests/test_if_id.fard
    fardrun test --program tests/test_image_receipt.fard

## Status

v0.1.0 -- IF-ID, image receipts, image_explain. image_diff and
edit chains (image_chain) are next.
