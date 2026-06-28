# Image Capture in FARD

Tamper-evident image identity using IF-Protocol-1.0.0 and PERC-1.0.
Written in FARD with four language ports and a live MacBook capture bridge.

## What this is

Every image gets two identities:

**IF-ID** (cryptographic) -- `IF-XXXXXXXXXXXX-XXXXXXXXXXXX`
Derived from the pixel sequence via SHA-256. One changed pixel changes
it completely. Used for tamper detection and chain-of-custody.

**PERC-ID** (perceptual) -- `PERC-XXXXXXXXXXXXXXXX-XXXXXXXX`
Derived from a 64-bit dHash and a colour histogram sketch. Stable across
JPEG re-encoding, moderate resize, and brightness changes. Used for
near-duplicate detection (Hamming distance on dHash).

Together they answer two different questions:
- "Is this the exact same file?" -> compare IF-IDs
- "Is this the same image?" -> compare PERC-IDs (Hamming <= 5 on dHash)

## Specifications

- `SPEC-IF-1.0.md`   -- IF-Protocol-1.0.0: pixel_digest, receipt, IF-ID
- `SPEC-PERC-1.0.md` -- PERC-1.0: dHash, histogram sketch, PERC-ID
- `conformance/if_vectors.json` -- canonical cross-language test vectors
- `conformance/CONFORMANCE.md`  -- conformance guide

## Repository structure

   src/core/
     if_id.fard           -- pixel_digest_canonical, make_if_id, hash12
     image_receipt.fard   -- make_receipt, verify_receipt, build_palette
     image_read.fard      -- read_image_pixels, read_image_full (via sips)
     exif_claims.fard     -- extract_exif_claims (via sips -g all)
     perc_id.fard         -- dhash16, hist_sketch, make_perc_id, hamming
     vendor/              -- patched copies of cf_id, rgb_lab, kmeans etc.

   apps/
     image_explain.fard      -- full identity profile for any image
     image_diff.fard         -- compare two images (byte/pixel/IF-ID/palette)
     image_claim.fard        -- create/verify tamper-evident IF-Claims
     image_chain.fard        -- init/add/verify/show edit chains
     capture_explain.fard    -- IF-ID + PERC-ID for captured images,
                                two-image comparison with Hamming similarity

   tools/
     capture_macbook.sh      -- MacBook camera capture bridge (imagesnap)

   tests/                    -- 58 FARD tests, 0 failures

   cfid_swift_if/            -- Swift port (Phase B.1), 9 tests
   cfid_kotlin_if/           -- Kotlin port (Phase B.2), 9 tests
   cfid_go_if/               -- Go port (Phase B.3), 9 tests
   cfid_ts_if/               -- TypeScript port (Phase B.4), 9 tests

   conformance/
     if_vectors.json         -- canonical pixel_digest + hash12 vectors
     CONFORMANCE.md          -- conformance guide for new implementations

## Capture bridge (Phase D)

Requires imagesnap:

   brew install imagesnap

Single capture with full identity profile:

   bash tools/capture_macbook.sh

Or manually:

   imagesnap -w 1.5 out/captures/capture.jpg
   sips -Z 320 out/captures/capture.jpg --out out/captures/capture_small.jpg
   fardrun run --program apps/capture_explain.fard --out out/explain -- out/captures/capture_small.jpg 4

Two-image comparison (PERC-ID similarity):

   fardrun run --program apps/capture_explain.fard --out out/compare -- image_a.jpg image_b.jpg 4

Example output from two consecutive MacBook captures of the same scene:

   IF-ID:    IF-90970EA1914A-D501D653C694   (capture 1)
   IF-ID:    IF-E94B5F1CDF32-7666419C6747   (capture 2)
   PERC-ID:  PERC-0D4D78691838312A-0304080C (capture 1)
   PERC-ID:  PERC-808088C85CD1FA32-0304080C (capture 2)
   hist sketch: identical (0304080C) -- same dominant colour palette
   dHash Hamming: 25 -- DIFFERENT (subject moved between captures)

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
| D | Local camera capture bridge (MacBook) | Complete |
| E | Capture validation experiments (PERC-ID stability) | Not started |
| F | Live capture protocol (frame chains, video keyframes) | Not started |

## Stats

- 2,559 lines of FARD
- 58 FARD tests, 36 language port tests
- 94 total tests, 0 failures
- 5 implementations (FARD + Swift + Kotlin + Go + TypeScript)
- Live MacBook camera capture bridge working end-to-end
