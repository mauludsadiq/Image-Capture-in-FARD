# ifreceipt (Python)

IF-Protocol-1.0.0 and PERC-1.0 image identity in Python.

Zero runtime dependencies (hashlib from the standard library).
Fifth language port after Swift, Kotlin, Go, and TypeScript.
Conformant with SPEC-IF-1.0.md and SPEC-PERC-1.0.md.

## Install

    pip install ifreceipt   # not yet published -- install from source:
    pip install -e cfid_python_if/

## Usage

    from ifreceipt import pixel_digest_canonical, make_receipt, verify_receipt
    from ifreceipt import dhash16, hist_sketch, make_perc_id, hamming

    pixels = [(220,50,50),(50,180,80),(50,90,220),(240,200,40)] * 4
    digest = pixel_digest_canonical(pixels)
    # sha256:2a40854db5a950bbdc8d921620a2ee8074fb8b950102150b6be7b10990e1ddb6

    receipt = make_receipt({
        "pixels": pixels,
        "byte_digest": "sha256:" + "0"*64,
        "format": "png", "width": 4, "height": 4
    })
    print(receipt["if_id"])   # IF-2A40854DB5A9-...

    perc = make_perc_id(pixels, 4, 4)
    print(perc)               # PERC-...-...

## Test

    python3 -m pytest tests/ -v

## Status

Part of Image Capture in FARD, Phase B (language ports).
