# IF Protocol Specification v1.0.0

**Image in FARD -- deterministic image identity and receipted capture.**

## Status

Draft v1.0.0. This document defines the IF-ID identity scheme, the
canonical image record, and the image receipt format. It is a companion
to CF-Protocol-2.0.0 (Colour in FARD), which defines CF-ID for
individual colours. IF Protocol builds on CF-ID: every image has an
IF-ID; every pixel in that image has a CF-ID.

---

## 1. Motivation

A file hash identifies bytes. A pixel hash identifies decoded visual
content. A perceptual summary identifies what a human sees. These are
three distinct identities, and none of them is sufficient alone:

- JPEG recompression changes bytes but may not change pixels meaningfully.
- Two files with different bytes may decode to identical pixels.
- Two images with identical pixels may differ in colour profile,
  orientation, or metadata.

IF Protocol provides all three identities simultaneously, in a single
verifiable receipt, with no trust required in file metadata or EXIF.

---

## 2. IF-ID

### 2.1 Format

    IF-<PIXELHASH12>-<RECEIPTHASH12>

Where:

- PIXELHASH12 is the first 12 uppercase hex characters of the SHA-256
  of the canonical pixel byte sequence (see section 3).
- RECEIPTHASH12 is the first 12 uppercase hex characters of the SHA-256
  of the canonical receipt JSON (see section 4), excluding the if_id and
  receipt_digest fields.

### 2.2 Properties

- Content-derived: dimensions, format, and metadata are NOT embedded
  in the identifier. They belong in the receipt.
- Permanent: same decoded canonical pixels + same receipt fields =>
  same IF-ID.
- Separable: PIXELHASH12 alone identifies pixel content. A
  metadata-only change produces the same PIXELHASH12 but a different
  RECEIPTHASH12, and therefore a different IF-ID.
- Edit-chained: a crop, resize, rotate, or any pixel-modifying
  transformation produces a new IF-ID with parent_if_id in its receipt
  pointing to the original.

---

## 3. Canonical Pixel Sequence

The canonical pixel byte sequence is constructed as follows:

1. Decode the image to a raster of 8-bit sRGB pixels (r, g, b), applying
   ICC profile conversion if a profile is present, or assuming sRGB if
   not.
2. Apply EXIF orientation (if available) so that the canonical sequence
   always represents the image in its correct upright orientation.
3. Serialise left-to-right, top-to-bottom, one pixel per triplet:
   r0 g0 b0 r1 g1 b1 ... rN gN bN
4. Compute SHA-256 over this byte sequence.

EXIF and ICC metadata are NOT included in the pixel sequence. They are
recorded in the receipt as claims, not as identity inputs.

### 3.1 Sampling

For large images, an implementation MAY compute the pixel digest over a
uniformly sampled subset (e.g. every Nth pixel), recording the sampling
stride in the receipt. A full-resolution digest uses stride=1. Sampled
digests are not interchangeable with full-resolution digests; the stride
MUST be included in the receipt.

---

## 4. Image Receipt

The canonical image receipt is a JSON object with alphabetical keys and
no extraneous whitespace (same canonical JSON as CF-Protocol-2.0.0).

Fields:

  byte_digest           SHA-256 of the raw file bytes as read from disk.
  colorspace            sRGB | DisplayP3 | RAW_SENSOR | unknown
  format                png | jpeg | bmp | heic | raw | unknown
  height                pixel height after orientation correction
  icc_digest            SHA-256 of the embedded ICC profile, or null
  if_version            IF-CAPTURE-1.0.0
  orientation           EXIF orientation tag (1-8), or 1 if absent
  palette               list of {hex, cf_id} dominant colours
  parent_if_id          IF-ID of the source image, or null
  pixel_digest          SHA-256 of the canonical pixel byte sequence
  pixel_sample_stride   1 for full-resolution, N for sampled
  source                {device_claim, lens_profile_digest,
                         location_claim, sensor_profile_digest,
                         timestamp} -- all null if not supplied
  width                 pixel width after orientation correction

### 4.1 receipt_digest and if_id

After constructing the receipt object (without if_id or receipt_digest):

1. Serialise to canonical JSON (alphabetical keys, no whitespace).
2. receipt_digest = sha256: + hex(SHA-256(canonical_json)).
3. RECEIPTHASH12 = first 12 uppercase hex chars of the SHA-256.
4. PIXELHASH12 = first 12 uppercase hex chars of pixel_digest
   (stripping the sha256: prefix).
5. if_id = IF-<PIXELHASH12>-<RECEIPTHASH12>.
6. Add if_id and receipt_digest to the receipt object.

---

## 5. Edit Receipts

A transformation (crop, resize, rotate, white balance, etc.) produces a
new receipt with:

- parent_if_id set to the IF-ID of the source image.
- A new pixel_digest reflecting the transformed pixels.
- An operation field (crop | resize | rotate | original).
- A params field with transformation parameters (alphabetical keys).

Example crop receipt fields:
  operation    crop
  params       {"h":600,"w":900,"x":120,"y":80}
  parent_if_id IF-AABBCCDD1122-EEFF00112233

---

## 6. Conformance

A conformant IF-Protocol-1.0.0 implementation MUST:

1. Produce the same pixel_digest for the same decoded canonical pixels.
2. Produce the same receipt_digest for the same receipt fields.
3. Produce the same if_id from the same pixel_digest and receipt_digest.
4. Record parent_if_id for any derived image.
5. Never embed dimensions or format in the IF-ID itself.

---

## 7. Relationship to CF-Protocol

CF-ID and IF-ID are independent but complementary:

- CF-ID identifies a single 8-bit sRGB colour, permanently.
- IF-ID identifies a decoded image, permanently.
- Every pixel of an IF-ID image has a CF-ID.
- An image palette is a list of CF-IDs derived from its pixels.

IF Protocol depends on CF-Protocol-2.0.0 for palette generation.
CF-Protocol has no dependency on IF Protocol.

---

## Appendix A: Test Vectors

These vectors are canonical. Any conformant IF-Protocol-1.0.0
implementation MUST reproduce them exactly.

### A.1 Synthetic image: 4x4 colorful PNG

Source: a 4x4 PNG with four cycling colours (red=#D13232, green=#32CB32,
blue=#3232CB, yellow=#CBCB32), tiled across 16 pixels. Generated by the
reference test suite as out/colorful.png.

Canonical pixel sequence preimage: the string
"r0,g0,b0,r1,g1,b1,...,r15,g15,b15," where each value is the 8-bit
integer (0-255) representation of the decoded sRGB pixel channel,
left-to-right, top-to-bottom.

  pixel_digest:
    sha256:2a40854db5a950bbdc8d921620a2ee8074fb8b950102150b6be7b10990e1ddb6

  PIXELHASH12:  2A40854DB5A9

The byte_digest and receipt_digest depend on metadata fields (format,
byte content, palette_k, etc.) and are therefore not canonical across
implementations -- only pixel_digest and PIXELHASH12 are stable
conformance targets.

### A.2 round3 vectors (inherited from CF-Protocol-2.0.0)

The pixel_digest preimage uses integer-rounded values (cast.int(r*255+0.5))
not round3. These rounding rules are independent; see CF-Protocol-2.0.0
conformance/vectors.json for CF-ID round3 vectors.

### A.3 IF-ID format

  IF-ID length: 28 characters
  Format:       IF-XXXXXXXXXXXX-XXXXXXXXXXXX
  where X is an uppercase hex digit (0-9, A-F).

Any implementation producing an IF-ID of a different length or format
for any input is non-conformant.
