# Image Capture in FARD

Tamper-evident image identity using the IF-Protocol-1.0.0 and PERC-1.0
perceptual identity protocol. Written in FARD with four language ports.

## What this is

Every image gets two identities:

**IF-ID** (cryptographic) -- `IF-XXXXXXXXXXXX-XXXXXXXXXXXX`
Derived from the exact pixel sequence via SHA-256. One changed pixel
changes it completely. Used for tamper detection and chain-of-custody.

**PERC-ID** (perceptual) -- `PERC-XXXXXXXXXXXXXXXX-XXXXXXXX`
Derived from a dHash (64-bit horizontal gradient) and a colour histogram
sketch. Stable across JPEG re-encoding, moderate resize, and brightness
changes. Used for near-duplicate detection (Hamming distance on dHash).

Together they answer two different questions:
- "Is this the exact same file?" -> compare IF-IDs
- "Is this the same image?" -> compare PERC-IDs (Hamming <= 5 on dHash)

## Specifications

- `SPEC-IF-1.0.md` -- IF-Protocol-1.0.0: pixel_digest, receipt, IF-ID
- `SPEC-PERC-1.0.md` -- PERC-1.0: dHash, histogram sketch, PERC-ID
- `conformance/if_vectors.json` -- canonical cross-language test vectors
- `conformance/CONFORMANCE.md` -- conformance guide

## Repository structure

   src/core/
     if_id.fard           -- pixel_digest_canonical, make_if_id, hash12
     image_receipt.fard   -- make_receipt, verify_receipt, build_palette
     image_read.fard      -- read_image_pixels, read_image_full (via sips)
     exif_claims.fard     -- extract_exif_claims (via sips -g all)
     perc_id.fard         -- dhash16, hist_sketch, make_perc_id, hamming
     vendor/              -- patched copies of cf_id, rgb_lab, kmeans etc.

   apps/
     image_explain.fard   -- full identity profile for an image
     image_diff.fard      -- compare two images (byte/pixel/IF-ID/palette)
     image_claim.fard     -- create/verify tamper-evident IF-Claims
     image_chain.fard     -- init/add/verify/show edit chains

   tests/                 -- 58 FARD tests, 0 failures

   cfid_swift_if/         -- Swift port (Phase B.1), 9 tests
   cfid_kotlin_if/        -- Kotlin port (Phase B.2), 9 tests
   cfid_go_if/            -- Go port (Phase B.3), 9 tests
   cfid_ts_if/            -- TypeScript port (Phase B.4), 9 tests

   conformance/
     if_vectors.json      -- canonical pixel_digest + hash12 vectors
     CONFORMANCE.md       -- conformance guide for new implementations

## Running the apps

   # Full identity profile
   fardrun run --program apps/image_explain.fard --out out/explain -- photo.jpg 4

   # Compare two images
   fardrun run --program apps/image_diff.fard --out out/diff -- a.jpg b.jpg

   # Create a tamper-evident claim
   fardrun run --program apps/image_claim.fard --out out/claim -- create photo.jpg

   # Verify a claim
   fardrun run --program apps/image_claim.fard --out out/claim -- verify photo.jpg claim.json

   # Start an edit chain
   fardrun run --program apps/image_chain.fard --out out/chain -- init photo.jpg

## Running the tests

   # FARD reference implementation (58 tests)
   for f in tests/test_*.fard; do fardrun test --program $f; done

   # Swift port
   cd cfid_swift_if && swift test

   # Kotlin port
   cd cfid_kotlin_if
   kotlinc src/main/kotlin/com/imagecapture/ifreceipt/IFReceipt.kt \
           src/test/kotlin/com/imagecapture/ifreceipt/IFReceiptTest.kt \
           -include-runtime -d test.jar
   java -cp test.jar com.imagecapture.ifreceipt.IFReceiptTestKt

   # Go port
   cd cfid_go_if && go test ./...

   # TypeScript port
   cd cfid_ts_if && npm install && npx tsc && node --test test/index.test.mjs

## Conformance

All five implementations agree on the canonical pixel_digest vector
from SPEC-IF-1.0.md Appendix A.1 (4x4 colorful test image):

   sha256:2a40854db5a950bbdc8d921620a2ee8074fb8b950102150b6be7b10990e1ddb6
   PIXELHASH12: 2A40854DB5A9

See `conformance/if_vectors.json` for the full conformance suite.

## Status

| Phase | Description | Status |
|---|---|---|
| A | Full-res pixel identity, EXIF claims, conformance vectors | Complete |
| B | Swift, Kotlin, Go, TypeScript ports + conformance suite | Complete |
| C | Perceptual identity (dHash + histogram sketch) | Complete |
| D | Native apps (iOS D.1, Android D.2) | Not started (requires Apple/Google accounts) |
| E | Browser/extension surfaces | Not started (requires browser store accounts) |
| F | W3C submission, governance | Not started (requires W3C membership) |

## Stats

- 2,559 lines of FARD
- 58 FARD tests, 36 language port tests
- 94 total tests, 0 failures
- 5 implementations (FARD + Swift + Kotlin + Go + TypeScript)
