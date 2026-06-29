# IF-Protocol and C2PA: a comparison

## What is C2PA?

C2PA (Coalition for Content Provenance and Authenticity) is an industry
standard led by Adobe, Microsoft, Google, Sony, and others. It defines
a manifest format embedded in media files that records the origin,
editing history, and signing chain of an asset. C2PA is the technical
foundation of "Content Credentials" -- the provenance badges appearing
in Adobe products, Bing Image Search, and some camera hardware.

C2PA is a large standard: manifests, claims, assertions, signed
certificates, hardware attestation, JUMBF embedding, COSE signing.
It is designed for enterprise workflows and content platforms.

## What is IF-Protocol?

IF-Protocol-1.0.0 is a lightweight, offline-first, pixel-level image
identity protocol. It derives a deterministic cryptographic identity
(IF-ID) from the exact pixel sequence of an image, and a perceptual
identity (PERC-ID) from structural and colour features. Both identities
are computed from the image data alone -- no signing keys, no
certificates, no network.

## Where they agree

Both systems are motivated by the same problem: in an era of generative
AI and cheap manipulation, it matters whether an image is what it claims
to be, and where it came from.

Both record:
- A representation of the image content
- Metadata about capture device and timestamp (as claims)
- A chain linking edits back to an origin

## Where they differ

| | IF-Protocol | C2PA |
|---|---|---|
| Identity basis | Pixel data (SHA-256) | Signed manifest (X.509 certificates) |
| Trust model | Content-derived, no keys required | Certificate chain from trusted authority |
| Perceptual identity | Yes (PERC-ID: dHash + histogram) | No |
| Offline verification | Yes -- verify with no network | Requires certificate validation |
| Hardware requirement | None | Hardware attestation optional but recommended |
| Embedding | Sidecar JSON receipts | Embedded in file (JUMBF) |
| Complexity | Single-file spec, ~200 lines core | Large multi-document standard |
| Language ports | FARD, Swift, Kotlin, Go, TypeScript | Many (C2PA-rs is the reference) |
| Conformance suite | if_vectors.json (canonical vectors) | C2PA conformance test suite |
| Capture bridge | imagesnap + sips (MacBook) | Camera hardware (Sony, Leica, etc.) |
| Edit chains | parent_if_id linkage | C2PA ingredient assertions |
| AI-generated flag | Not yet | Yes (ai_generated_content assertion) |
| License | Open | Open (specification), implementations vary |

## The fundamental difference in trust

C2PA answers: "A trusted authority (camera manufacturer, platform, or
signing service) asserts this image was captured or edited in a specific
way."

IF-Protocol answers: "These are the exact pixels. Here is their
cryptographic identity. You can verify this yourself, right now, with
no keys and no network."

C2PA trust is delegated to a certificate chain. If the signing key is
compromised, or the hardware attestation is bypassed, the provenance
claim is void. The signature can be valid while the content is false.

IF-Protocol trust is derived from the content itself. The IF-ID is a
commitment to the exact pixel sequence. If the pixels change, the IF-ID
changes. There is no key to compromise. The limitation is the inverse:
IF-Protocol cannot assert who captured the image, only what the pixels
are.

## They are complementary, not competing

A complete provenance system for the AI era needs both:

- IF-Protocol provides the pixel-level commitment and perceptual
 identity. It is verifiable by anyone, offline, forever.
- C2PA provides the signing chain and hardware attestation. It asserts
 who captured or processed the image.

An image with both a C2PA manifest and an IF-Protocol receipt gives:
- "A verified Sony camera captured this" (C2PA)
- "These are the exact pixels from that capture, unchanged" (IF-Protocol)
- "This image is perceptually similar to these other images" (PERC-ID)

## Using IF-Protocol alongside C2PA

IF-Protocol receipts are plain JSON. They can be:
1. Stored as sidecar files alongside C2PA-signed assets
2. Embedded as a custom C2PA assertion (c2pa.unknown or a custom label)
3. Used independently where C2PA infrastructure is unavailable

The IF-ID and PERC-ID are short enough (28-30 chars each) to include
in any metadata field, database record, or blockchain transaction.

## When to use IF-Protocol without C2PA

- Offline or air-gapped environments
- Personal archiving and journalism (no enterprise PKI)
- Verification of images where C2PA was not present at capture
- Near-duplicate detection across large image sets (PERC-ID)
- Any context where pixel-level commitment matters more than identity
 of the signer

## When C2PA is the right tool

- Publishing platforms that need to display provenance badges
- Camera manufacturers building hardware attestation
- Enterprise content pipelines with existing PKI
- Contexts where "who signed this" matters more than "what are the pixels"

## Summary

IF-Protocol is to C2PA what a cryptographic hash is to a digital
signature: simpler, more universally verifiable, content-derived, and
requiring no trust infrastructure -- at the cost of not asserting
identity of the creator. Both have a role. For capture-native,
offline-first, multi-language image provenance, IF-Protocol fills a
gap that C2PA was not designed to fill.
