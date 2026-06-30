//! IF-Protocol-1.0.0 and PERC-1.0 image identity in Rust.
//!
//! Pure Rust, single dependency (sha2 for SHA-256). Sixth language port
//! after Swift, Kotlin, Go, TypeScript, and Python. Conformant with
//! SPEC-IF-1.0.md and SPEC-PERC-1.0.md.

use sha2::{Digest, Sha256};

// ---------------------------------------------------------------------
// Core SHA-256 helpers
// ---------------------------------------------------------------------

/// SHA-256 of a UTF-8 string, returned as "sha256:<hex>".
pub fn sha256_text(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    format!("sha256:{:x}", hasher.finalize())
}

/// SHA-256 of raw bytes, returned as "sha256:<hex>".
pub fn sha256_bytes(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("sha256:{:x}", hasher.finalize())
}

/// First 12 uppercase hex chars from a "sha256:..." digest.
pub fn hash12(digest: &str) -> String {
    digest[7..19].to_uppercase()
}

// ---------------------------------------------------------------------
// Pixel digest (IF-Protocol-1.0.0 section 3)
// ---------------------------------------------------------------------

/// Canonical pixel digest. Preimage: "r0,g0,b0,r1,g1,b1,...,rN,gN,bN,"
/// (trailing comma, same format as FARD reference and all other ports).
pub fn pixel_digest_canonical(pixels: &[(u8, u8, u8)]) -> String {
    let mut s = String::with_capacity(pixels.len() * 12);
    for (r, g, b) in pixels {
        s.push_str(&format!("{},{},{},", r, g, b));
    }
    sha256_text(&s)
}

/// Pixel digest from float [0,1] pixels.
pub fn pixel_digest_from_floats(pixels: &[(f64, f64, f64)]) -> String {
    let converted: Vec<(u8, u8, u8)> = pixels
        .iter()
        .map(|(r, g, b)| {
            let clamp = |v: f64| -> u8 {
                let n = (v * 255.0 + 0.5) as i32;
                n.clamp(0, 255) as u8
            };
            (clamp(*r), clamp(*g), clamp(*b))
        })
        .collect();
    pixel_digest_canonical(&converted)
}

// ---------------------------------------------------------------------
// Receipt types
// ---------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Default)]
pub struct IfSource {
    pub device_claim: Option<String>,
    pub sensor_profile_digest: Option<String>,
    pub lens_profile_digest: Option<String>,
    pub timestamp: Option<String>,
    pub location_claim: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfPalette {
    pub hex: String,
    pub cf_id: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfReceipt {
    pub byte_digest: String,
    pub colorspace: String,
    pub format: String,
    pub height: i64,
    pub icc_digest: Option<String>,
    pub if_id: String,
    pub if_version: String,
    pub operation: String,
    pub orientation: i64,
    pub palette: Vec<IfPalette>,
    pub params: Option<String>,
    pub parent_if_id: Option<String>,
    pub pixel_digest: String,
    pub pixel_sample_stride: i64,
    pub receipt_digest: String,
    pub source: IfSource,
    pub width: i64,
}

#[derive(Debug, Clone)]
pub struct IfReceiptFields {
    pub pixels: Vec<(u8, u8, u8)>,
    pub byte_digest: String,
    pub format: String,
    pub width: i64,
    pub height: i64,
    pub colorspace: String,
    pub icc_digest: Option<String>,
    pub orientation: i64,
    pub palette: Vec<IfPalette>,
    pub source: IfSource,
    pub parent_if_id: Option<String>,
    pub operation: String,
    pub params: Option<String>,
}

impl IfReceiptFields {
    pub fn new(pixels: Vec<(u8, u8, u8)>, byte_digest: String, format: String, width: i64, height: i64) -> Self {
        IfReceiptFields {
            pixels,
            byte_digest,
            format,
            width,
            height,
            colorspace: "sRGB".to_string(),
            icc_digest: None,
            orientation: 1,
            palette: Vec::new(),
            source: IfSource::default(),
            parent_if_id: None,
            operation: "original".to_string(),
            params: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfVerifyResult {
    pub valid: bool,
    pub reason: String,
}

// ---------------------------------------------------------------------
// Canonical JSON (alphabetical keys, no whitespace)
// ---------------------------------------------------------------------

fn escape_json(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    for c in s.chars() {
        if c == '\\' {
            out.push('\\');
            out.push('\\');
        } else if c == '"' {
            out.push('\\');
            out.push('"');
        } else {
            out.push(c);
        }
    }
    out
}

fn js(s: &Option<String>) -> String {
    match s {
        None => "null".to_string(),
        Some(s) => {
            let escaped = escape_json(s);
            let mut out = String::with_capacity(escaped.len() + 2);
            out.push('"');
            out.push_str(&escaped);
            out.push('"');
            out
        }
    }
}

fn js_str(s: &str) -> String {
    let escaped = escape_json(s);
    let mut out = String::with_capacity(escaped.len() + 2);
    out.push('"');
    out.push_str(&escaped);
    out.push('"');
    out
}

fn jp(palette: &[IfPalette]) -> String {
    let entries: Vec<String> = palette
        .iter()
        .map(|e| format!("{{\"cf_id\":{},\"hex\":{}}}", js_str(&e.cf_id), js_str(&e.hex)))
        .collect();
    format!("[{}]", entries.join(","))
}

fn jsrc(s: &IfSource) -> String {
    format!(
        "{{\"device_claim\":{},\"lens_profile_digest\":{},\"location_claim\":{},\"sensor_profile_digest\":{},\"timestamp\":{}}}",
        js(&s.device_claim),
        js(&s.lens_profile_digest),
        js(&s.location_claim),
        js(&s.sensor_profile_digest),
        js(&s.timestamp)
    )
}

fn preimage_json(r: &IfReceipt) -> String {
    format!(
        "{{\"byte_digest\":{},\"colorspace\":{},\"format\":{},\"height\":{},\"icc_digest\":{},\"if_version\":{},\"operation\":{},\"orientation\":{},\"palette\":{},\"params\":{},\"parent_if_id\":{},\"pixel_digest\":{},\"pixel_sample_stride\":{},\"source\":{},\"width\":{}}}",
        js_str(&r.byte_digest),
        js_str(&r.colorspace),
        js_str(&r.format),
        r.height,
        js(&r.icc_digest),
        js_str(&r.if_version),
        js_str(&r.operation),
        r.orientation,
        jp(&r.palette),
        js(&r.params),
        js(&r.parent_if_id),
        js_str(&r.pixel_digest),
        r.pixel_sample_stride,
        jsrc(&r.source),
        r.width
    )
}

// ---------------------------------------------------------------------
// Receipt builder
// ---------------------------------------------------------------------

/// Build a complete IF-Protocol-1.0.0 receipt.
pub fn make_receipt(fields: IfReceiptFields) -> IfReceipt {
    let pd = pixel_digest_canonical(&fields.pixels);
    let partial = IfReceipt {
        byte_digest: fields.byte_digest,
        colorspace: fields.colorspace,
        format: fields.format,
        height: fields.height,
        icc_digest: fields.icc_digest,
        if_id: String::new(),
        if_version: "IF-CAPTURE-1.0.0".to_string(),
        operation: fields.operation,
        orientation: fields.orientation,
        palette: fields.palette,
        params: fields.params,
        parent_if_id: fields.parent_if_id,
        pixel_digest: pd.clone(),
        pixel_sample_stride: 1,
        receipt_digest: String::new(),
        source: fields.source,
        width: fields.width,
    };
    let rd = sha256_text(&preimage_json(&partial));
    let if_id = format!("IF-{}-{}", hash12(&pd), hash12(&rd));
    IfReceipt {
        if_id,
        receipt_digest: rd,
        ..partial
    }
}

/// Verify a receipt by recomputing if_id and receipt_digest.
pub fn verify_receipt(receipt: &IfReceipt) -> IfVerifyResult {
    let expected_rd = sha256_text(&preimage_json(receipt));
    let expected_id = format!("IF-{}-{}", hash12(&receipt.pixel_digest), hash12(&expected_rd));
    if receipt.receipt_digest != expected_rd {
        return IfVerifyResult { valid: false, reason: "receipt_digest mismatch".to_string() };
    }
    if receipt.if_id != expected_id {
        return IfVerifyResult { valid: false, reason: "if_id mismatch".to_string() };
    }
    IfVerifyResult { valid: true, reason: "ok".to_string() }
}

// ---------------------------------------------------------------------
// PERC-1.0: dHash + histogram sketch
// ---------------------------------------------------------------------

fn luminance(r: u8, g: u8, b: u8) -> f64 {
    0.299 * r as f64 + 0.587 * g as f64 + 0.114 * b as f64
}

/// Bilinear resize to tgt_w x tgt_h, returns flat grayscale floats.
fn resize_gray(pixels: &[(u8, u8, u8)], src_w: i64, src_h: i64, tgt_w: i64, tgt_h: i64) -> Vec<f64> {
    let mut result = Vec::with_capacity((tgt_w * tgt_h) as usize);
    for ty in 0..tgt_h {
        for tx in 0..tgt_w {
            let sx = (tx as f64 + 0.5) * src_w as f64 / tgt_w as f64 - 0.5;
            let sy = (ty as f64 + 0.5) * src_h as f64 / tgt_h as f64 - 0.5;
            let x0 = (sx.floor() as i64).max(0);
            let y0 = (sy.floor() as i64).max(0);
            let x1 = (x0 + 1).min(src_w - 1);
            let y1 = (y0 + 1).min(src_h - 1);
            let dx = sx - x0 as f64;
            let dy = sy - y0 as f64;
            let p = pixels[(y0 * src_w + x0) as usize];
            let q = pixels[(y0 * src_w + x1) as usize];
            let r = pixels[(y1 * src_w + x0) as usize];
            let s = pixels[(y1 * src_w + x1) as usize];
            let lp = luminance(p.0, p.1, p.2);
            let lq = luminance(q.0, q.1, q.2);
            let lr = luminance(r.0, r.1, r.2);
            let ls = luminance(s.0, s.1, s.2);
            let val = (1.0 - dx) * (1.0 - dy) * lp
                + dx * (1.0 - dy) * lq
                + (1.0 - dx) * dy * lr
                + dx * dy * ls;
            result.push(val);
        }
    }
    result
}

/// 64-bit dHash as 16 uppercase hex chars.
pub fn dhash16(pixels: &[(u8, u8, u8)], width: i64, height: i64) -> String {
    let thumb = resize_gray(pixels, width, height, 9, 8);
    let mut bits = Vec::with_capacity(64);
    for row in 0..8 {
        for col in 0..8 {
            let left = thumb[(row * 9 + col) as usize];
            let right = thumb[(row * 9 + col + 1) as usize];
            bits.push(if left > right { 1u8 } else { 0u8 });
        }
    }
    let hex_chars: Vec<char> = "0123456789ABCDEF".chars().collect();
    let mut result = String::with_capacity(16);
    for i in 0..16 {
        let nibble = bits[i * 4] * 8 + bits[i * 4 + 1] * 4 + bits[i * 4 + 2] * 2 + bits[i * 4 + 3];
        result.push(hex_chars[nibble as usize]);
    }
    result
}

/// 32-bit colour histogram sketch as 8 uppercase hex chars.
pub fn hist_sketch(pixels: &[(u8, u8, u8)]) -> String {
    let total = pixels.len();
    let mut counts = [0u32; 64];
    for (r, g, b) in pixels {
        let bucket = (*r as usize / 64) * 16 + (*g as usize / 64) * 4 + (*b as usize / 64);
        counts[bucket] += 1;
    }
    let mut indexed: Vec<(usize, u32)> = counts.iter().enumerate().map(|(i, c)| (i, *c)).collect();
    indexed.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    let top4 = &indexed[0..4];

    let count_tier = |cnt: u32| -> u8 {
        let pct = if total > 0 { cnt as f64 / total as f64 } else { 0.0 };
        if pct > 0.5 { 3 } else if pct > 0.25 { 2 } else if pct > 0.1 { 1 } else { 0 }
    };

    let hex_chars: Vec<char> = "0123456789ABCDEF".chars().collect();
    let mut result = String::with_capacity(8);
    for &(idx, cnt) in top4 {
        let tier = count_tier(cnt);
        let high_nibble = idx / 4;
        let low_nibble = (idx % 4) * 4 + tier as usize;
        result.push(hex_chars[high_nibble]);
        result.push(hex_chars[low_nibble]);
    }
    result
}

/// PERC-ID string, or None if image has fewer than 9 pixels.
pub fn make_perc_id(pixels: &[(u8, u8, u8)], width: i64, height: i64) -> Option<String> {
    if pixels.len() < 9 {
        return None;
    }
    let dh = dhash16(pixels, width, height);
    let hs = hist_sketch(pixels);
    Some(format!("PERC-{}-{}", dh, hs))
}

/// Hamming distance between two DHASH16 hex strings.
pub fn hamming(a: &str, b: &str) -> u32 {
    a.chars()
        .zip(b.chars())
        .map(|(ca, cb)| {
            let va = ca.to_digit(16).unwrap_or(0);
            let vb = cb.to_digit(16).unwrap_or(0);
            (va ^ vb).count_ones()
        })
        .sum()
}
