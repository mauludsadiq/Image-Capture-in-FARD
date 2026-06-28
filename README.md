# Image Capture in FARD

**Deterministic image identity and receipted capture.**

Every image has a permanent, verifiable identity: an IF-ID, computed from
its decoded pixels, not its file bytes. JPEG recompression changes bytes
but not pixels; metadata edits change receipts but not pixel identity.
IF Protocol separates these four identities simultaneously:

   byte_digest    -- exact file identity (SHA-256 of raw file bytes)
   pixel_digest   -- decoded pixel identity (SHA-256 of canonical RGB)
   IF-ID          -- composite identity (pixel + receipt hash)
   CF palette     -- every dominant colour identified by its CF-ID

Built on FARD and CF-Protocol-2.0.0 (Colour in FARD).
2,001 lines of FARD, 38 tests, zero failures.

---

## Quickstart

Full identity profile for any image:

   fardrun run --program apps/image_explain.fard --out out/run -- photo.jpg

   IF-ID:         IF-8EDC2910BB67-4B902CEE5506
   pixel_digest:  sha256:8edc2910bb67d837ddf1a5e0d2e23f86879c8254d5ce4e034e9d5301b40e9e30
   byte_digest:   sha256:e358d4fdb4c1028bd902d8dfb8e50ae5e2a4547f523631bc59130d2767c55ac1
   receipt_digest: sha256:4b902cee5506117dd1f609d0fb600eccfbe1c048170f9b69c1dbc8aab93703e3
   format:        png
   colorspace:    sRGB
   dominant palette (k=4):
     1. #cb5236  CF-CB5236-1EF4B185  contrast/white=4.38
     2. #4e9d5c  CF-4E9D5C-B80A0AC5  contrast/white=3.33
     3. #4e6fba  CF-4E6FBA-9EF205F1  contrast/white=4.88
     4. #dcae3d  CF-DCAE3D-06F90470  contrast/white=2.07

Compare two images across all four identities:

   fardrun run --program apps/image_diff.fard --out out/run -- original.jpg published.jpg

   byte changed?    YES
   pixel changed?   no
   IF-ID changed?   YES
   palette changed? no
   diagnosis: METADATA ONLY -- bytes differ but pixels are identical

Create and verify a tamper-evident image claim:

   fardrun run --program apps/image_claim.fard --out out/run -- create photo.jpg "Press photo 2024-06-28" out/claim.json
   fardrun run --program apps/image_claim.fard --out out/run -- verify out/claim.json

   VALID -- if_id matches pixel_digest, claim_digest matches content

Build and verify an edit chain:

   fardrun run --program apps/image_chain.fard --out out/run -- init original.jpg out/chain
   fardrun run --program apps/image_chain.fard --out out/run -- add out/chain crop
   fardrun run --program apps/image_chain.fard --out out/run -- add out/chain compress cropped.jpg
   fardrun run --program apps/image_chain.fard --out out/run -- verify out/chain

   step 0 (original)  IF-8EDC2910BB67-4B902CEE5506  VALID
   step 1 (crop)      IF-E3B0C44298FC-6481F1E2EF7F  VALID
   step 2 (compress)  IF-6821987238D5-137A3986935B  VALID
   3/3 steps valid

---

## Apps

| App | Usage |
|---|---|
| apps/image_explain.fard | Full IF-ID profile: four identities + CF palette |
| apps/image_diff.fard | Compare two images across all four identities |
| apps/image_claim.fard | Create/verify tamper-evident IF-Protocol claims |
| apps/image_chain.fard | Build/verify/show edit chains with parent linkage |

---

## How It Works

**IF-ID.** Twenty-eight characters: IF-<PIXELHASH12>-<RECEIPTHASH12>.
The pixel half is the first 12 uppercase hex characters of SHA-256 over
the canonical pixel byte sequence (r0 g0 b0 r1 g1 b1 ... left-to-right,
top-to-bottom, 8-bit sRGB). The receipt half is derived from the receipt
JSON. Same decoded pixels always produce the same PIXELHASH12, regardless
of file format, compression, or metadata.

**Four identities, cleanly separated.** A JPEG recompression changes
byte_digest but not pixel_digest (if the pixels are visually identical
after decoding). A metadata/EXIF edit changes byte_digest and
receipt_digest but not pixel_digest. A crop or colour edit changes all
four. image_diff diagnoses which category an edit falls into.

**Edit chains.** Each transformation step records the parent_if_id of
the previous step, forming a verifiable chain from original capture to
final published asset. Any step can be independently verified; a broken
parent link is immediately detectable.

**CF palette.** Every dominant colour in an image is identified by its
CF-ID (from CF-Protocol-2.0.0), making the palette portable, verifiable,
and consistent with the Colour in FARD ecosystem.

**EXIF is a claim, not truth.** IF Protocol records EXIF fields in the
receipt source block but treats them as unverified claims. The pixel
identity is independent of all metadata.

---

## Architecture

   SPEC-IF-1.0.md               -- IF Protocol Specification v1.0.0
   src/core/if_id.fard          -- pixel_digest_canonical, hash12, make_if_id
   src/core/image_receipt.fard  -- make_receipt, verify_receipt, build_palette
   src/core/image_read.fard     -- sips-based image pixel reader
   src/core/vendor/             -- vendored CF-Protocol core (cf_id, rgb_lab, etc.)
   apps/image_explain.fard      -- full identity profile
   apps/image_diff.fard         -- four-identity image comparison
   apps/image_claim.fard        -- tamper-evident image claims
   apps/image_chain.fard        -- edit chain build/verify/show
   tests/test_if_id.fard        -- 8 tests: hash12, pixel_digest, make_if_id
   tests/test_image_receipt.fard -- 10 tests: make_receipt, verify_receipt, tamper
   tests/test_image_diff.fard   -- 6 tests: diff logic across all four identities
   tests/test_image_claim.fard  -- 7 tests: claim create/verify/tamper
   tests/test_image_chain.fard  -- 7 tests: chain linkage, parent_if_id, verify

---

## Validation

38 tests, 0 failures:

- IF-ID format and length (IF-XXXXXXXXXXXX-XXXXXXXXXXXX, 28 chars)
- Same pixels always produce the same pixel_digest and IF-ID
- Different pixels produce different pixel_digest
- Metadata-only change: same pixel_digest, different receipt_digest, different IF-ID
- Tampered receipt_digest detected
- Tampered claim fields (if_id, name) detected
- Edit chain parent linkage verified
- Broken chain linkage detectable
- All steps in a 4-step chain verify as VALID

---

## Relationship to Colour in FARD

IF Protocol and CF-Protocol are independent but complementary:

- CF-ID identifies a single 8-bit sRGB colour, permanently.
- IF-ID identifies a decoded image, permanently.
- Every pixel of an IF-ID image has a CF-ID.
- An image palette is a list of CF-IDs derived from its pixels.

IF Protocol depends on CF-Protocol-2.0.0 for palette generation.
CF-Protocol has no dependency on IF Protocol.

   Colour in FARD: https://github.com/mauludsadiq/Colour-in-Fard
   IF Protocol:    https://github.com/mauludsadiq/Image-Capture-in-FARD

---

## Built with FARD

https://github.com/mauludsadiq/FARD

## License

MUI
