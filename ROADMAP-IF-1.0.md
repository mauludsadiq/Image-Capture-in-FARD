# ROADMAP-IF-1.0.md

Image Capture in FARD -- Roadmap from MVP to On-Device Capture

Current state (v0.2.0): IF Protocol specified, four apps working
(image_explain, image_diff, image_claim, image_chain), 38 tests, zero
failures. The protocol is correct and complete. The gap is between the
FARD reference implementation and a user taking a picture with every
colour classified at the moment of capture.

This roadmap is honest about prerequisites. Each phase lists what is
needed, what is blocked, and what can be built now.

---

## Phase A -- Full-Resolution Pixel Identity (no blockers)

Current: pixel_digest is computed over a 16px thumbnail sample.
Correct for palette extraction. Wrong for forensic identity.

A.1  Compute pixel_digest at full resolution.
    sips can output a full-resolution BMP. Read it with bmp_read,
    hash the raw RGB bytes before any downsampling. Record the
    full-resolution pixel_digest in the receipt; record the
    thumbnail stride used for palette extraction separately
    (pixel_sample_stride field already exists in the spec).
    The IF-ID is then derived from the full-resolution digest.

A.2  Add image dimensions to receipt correctly.
    Currently width=0 (sips thumbnail strips metadata before FARD
    sees it). Parse the BMP header dimensions from the full-res
    output and record them in the receipt.

A.3  Add a known pixel_digest test vector.
    Once full-resolution hashing is stable, record the canonical
    pixel_digest for a known synthetic image (e.g. the 4x4 colorful
    test PNG) and add it to SPEC-IF-1.0.md Appendix A. This is the
    IF-Protocol equivalent of the CF-ID conformance vectors.

A.4  EXIF extraction via sips/exiftool (macOS only, no new accounts).
    sips -g all <path> returns EXIF as text. Parse the relevant
    fields (timestamp, orientation, device model, GPS) and populate
    the receipt source block. Record them as claims (unverified),
    consistent with SPEC-IF-1.0.md section 4 -- EXIF is a claim,
    not truth.

Deliverable: receipts with correct dimensions, full-resolution
pixel identity, and EXIF claims. IF-ID is now forensically meaningful
for real photos.

---

## Phase B -- Language Ports of IF-Receipt (no blockers)

The CF-ID has eight conformant language ports. IF-receipt needs the
same treatment for the protocol to be independently verifiable.

B.1  Swift port: src/swift/IFReceipt.swift
    Port pixel_digest_canonical, make_receipt, verify_receipt to
    Swift. Uses CryptoKit SHA-256 (same as cfid_swift). This is the
    direct prerequisite for Phase D (iOS app).
    ~200 lines, same pattern as cfid_swift.

B.2  Kotlin port: src/kotlin/IFReceipt.kt
    Same, using java.security.MessageDigest (same as cfid_kotlin).
    Direct prerequisite for Phase D (Android app).

B.3  Go port: src/go/ifreceipt.go
    Same, using crypto/sha256 (same as cfid_go).
    Useful for server-side verification pipelines.

B.4  TypeScript port: src/ts/ifreceipt.ts
    Same, using node:crypto. Prerequisite for browser/extension
    capture (Phase E).

B.5  Conformance suite: conformance/if_vectors.json
    Canonical cross-language test vectors for IF-receipt, same
    structure as Colour in FARD conformance/vectors.json.
    Any implementation that passes all vectors is conformant.

Deliverable: IF-Protocol is independently verifiable in four languages.
A server receiving an image claim can verify it in Go without trusting
the FARD reference implementation.

---

## Phase C -- Perceptual Identity (no blockers)

IF-ID identifies exact pixel content. Two perceptually identical images
with minor JPEG noise will have different pixel_digest values. Phase C
adds a perceptual layer that is complementary to (not a replacement for)
the exact pixel identity.

C.1  Perceptual hash (pHash-style) in FARD.
    Compute a 64-bit discrete cosine transform hash over a 32x32
    LAB thumbnail. Store as perceptual_digest in the receipt.
    Two images with dHash distance < 10 are perceptually similar.

C.2  image_diff: add perceptual similarity score.
    Extend image_diff to report perceptual distance alongside
    byte/pixel/palette comparison.
    "perceptually similar? YES (distance=3)"

C.3  Metamerism risk per image.
    For each palette colour, compute metamerism_index_a (from
    Colour in FARD src/core/multi_illuminant.fard). Report which
    brand colours in the image are at risk of shifting under
    Illuminant A (incandescent). Useful for print/packaging.

C.4  Colour drift across exports.
    Run image_explain on original and JPEG-compressed version.
    Report which palette CF-IDs changed, which stayed stable.
    "CF-7B3F00-EA262463 survived export. CF-CB5236-1EF4B185 drifted
    to CF-CC5438-XXXXXXXX (dE2000=1.2)."

Deliverable: receipts carry both exact and perceptual identity.
image_diff can distinguish pixel-identical, perceptually-similar, and
visually-distinct images as three separate categories.

---

## Phase D -- Native Capture Apps (requires accounts)

Prerequisites: Phase A (full-res identity), Phase B (Swift/Kotlin ports).
External requirements: Apple Developer account, Google Play account.

D.1  iOS app: Image Capture in FARD for iOS.
    AVFoundation capture -> cfid_swift (CF-ID per pixel sample) ->
    IFReceipt.swift (IF-ID at moment of capture) -> receipt saved
    to Files app or shared via standard iOS share sheet.
    The receipt is computed before the image leaves the device.
    Stack: SwiftUI + AVFoundation + cfid_swift + IFReceipt.swift.
    No server required. Receipt is self-contained.

D.2  Android app.
    CameraX capture -> cfid_kotlin -> IFReceipt.kt -> receipt
    saved locally or shared. Same architecture as iOS.

D.3  macOS CLI wrapper.
    A thin Swift/Go CLI that wraps image_explain, image_diff,
    image_claim, image_chain with a native macOS UX (drag-and-drop,
    Quick Look preview, Spotlight metadata). Distributable via
    Homebrew without an App Store account.

D.4  Capture receipt as a sidecar file.
    Every captured image gets a <filename>.ifreceipt.json sidecar,
    containing the full IF-Protocol receipt. Importable into
    Lightroom/Capture One as metadata. Can be embedded in the image
    file itself via the PNG tEXt chunk mechanism from Colour in FARD.

Deliverable: a user takes a picture. At the moment of capture, the
app computes the IF-ID, identifies every dominant colour by its CF-ID,
and writes a tamper-evident receipt. The image and its identity are
inseparable from that point forward.

---

## Phase E -- Browser and Pro Tool Surfaces (requires accounts)

E.1  Chrome DevTools extension.
    Inspect any image on a webpage: right-click -> "Get IF-ID".
    Uses cfid_wasm (already built) + TypeScript IF-receipt port
    (Phase B.4). Shows IF-ID, pixel_digest, CF palette, WCAG
    contrast for each dominant colour.
    Requires: Chrome Web Store developer account.

E.2  Figma plugin extension.
    Extend the existing Colour in FARD Figma plugin to compute
    IF-ID for any image placed in a Figma frame.
    Shows whether an image has drifted from its original IF-ID.

E.3  Lightroom/Capture One plugin.
    On export, write the IF-ID into the image EXIF UserComment
    and generate a sidecar receipt. Any downstream recompression
    will change the byte_digest but not the pixel_digest, making
    the drift immediately visible.

E.4  Webhook / CI integration.
    A GitHub Action or webhook that runs image_diff on every
    image asset committed to a repo, flagging pixel changes vs
    metadata-only changes vs additions. Useful for design systems
    and brand asset management.

---

## Phase F -- Standards and Governance (requires organisations)

F.1  IF Protocol submitted to W3C Community Group.
    SPEC-IF-1.0.md is written in W3C CG format. The submission
    path is the same as CF-Protocol: w3c/SUBMISSION.md documents
    the manual steps. Requires a W3C account and an organisation
    that is not a personal GitHub account.

F.2  Journalism and legal use case documentation.
    A formal document explaining IF-Protocol for newsroom photo
    editors, legal evidence handlers, and insurance adjusters.
    Explains what IF-ID proves, what it does not prove, and how
    to verify a receipt independently.

F.3  Reference verification service.
    A minimal static web service (no database, no accounts) that
    accepts an IF-receipt JSON and returns VALID/INVALID. Built on
    the Go or TypeScript port from Phase B. Hostable on any static
    server. Proves that verification requires no trust in the
    original issuer.

---

## Summary

Phase A -- Full-resolution pixel identity       no blockers
Phase B -- Language ports of IF-receipt         no blockers
Phase C -- Perceptual identity layer            no blockers
Phase D -- Native capture apps                  needs Apple/Google accounts
Phase E -- Browser and pro tool surfaces        needs store accounts
Phase F -- Standards and governance             needs organisations

Everything in Phases A, B, C is buildable now. Phases D, E, F are
correctly identified as distribution and governance problems, not
engineering problems. The engineering prerequisites for D (Swift/Kotlin
IF-receipt ports) will be complete after Phase B.

The single most important next step: Phase A.1 (full-resolution
pixel_digest), because it makes IF-ID forensically meaningful for real
photos rather than thumbnails. Everything else builds on that.
