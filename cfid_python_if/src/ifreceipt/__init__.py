# ifreceipt -- IF-Protocol-1.0.0 and PERC-1.0 in Python.
#
# Pure Python, zero dependencies (hashlib for SHA-256, part of the
# standard library). Fifth language port after Swift, Kotlin, Go,
# and TypeScript. Conformant with SPEC-IF-1.0.md and SPEC-PERC-1.0.md.
#
# Public API:
#   pixel_digest_canonical(pixels)        -> str
#   hash12(digest)                        -> str
#   make_receipt(fields)                  -> dict
#   verify_receipt(receipt)               -> dict {valid, reason}
#   dhash16(pixels, width, height)        -> str
#   hist_sketch(pixels)                   -> str
#   make_perc_id(pixels, width, height)   -> str | None
#   hamming(a, b)                         -> int
#
# pixels: list of (r, g, b) tuples, integers 0-255.

import hashlib
import json
from dataclasses import dataclass, field, asdict
from typing import Optional


# ---------------------------------------------------------------------------
# Core SHA-256 helpers
# ---------------------------------------------------------------------------

def sha256_text(s: str) -> str:
   """SHA-256 of a UTF-8 string, returned as 'sha256:<hex>'."""
   h = hashlib.sha256(s.encode("utf-8")).hexdigest()
   return f"sha256:{h}"


def sha256_bytes(data: bytes) -> str:
   """SHA-256 of raw bytes, returned as 'sha256:<hex>'."""
   h = hashlib.sha256(data).hexdigest()
   return f"sha256:{h}"


def hash12(digest: str) -> str:
   """First 12 uppercase hex chars from a 'sha256:...' digest."""
   return digest[7:19].upper()


# ---------------------------------------------------------------------------
# Pixel digest (IF-Protocol-1.0.0 section 3)
# ---------------------------------------------------------------------------

def pixel_digest_canonical(pixels: list) -> str:
   """Canonical pixel digest.

   Preimage: 'r0,g0,b0,r1,g1,b1,...,rN,gN,bN,' (trailing comma, same
   format as FARD reference implementation and all language ports).

   pixels: list of (r, g, b) tuples, integers 0-255.
   """
   parts = []
   for r, g, b in pixels:
       parts.append(f"{r},{g},{b},")
   return sha256_text("".join(parts))


def pixel_digest_from_floats(pixels: list) -> str:
   """Pixel digest from float [0,1] pixels."""
   converted = [
       (min(255, max(0, int(r * 255.0 + 0.5))),
        min(255, max(0, int(g * 255.0 + 0.5))),
        min(255, max(0, int(b * 255.0 + 0.5))))
       for r, g, b in pixels
   ]
   return pixel_digest_canonical(converted)


# ---------------------------------------------------------------------------
# Canonical JSON for receipt preimage (alphabetical keys, no whitespace)
# ---------------------------------------------------------------------------

def _js(s) -> str:
   if s is None:
       return "null"
   escaped = str(s).replace("\\", "\\\\").replace('"', '\\"')
   return f'"{escaped}"'


def _ji(n: int) -> str:
   return str(n)


def _jp(palette: list) -> str:
   entries = [
       '{"cf_id":' + _js(e.get("cf_id")) + ',"hex":' + _js(e.get("hex")) + "}"
       for e in palette
   ]
   return "[" + ",".join(entries) + "]"


def _jsrc(source: dict) -> str:
   return (
       "{"
       + '"device_claim":' + _js(source.get("device_claim"))
       + ',"lens_profile_digest":' + _js(source.get("lens_profile_digest"))
       + ',"location_claim":' + _js(source.get("location_claim"))
       + ',"sensor_profile_digest":' + _js(source.get("sensor_profile_digest"))
       + ',"timestamp":' + _js(source.get("timestamp"))
       + "}"
   )


def _preimage_json(r: dict) -> str:
   return (
       "{"
       + '"byte_digest":' + _js(r["byte_digest"])
       + ',"colorspace":' + _js(r["colorspace"])
       + ',"format":' + _js(r["format"])
       + ',"height":' + _ji(r["height"])
       + ',"icc_digest":' + _js(r.get("icc_digest"))
       + ',"if_version":' + _js(r["if_version"])
       + ',"operation":' + _js(r["operation"])
       + ',"orientation":' + _ji(r["orientation"])
       + ',"palette":' + _jp(r.get("palette", []))
       + ',"params":' + _js(r.get("params"))
       + ',"parent_if_id":' + _js(r.get("parent_if_id"))
       + ',"pixel_digest":' + _js(r["pixel_digest"])
       + ',"pixel_sample_stride":' + _ji(r["pixel_sample_stride"])
       + ',"source":' + _jsrc(r.get("source", {}))
       + ',"width":' + _ji(r["width"])
       + "}"
   )


# ---------------------------------------------------------------------------
# Receipt builder
# ---------------------------------------------------------------------------

EMPTY_SOURCE = {
   "device_claim": None,
   "sensor_profile_digest": None,
   "lens_profile_digest": None,
   "timestamp": None,
   "location_claim": None,
}


def make_receipt(fields: dict) -> dict:
   """Build a complete IF-Protocol-1.0.0 receipt.

   Required fields: pixels, byte_digest, format, width, height.
   Optional: colorspace, icc_digest, orientation, palette,
             source, parent_if_id, operation, params.
   """
   pd = pixel_digest_canonical(fields["pixels"])
   partial = {
       "byte_digest":        fields["byte_digest"],
       "colorspace":         fields.get("colorspace", "sRGB"),
       "format":             fields["format"],
       "height":             fields["height"],
       "icc_digest":         fields.get("icc_digest"),
       "if_id":              "",
       "if_version":         "IF-CAPTURE-1.0.0",
       "operation":          fields.get("operation", "original"),
       "orientation":        fields.get("orientation", 1),
       "palette":            fields.get("palette", []),
       "params":             fields.get("params"),
       "parent_if_id":       fields.get("parent_if_id"),
       "pixel_digest":       pd,
       "pixel_sample_stride": 1,
       "receipt_digest":     "",
       "source":             fields.get("source", EMPTY_SOURCE),
       "width":              fields["width"],
   }
   rd = sha256_text(_preimage_json(partial))
   if_id = f"IF-{hash12(pd)}-{hash12(rd)}"
   partial["if_id"] = if_id
   partial["receipt_digest"] = rd
   return partial


def verify_receipt(receipt: dict) -> dict:
   """Verify a receipt by recomputing if_id and receipt_digest.

   Returns {"valid": bool, "reason": str}.
   """
   expected_rd = sha256_text(_preimage_json(receipt))
   expected_id = f"IF-{hash12(receipt['pixel_digest'])}-{hash12(expected_rd)}"
   if receipt["receipt_digest"] != expected_rd:
       return {"valid": False, "reason": "receipt_digest mismatch"}
   if receipt["if_id"] != expected_id:
       return {"valid": False, "reason": "if_id mismatch"}
   return {"valid": True, "reason": "ok"}


# ---------------------------------------------------------------------------
# PERC-1.0: dHash + histogram sketch
# ---------------------------------------------------------------------------

def _luminance(r: int, g: int, b: int) -> float:
   return 0.299 * r + 0.587 * g + 0.114 * b


def _resize_gray(pixels: list, src_w: int, src_h: int,
                tgt_w: int, tgt_h: int) -> list:
   """Bilinear resize to tgt_w x tgt_h, return flat list of grayscale floats."""
   result = []
   for ty in range(tgt_h):
       for tx in range(tgt_w):
           sx = (tx + 0.5) * src_w / tgt_w - 0.5
           sy = (ty + 0.5) * src_h / tgt_h - 0.5
           x0 = max(0, int(sx))
           y0 = max(0, int(sy))
           x1 = min(src_w - 1, x0 + 1)
           y1 = min(src_h - 1, y0 + 1)
           dx = sx - int(sx)
           dy = sy - int(sy)
           p = pixels[y0 * src_w + x0]
           q = pixels[y0 * src_w + x1]
           r = pixels[y1 * src_w + x0]
           s = pixels[y1 * src_w + x1]
           lp = _luminance(*p)
           lq = _luminance(*q)
           lr = _luminance(*r)
           ls = _luminance(*s)
           val = ((1-dx)*(1-dy)*lp + dx*(1-dy)*lq +
                  (1-dx)*dy*lr     + dx*dy*ls)
           result.append(val)
   return result


def dhash16(pixels: list, width: int, height: int) -> str:
   """64-bit dHash as 16 uppercase hex chars.

   pixels: list of (r, g, b) tuples, integers 0-255.
   """
   thumb = _resize_gray(pixels, width, height, 9, 8)
   bits = []
   for row in range(8):
       for col in range(8):
           left  = thumb[row * 9 + col]
           right = thumb[row * 9 + col + 1]
           bits.append(1 if left > right else 0)
   # Pack 64 bits into 16 hex chars (4 bits each)
   hex_chars = "0123456789ABCDEF"
   result = []
   for i in range(16):
       nibble = (bits[i*4] * 8 + bits[i*4+1] * 4 +
                 bits[i*4+2] * 2 + bits[i*4+3])
       result.append(hex_chars[nibble])
   return "".join(result)


def hist_sketch(pixels: list) -> str:
   """32-bit colour histogram sketch as 8 uppercase hex chars.

   4x4x4 RGB buckets, top-4 by population encoded as
   4 x (6-bit bucket_index | 2-bit count_tier).
   """
   total = len(pixels)
   counts = [0] * 64
   for r, g, b in pixels:
       bucket = (r // 64) * 16 + (g // 64) * 4 + (b // 64)
       counts[bucket] += 1

   indexed = sorted(
       enumerate(counts),
       key=lambda x: (-x[1], x[0])
   )
   top4 = indexed[:4]

   def count_tier(cnt):
       pct = cnt / total if total > 0 else 0
       if pct > 0.5:  return 3
       if pct > 0.25: return 2
       if pct > 0.1:  return 1
       return 0

   hex_chars = "0123456789ABCDEF"
   result = []
   for slot in range(4):
       idx, cnt = top4[slot]
       tier = count_tier(cnt)
       high_nibble = idx // 4
       low_nibble  = (idx % 4) * 4 + tier
       result.append(hex_chars[high_nibble])
       result.append(hex_chars[low_nibble])
   return "".join(result)


def make_perc_id(pixels: list, width: int, height: int):
   """PERC-ID string or None if image is too small (< 9 pixels)."""
   if len(pixels) < 9:
       return None
   dh = dhash16(pixels, width, height)
   hs = hist_sketch(pixels)
   return f"PERC-{dh}-{hs}"


def hamming(a: str, b: str) -> int:
   """Hamming distance between two DHASH16 strings (count of differing bits)."""
   def nibble_bits(c):
       v = int(c, 16)
       return bin(v).count("1")
   total = 0
   for ca, cb in zip(a, b):
       va = int(ca, 16)
       vb = int(cb, 16)
       total += bin(va ^ vb).count("1")
   return total
