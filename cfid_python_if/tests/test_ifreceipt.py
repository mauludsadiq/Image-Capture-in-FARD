# Tests for ifreceipt Python port.
# Run with: python3 -m pytest tests/ -v
# or:       python3 -m unittest discover tests/

import sys
import os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'src'))

import unittest
from ifreceipt import (
    pixel_digest_canonical, hash12, make_receipt, verify_receipt,
    dhash16, hist_sketch, make_perc_id, hamming, sha256_text
)


def colorful_pixels():
    """4x4 canonical test image from SPEC-IF-1.0.md Appendix A.1."""
    colors = [(220,50,50),(50,180,80),(50,90,220),(240,200,40)]
    return [colors[(i%4 + i//4) % 4] for i in range(16)]


ZERO_DIGEST = "sha256:" + "0" * 64


class TestPixelDigest(unittest.TestCase):
    def test_canonical_vector(self):
        """SPEC-IF-1.0.md Appendix A.1 canonical vector."""
        self.assertEqual(
            pixel_digest_canonical(colorful_pixels()),
            "sha256:2a40854db5a950bbdc8d921620a2ee8074fb8b950102150b6be7b10990e1ddb6"
        )

    def test_same_pixels_same_digest(self):
        self.assertEqual(
            pixel_digest_canonical(colorful_pixels()),
            pixel_digest_canonical(colorful_pixels())
        )

    def test_different_pixels_different_digest(self):
        red  = [(255,0,0)] * 4
        blue = [(0,0,255)] * 4
        self.assertNotEqual(
            pixel_digest_canonical(red),
            pixel_digest_canonical(blue)
        )


class TestHash12(unittest.TestCase):
    def test_canonical_vector(self):
        digest = "sha256:2a40854db5a950bbdc8d921620a2ee8074fb8b950102150b6be7b10990e1ddb6"
        self.assertEqual(hash12(digest), "2A40854DB5A9")

    def test_uppercase(self):
        digest = "sha256:abcdef000000000000000000000000000000000000000000000000000000000000"
        self.assertEqual(hash12(digest), "ABCDEF000000")


class TestReceipt(unittest.TestCase):
    def _fields(self, **kwargs):
        base = dict(
            pixels=colorful_pixels(),
            byte_digest=ZERO_DIGEST,
            format="png", width=4, height=4
        )
        base.update(kwargs)
        return base

    def test_make_receipt_is_valid(self):
        r = make_receipt(self._fields())
        result = verify_receipt(r)
        self.assertTrue(result["valid"], result["reason"])

    def test_if_id_format(self):
        r = make_receipt(self._fields())
        self.assertEqual(len(r["if_id"]), 28)
        self.assertTrue(r["if_id"].startswith("IF-"))

    def test_if_version(self):
        r = make_receipt(self._fields())
        self.assertEqual(r["if_version"], "IF-CAPTURE-1.0.0")

    def test_parent_if_id_none_for_original(self):
        r = make_receipt(self._fields())
        self.assertIsNone(r["parent_if_id"])

    def test_tampered_receipt_detected(self):
        r = make_receipt(self._fields())
        r["if_id"] = "IF-000000000000-000000000000"
        result = verify_receipt(r)
        self.assertFalse(result["valid"])

    def test_deterministic(self):
        r1 = make_receipt(self._fields())
        r2 = make_receipt(self._fields())
        self.assertEqual(r1["if_id"], r2["if_id"])


class TestDHash(unittest.TestCase):
    def test_returns_16_uppercase_hex(self):
        dh = dhash16(colorful_pixels(), 4, 4)
        self.assertEqual(len(dh), 16)
        self.assertEqual(dh, dh.upper())

    def test_uniform_9x9_is_zero(self):
        red = [(255,0,0)] * 81
        self.assertEqual(dhash16(red, 9, 9), "0000000000000000")

    def test_same_image_same_hash(self):
        self.assertEqual(
            dhash16(colorful_pixels(), 4, 4),
            dhash16(colorful_pixels(), 4, 4)
        )

    def test_different_images_different_hash(self):
        red  = [(255,0,0)] * 16
        blue = [(0,0,255)] * 16
        self.assertNotEqual(dhash16(red, 4, 4), dhash16(blue, 4, 4))


class TestHamming(unittest.TestCase):
    def test_identical_is_zero(self):
        dh = dhash16(colorful_pixels(), 4, 4)
        self.assertEqual(hamming(dh, dh), 0)

    def test_all_zeros_vs_all_ones_is_64(self):
        self.assertEqual(hamming("0000000000000000", "FFFFFFFFFFFFFFFF"), 64)


class TestHistSketch(unittest.TestCase):
    def test_returns_8_uppercase_hex(self):
        hs = hist_sketch(colorful_pixels())
        self.assertEqual(len(hs), 8)
        self.assertEqual(hs, hs.upper())

    def test_deterministic(self):
        red = [(255,0,0)] * 16
        self.assertEqual(hist_sketch(red), hist_sketch(red))

    def test_different_palette_different_sketch(self):
        red  = [(255,0,0)] * 16
        blue = [(0,0,255)] * 16
        self.assertNotEqual(hist_sketch(red), hist_sketch(blue))


class TestPercId(unittest.TestCase):
    def test_format(self):
        pid = make_perc_id(colorful_pixels(), 4, 4)
        self.assertEqual(len(pid), 30)
        self.assertTrue(pid.startswith("PERC-"))

    def test_none_for_tiny_image(self):
        self.assertIsNone(make_perc_id([(255,0,0)]*4, 2, 2))


if __name__ == "__main__":
    unittest.main()
