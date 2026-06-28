# IFReceipt (Go)

IF-Protocol-1.0.0 image identity in Go.

Pure Go, zero dependencies (crypto/sha256 from the standard library).
This is the third language port of IF-Protocol, after Swift and Kotlin,
conformant with SPEC-IF-1.0.md.

## Build and test

    go test ./...
    go vet ./...

9/9 tests pass including the canonical SPEC-IF-1.0.md Appendix A.1
pixel_digest vector.

## Status

Part of Image Capture in FARD, Roadmap Phase B.3. Useful for
server-side receipt verification pipelines (no Apple/Google accounts
required). Not yet published as a Go module.
