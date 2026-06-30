// Tests for ifreceipt Rust port.
// Run with: cargo test

use ifreceipt::*;

fn colorful_pixels() -> Vec<(u8, u8, u8)> {
    // 4x4 canonical test image from SPEC-IF-1.0.md Appendix A.1
    let colors: [(u8, u8, u8); 4] = [(220, 50, 50), (50, 180, 80), (50, 90, 220), (240, 200, 40)];
    (0..16).map(|i| colors[(i % 4 + i / 4) % 4]).collect()
}

fn zero_digest() -> String {
    format!("sha256:{}", "0".repeat(64))
}

#[test]
fn test_canonical_pixel_digest_vector() {
    assert_eq!(
        pixel_digest_canonical(&colorful_pixels()),
        "sha256:2a40854db5a950bbdc8d921620a2ee8074fb8b950102150b6be7b10990e1ddb6"
    );
}

#[test]
fn test_same_pixels_same_digest() {
    assert_eq!(
        pixel_digest_canonical(&colorful_pixels()),
        pixel_digest_canonical(&colorful_pixels())
    );
}

#[test]
fn test_different_pixels_different_digest() {
    let red = vec![(255u8, 0u8, 0u8); 4];
    let blue = vec![(0u8, 0u8, 255u8); 4];
    assert_ne!(pixel_digest_canonical(&red), pixel_digest_canonical(&blue));
}

#[test]
fn test_hash12_canonical_vector() {
    let digest = "sha256:2a40854db5a950bbdc8d921620a2ee8074fb8b950102150b6be7b10990e1ddb6";
    assert_eq!(hash12(digest), "2A40854DB5A9");
}

#[test]
fn test_if_id_format() {
    let pd = "sha256:2a40854db5a950bbdc8d921620a2ee8074fb8b950102150b6be7b10990e1ddb6";
    let rd = zero_digest();
    let id = format!("IF-{}-{}", hash12(pd), hash12(&rd));
    assert_eq!(id.len(), 28);
    assert!(id.starts_with("IF-"));
}

#[test]
fn test_make_receipt_is_valid() {
    let fields = IfReceiptFields::new(colorful_pixels(), zero_digest(), "png".to_string(), 4, 4);
    let r = make_receipt(fields);
    let result = verify_receipt(&r);
    assert!(result.valid, "{}", result.reason);
}

#[test]
fn test_if_version() {
    let fields = IfReceiptFields::new(colorful_pixels(), zero_digest(), "png".to_string(), 4, 4);
    let r = make_receipt(fields);
    assert_eq!(r.if_version, "IF-CAPTURE-1.0.0");
}

#[test]
fn test_parent_if_id_none_for_original() {
    let fields = IfReceiptFields::new(colorful_pixels(), zero_digest(), "png".to_string(), 4, 4);
    let r = make_receipt(fields);
    assert!(r.parent_if_id.is_none());
}

#[test]
fn test_tampered_receipt_detected() {
    let fields = IfReceiptFields::new(colorful_pixels(), zero_digest(), "png".to_string(), 4, 4);
    let mut r = make_receipt(fields);
    r.if_id = "IF-000000000000-000000000000".to_string();
    let result = verify_receipt(&r);
    assert!(!result.valid);
}

#[test]
fn test_deterministic() {
    let f1 = IfReceiptFields::new(colorful_pixels(), zero_digest(), "png".to_string(), 4, 4);
    let f2 = IfReceiptFields::new(colorful_pixels(), zero_digest(), "png".to_string(), 4, 4);
    let r1 = make_receipt(f1);
    let r2 = make_receipt(f2);
    assert_eq!(r1.if_id, r2.if_id);
}

#[test]
fn test_dhash16_returns_16_uppercase_hex() {
    let dh = dhash16(&colorful_pixels(), 4, 4);
    assert_eq!(dh.len(), 16);
    assert_eq!(dh, dh.to_uppercase());
}

#[test]
fn test_dhash16_uniform_9x9_is_zero() {
    let red = vec![(255u8, 0u8, 0u8); 81];
    assert_eq!(dhash16(&red, 9, 9), "0000000000000000");
}

#[test]
fn test_dhash16_same_image_same_hash() {
    assert_eq!(dhash16(&colorful_pixels(), 4, 4), dhash16(&colorful_pixels(), 4, 4));
}

#[test]
fn test_dhash16_different_images_different_hash() {
    let red = vec![(255u8, 0u8, 0u8); 16];
    let blue = vec![(0u8, 0u8, 255u8); 16];
    assert_ne!(dhash16(&red, 4, 4), dhash16(&blue, 4, 4));
}

#[test]
fn test_hamming_identical_is_zero() {
    let dh = dhash16(&colorful_pixels(), 4, 4);
    assert_eq!(hamming(&dh, &dh), 0);
}

#[test]
fn test_hamming_all_zeros_vs_all_ones_is_64() {
    assert_eq!(hamming("0000000000000000", "FFFFFFFFFFFFFFFF"), 64);
}

#[test]
fn test_hist_sketch_returns_8_uppercase_hex() {
    let hs = hist_sketch(&colorful_pixels());
    assert_eq!(hs.len(), 8);
    assert_eq!(hs, hs.to_uppercase());
}

#[test]
fn test_hist_sketch_deterministic() {
    let red = vec![(255u8, 0u8, 0u8); 16];
    assert_eq!(hist_sketch(&red), hist_sketch(&red));
}

#[test]
fn test_hist_sketch_different_palette_different_sketch() {
    let red = vec![(255u8, 0u8, 0u8); 16];
    let blue = vec![(0u8, 0u8, 255u8); 16];
    assert_ne!(hist_sketch(&red), hist_sketch(&blue));
}

#[test]
fn test_perc_id_format() {
    let pid = make_perc_id(&colorful_pixels(), 4, 4).unwrap();
    assert_eq!(pid.len(), 30);
    assert!(pid.starts_with("PERC-"));
}

#[test]
fn test_perc_id_none_for_tiny_image() {
    let tiny = vec![(255u8, 0u8, 0u8); 4];
    assert!(make_perc_id(&tiny, 2, 2).is_none());
}
