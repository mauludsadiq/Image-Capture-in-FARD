# SPEC-PERC-1.0.md — IF Perceptual Identity Protocol 1.0

## Overview

The Perceptual Identity (PERC) protocol extends IF-Protocol-1.0.0 with
a similarity-stable image fingerprint. Where pixel_digest is
cryptographically exact (one changed pixel changes it completely),
PERC-ID is designed to be stable across:

- JPEG re-encoding at any quality
- Moderate resize (up to ~50% change in dimensions)
- Slight brightness or contrast adjustment
- Lossless format conversion (PNG -> JPEG -> WebP)

PERC-ID is NOT a cryptographic identity. It is a similarity signal.
Two images with the same PERC-ID are likely visually similar. Two
images with different PERC-IDs may still be visually similar (false
negative) but are unlikely to be identical.

## PERC-ID Format

   PERC-<DHASH16>-<HISTSKETCH8>

Total length: 30 characters.

- PERC-  : literal prefix (5 chars)
- DHASH16: 16 uppercase hex chars encoding a 64-bit dHash (8 chars)
- -      : separator (1 char)
- HISTSKETCH8: 8 uppercase hex chars encoding a 32-bit histogram sketch

Example: PERC-A3F2B1C4D5E6F708-1A2B3C4D

## Component 1: dHash (64-bit difference hash)

dHash measures horizontal gradient in a grayscale thumbnail. It is
robust to JPEG compression, moderate brightness changes, and resize.

### Algorithm

1. Resize the image to 9x8 pixels using bilinear interpolation,
  converting to grayscale (luminance = 0.299r + 0.587g + 0.114b).
2. For each row (8 rows), compute 8 bits by comparing adjacent pixels:
  bit = 1 if pixel[col] > pixel[col+1], else 0.
  This gives 8 rows x 8 bits = 64 bits total.
3. Encode the 64 bits as 16 uppercase hex characters (DHASH16),
  most-significant bit first, row 0 first.

### Similarity

Hamming distance on the 64-bit value (count of differing bits):
- 0     : identical images (or perceptually identical)
- 1-5   : near-duplicate (compression artefact, slight edit)
- 6-10  : possibly related (crop, filter)
- 11+   : different images

### Reference vector

Single-colour red image (any size): all pixels (255,0,0).
Thumbnail: 9x8 red pixels. All rows are uniform -> all gradient
bits are 0 -> dHash = 0x0000000000000000 -> DHASH16 = "0000000000000000".

4x4 colorful test image (see if_vectors.json pixel_digest_vectors[0]):
DHASH16 = (computed by reference implementation, see perc_vectors.json)

## Component 2: Histogram Sketch (32-bit)

The histogram sketch captures the colour palette of the image,
independently of spatial arrangement. It is stable across resize,
crop, and compression but changes when the dominant colours change.

### Algorithm

1. Quantise each pixel to a 4x4x4 RGB bucket:
  bucket = (r // 64) * 16 + (g // 64) * 4 + (b // 64)
  This gives 64 possible buckets (values 0-63).
2. Count pixels in each bucket. Normalise counts to [0,255] by
  dividing by total pixels and multiplying by 255.
3. Find the 4 most populated buckets (ties broken by bucket index).
4. Encode as 32 bits: 4 x (6-bit bucket_index | 2 padding bits) -> 
  pack as two 16-bit values -> 8 uppercase hex chars (HISTSKETCH8).

  Bit layout (32 bits, MSB first):
  [6-bit bucket0][2-bit count_tier0][6-bit bucket1][2-bit count_tier1]
  [6-bit bucket2][2-bit count_tier2][6-bit bucket3][2-bit count_tier3]

  count_tier: 0=rare(<10%), 1=common(10-25%), 2=dominant(25-50%), 3=majority(>50%)

### Similarity

Images with the same dominant palette will share HISTSKETCH8.
Used as a secondary filter: same DHASH16 + same HISTSKETCH8 = very
high confidence near-duplicate.

## PERC-ID in IF-Protocol receipts

The perc_id field is added to IFReceipt as an optional string:

   perc_id: string | null

It is null when:
- The image has fewer than 9 pixels (too small for dHash)
- The implementation does not support Phase C

perc_id is NOT included in the receipt_digest preimage. It is
informational only and does not affect the cryptographic identity.

## Versioning

This document describes PERC-1.0. The prefix "PERC-" identifies the
version implicitly. Future versions would use a different prefix.
