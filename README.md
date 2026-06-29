# Image Capture in FARD

Tamper-evident image identity using IF-Protocol-1.0.0 and PERC-1.0.
Written in FARD with four language ports and a live MacBook capture bridge.

## What this is

Every image gets two identities:

**IF-ID** (cryptographic) -- IF-XXXXXXXXXXXX-XXXXXXXXXXXX
Derived from the pixel sequence via SHA-256. One changed pixel changes
it completely. Used for tamper detection and chain-of-custody.

**PERC-ID** (perceptual) -- PERC-XXXXXXXXXXXXXXXX-XXXXXXXX
Derived from a 64-bit dHash and a colour histogram sketch. Stable across
JPEG re-encoding, moderate resize, and brightness changes. Used for
near-duplicate detection (Hamming distance on dHash, threshold <= 5).

Together they answer two different questions:
- Is this the exact same file? -> compare IF-IDs
- Is this the same image? -> compare PERC-IDs

## Specifications

- SPEC-IF-1.0.md   -- IF-Protocol-1.0.0: pixel_digest, receipt, IF-ID
- SPEC-PERC-1.0.md -- PERC-1.0: dHash, histogram sketch, PERC-ID
- conformance/if_vectors.json -- canonical cross-language test vectors
- conformance/CONFORMANCE.md  -- conformance guide
- C2PA_COMPARISON.md          -- how IF-Protocol relates to C2PA/Content Credentials

## Repository structure

    src/core/
      if_id.fard           -- pixel_digest_canonical, make_if_id, hash12
      image_receipt.fard   -- make_receipt, verify_receipt, build_palette
      image_read.fard      -- read_image_pixels via sips + BMP parser
      exif_claims.fard     -- extract_exif_claims via sips -g all
      perc_id.fard         -- dhash16, hist_sketch, make_perc_id, hamming
      vendor/              -- patched copies of cf_id, rgb_lab, kmeans etc.

    apps/
      image_explain.fard      -- full identity profile for any image
      image_diff.fard         -- compare two images (byte/pixel/IF-ID/palette)
      image_claim.fard        -- create/verify tamper-evident IF-Claims
      image_chain.fard        -- init/add/verify/show edit chains
      capture_explain.fard    -- IF-ID + PERC-ID for a captured image;
                                 two-image comparison with Hamming similarity
      capture_session.fard    -- live capture session: start/add/show/verify
                                 tamper-evident frame chain with PERC-ID and
                                 Hamming distance at each step
      phase_e_stability.fard  -- E.1: dHash stability across still captures
      phase_e2_lighting.fard  -- E.2: lighting change experiment
      phase_e3_reencode.fard  -- E.3: JPEG re-encode stability
      phase_e4_movement.fard  -- E.4: movement inflection test

    tools/
      capture_macbook.sh   -- single-shot MacBook capture + explain + claim

    tests/                 -- 58 FARD tests, 0 failures

    cfid_swift_if/         -- Swift port (Phase B.1), 9 tests
    cfid_kotlin_if/        -- Kotlin port (Phase B.2), 9 tests
    cfid_go_if/            -- Go port (Phase B.3), 9 tests
    cfid_ts_if/            -- TypeScript port (Phase B.4), 9 tests
    cfid_python_if/        -- Python port (Phase B.5), 22 tests

    conformance/
      if_vectors.json      -- canonical pixel_digest + hash12 vectors
      CONFORMANCE.md       -- conformance guide for new implementations

## Usage

Requires imagesnap:

    brew install imagesnap

Single capture with full identity profile:

    bash tools/capture_macbook.sh

Or manually:

    imagesnap -w 1.5 out/captures/capture.jpg
    sips -Z 320 out/captures/capture.jpg --out out/captures/capture_small.jpg
    fardrun run --program apps/capture_explain.fard --out out/explain -- out/captures/capture_small.jpg 4

Two-image comparison:

    fardrun run --program apps/capture_explain.fard --out out/compare -- image_a.jpg image_b.jpg 4

Live capture session (tamper-evident frame chain):

    fardrun run --program apps/capture_session.fard --out out/s -- start out/sessions/my_session image_001.jpg
    fardrun run --program apps/capture_session.fard --out out/s -- add   out/sessions/my_session image_002.jpg
    fardrun run --program apps/capture_session.fard --out out/s -- show  out/sessions/my_session
    fardrun run --program apps/capture_session.fard --out out/s -- verify out/sessions/my_session

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

    # Python port
    cd cfid_python_if && python3 -m pytest tests/ -v

## Conformance

All five implementations agree on the canonical pixel_digest vector
from SPEC-IF-1.0.md Appendix A.1 (4x4 colorful test image):

    sha256:2a40854db5a950bbdc8d921620a2ee8074fb8b950102150b6be7b10990e1ddb6
    PIXELHASH12: 2A40854DB5A9

See conformance/if_vectors.json for the full conformance suite.

## Phase E experimental results

Validated by real MacBook FaceTime HD Camera captures.

| Experiment | dHash result | Histogram sketch |
|---|---|---|
| E.1 still x5 | Hamming max=5 -- STABLE | Identical across all frames |
| E.2 lighting change | Dominated by pose, not lighting | Identical -- lighting-invariant |
| E.3 JPEG re-encode q=95 to q=25 | Hamming max=2 -- fully stable | Identical across all qualities |
| E.4 movement inflection | Hamming=8 slight turn, 18 medium turn | Identical -- movement-invariant |

Conclusions:
- dHash <= 5 near-duplicate threshold validated by real camera data
- Histogram sketch is stable across lighting, movement, and compression
- dHash is sensitive to pose change, robust to compression and lighting

## Status

| Phase | Description | Status |
|---|---|---|
| A | Full-res pixel identity, EXIF claims, conformance vectors | Complete |
| B | Swift, Kotlin, Go, TypeScript ports + conformance suite | Complete |
| C | Perceptual identity (dHash + histogram sketch) | Complete |
| D | Local camera capture bridge (MacBook) | Complete |
| E | Capture validation experiments (PERC-ID stability) | Complete |
| F | Live capture protocol (frame chains) | Complete |

## Stats

- 3,148 lines of FARD
- 58 FARD tests, 58 language port tests, 116 total, 0 failures
- 6 implementations (FARD + Swift + Kotlin + Go + TypeScript + Python)
