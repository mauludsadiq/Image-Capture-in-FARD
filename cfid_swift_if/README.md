# IFReceipt (Swift)

IF-Protocol-1.0.0 image identity in Swift.

Pure Swift, zero third-party dependencies (CryptoKit SHA-256, part of
the Apple SDK). This is the first language port of IF-Protocol, after
the FARD reference implementation, and the direct prerequisite for an
iOS/macOS capture app (Roadmap Phase D.1).

Conformant with SPEC-IF-1.0.md: same pixel_digest_canonical preimage,
same canonical JSON receipt structure, same IF-ID derivation.

## Key functions

    pixelDigestCanonical(_ pixels: [(r: Int, g: Int, b: Int)]) -> String
    pixelDigestFromFloats(_ pixels: [(r: Double, g: Double, b: Double)]) -> String
    makeReceipt(_ fields: IFReceiptFields) -> IFReceipt
    verifyReceipt(_ receipt: IFReceipt) -> IFVerifyResult
    hash12(_ digest: String) -> String
    sha256Text(_ input: String) -> String
    sha256Bytes(_ data: Data) -> String

## Build and test

    swift test

9/9 tests pass, including the canonical pixel_digest vector from
SPEC-IF-1.0.md Appendix A.1:

    pixelDigestCanonical(colorful4x4Pixels) ==
      "sha256:2a40854db5a950bbdc8d921620a2ee8074fb8b950102150b6be7b10990e1ddb6"

## Usage (iOS/macOS capture)

    import IFReceipt

    // At moment of capture (AVFoundation CMSampleBuffer -> pixel array)
    let pixels: [(r: Int, g: Int, b: Int)] = decodePixels(sampleBuffer)
    let byteDigest = sha256Bytes(rawJpegData)
    let fields = IFReceiptFields(
        pixels: pixels,
        byteDigest: byteDigest,
        format: "heic",
        width: 4032,
        height: 3024,
        source: IFSource(
            device_claim: "Apple iPhone 15 Pro",
            timestamp: ISO8601DateFormatter().string(from: Date())
        )
    )
    let receipt = makeReceipt(fields)
    // receipt.if_id: "IF-XXXXXXXXXXXX-XXXXXXXXXXXX"
    // receipt is Codable -- save as JSON sidecar or embed in image

## Status

Part of Image Capture in FARD, Roadmap Phase B.1. Not yet published
as a Swift package. Intended for use in Phase D.1 (iOS capture app).
