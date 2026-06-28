# IF Conformance Suite

conformance/if_vectors.json is the canonical, language-neutral source of
test vectors for IF-Protocol-1.0.0. Any implementation -- existing or
future -- is conformant if and only if it reproduces every value in that
file exactly.

## Vectors

### pixel_digest_vectors

Three canonical pixel sequences with known SHA-256 digests:

1. The 4x4 colorful test image (16 pixels, cycling colours) -- the
   primary conformance target, used in SPEC-IF-1.0.md Appendix A.1.
2. A single white pixel [255,255,255].
3. A single black pixel [0,0,0].

Each entry gives the full pixel_digest and the PIXELHASH12 (first 12
uppercase hex chars), which is the value embedded in the IF-ID.

### hash12_vectors

Input SHA-256 digests with expected 12-char uppercase extraction results.

### if_id_format

The IF-ID format invariant: always 28 characters, IF-XXXXXXXXXXXX-XXXXXXXXXXXX.

### receipt_invariants

Seven properties that MUST hold for any conformant makeReceipt
implementation (determinism, tamper-detection, null parent_if_id for
originals, etc.).

### canonical_preimage_format

The exact preimage format for pixel_digest:
  "r0,g0,b0,r1,g1,b1,...,rN,gN,bN,"
(comma-separated 8-bit integers, trailing comma, UTF-8 encoded).

## Status

| Implementation | Vectors covered | How verified |
|---|---|---|
| FARD (reference) | pixel_digest x3, hash12, receipt_invariants | tests/test_if_id.fard, tests/test_image_receipt.fard |
| Swift (IFReceipt) | pixel_digest (colorful), hash12, receipt_invariants | IFReceiptTests.swift (9 tests) |
| Kotlin (IFReceipt) | pixel_digest (colorful), hash12, receipt_invariants | IFReceiptTest.kt (9 tests) |
| Go (IFReceipt) | pixel_digest (colorful), hash12, receipt_invariants | ifreceipt_test.go (9 tests) |
| TypeScript (IFReceipt) | pixel_digest (colorful), hash12, receipt_invariants | index.test.mjs (9 tests) |

The single most important vector is pixel_digest_vectors[0] (the 4x4
colorful image). All five implementations produce the same value:

  sha256:2a40854db5a950bbdc8d921620a2ee8074fb8b950102150b6be7b10990e1ddb6

## Using if_vectors.json from a new implementation

1. For each entry in pixel_digest_vectors, compute pixel_digest from
   the pixels array using the canonical preimage format and assert it
   equals the expected value.
2. For each entry in hash12_vectors, extract the first 12 uppercase
   hex chars from the digest and assert equality.
3. Build a receipt and verify receipt_invariants hold.

A conformant implementation passes all of (1)-(2) at minimum.

## Versioning

suite_version (IF-Conformance-1.0.0) is independent of
spec_version (IF-Protocol-1.0.0). New vectors may be appended as new
features are added, but existing vectors are permanent.
