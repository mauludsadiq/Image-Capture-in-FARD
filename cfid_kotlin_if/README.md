# IFReceipt (Kotlin)

IF-Protocol-1.0.0 image identity in Kotlin.

Pure Kotlin, zero third-party dependencies (java.security.MessageDigest
from the JDK for SHA-256). This is the second language port of
IF-Protocol, after Swift, and the direct prerequisite for an Android
capture app (Roadmap Phase D.2).

Conformant with SPEC-IF-1.0.md: same pixel_digest_canonical preimage,
same canonical JSON receipt structure, same IF-ID derivation. The
canonical pixel_digest vector from SPEC-IF-1.0.md Appendix A.1 passes.

## Build and test

    cd cfid_kotlin_if
    kotlinc src/main/kotlin/com/imagecapture/ifreceipt/IFReceipt.kt \
            src/test/kotlin/com/imagecapture/ifreceipt/IFReceiptTest.kt \
            -include-runtime -d test.jar
    java -cp test.jar com.imagecapture.ifreceipt.IFReceiptTestKt

9/9 tests pass.

## Status

Part of Image Capture in FARD, Roadmap Phase B.2. Not yet packaged
as an Android .aar; a Gradle build can be added when an Android
module is started (Phase D.2).
