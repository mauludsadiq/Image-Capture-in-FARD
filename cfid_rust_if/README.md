# ifreceipt (Rust)

IF-Protocol-1.0.0 and PERC-1.0 image identity in Rust.

Single dependency (sha2, for SHA-256). Sixth language port after
Swift, Kotlin, Go, TypeScript, and Python. Conformant with
SPEC-IF-1.0.md and SPEC-PERC-1.0.md.

## Build and test

    cargo test

21/21 tests pass, including the canonical SPEC-IF-1.0.md Appendix A.1
pixel_digest vector.

## Usage

    use ifreceipt::{pixel_digest_canonical, make_receipt, IfReceiptFields, make_perc_id};

    let pixels = vec![(220u8,50u8,50u8), (50,180,80), (50,90,220), (240,200,40)];
    let digest = pixel_digest_canonical(&pixels);

    let fields = IfReceiptFields::new(
        pixels.clone(),
        "sha256:".to_string() + &"0".repeat(64),
        "png".to_string(), 4, 4
    );
    let receipt = make_receipt(fields);
    println!("{}", receipt.if_id);

    let perc = make_perc_id(&pixels, 2, 2);

## Status

Part of Image Capture in FARD, Phase B (language ports).
