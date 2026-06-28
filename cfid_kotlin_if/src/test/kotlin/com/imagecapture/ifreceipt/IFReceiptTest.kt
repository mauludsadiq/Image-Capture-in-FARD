package com.imagecapture.ifreceipt

import kotlin.system.exitProcess

private var passed = 0
private var failed = 0

private fun check(name: String, ok: Boolean) {
    if (ok) { println("  ok  $name"); passed++ }
    else    { println("  FAIL $name"); failed++ }
}

private fun colorfulPixels(): List<Triple<Int, Int, Int>> {
    val colors = listOf(
        Triple(220,50,50), Triple(50,180,80),
        Triple(50,90,220), Triple(240,200,40)
    )
    return (0 until 16).map { i -> colors[(i % 4 + i / 4) % 4] }
}

fun main() {
    check("canonical pixel_digest vector: colorful 4x4 PNG",
        pixelDigestCanonical(colorfulPixels()) ==
        "sha256:2a40854db5a950bbdc8d921620a2ee8074fb8b950102150b6be7b10990e1ddb6"
    )

    check("hash12 extracts 12 uppercase hex chars",
        hash12("sha256:2a40854db5a950bbdc8d921620a2ee8074fb8b950102150b6be7b10990e1ddb6") == "2A40854DB5A9"
    )

    val pd = "sha256:2a40854db5a950bbdc8d921620a2ee8074fb8b950102150b6be7b10990e1ddb6"
    val rd = "sha256:" + "0".repeat(64)
    val testId = "IF-" + hash12(pd) + "-" + hash12(rd)
    check("IF-ID format: 28 chars, starts with IF-", testId.length == 28 && testId.startsWith("IF-"))

    val fields = IFReceiptFields(
        pixels = colorfulPixels(), byteDigest = "sha256:" + "0".repeat(64),
        format = "png", width = 4, height = 4
    )
    val receipt = makeReceipt(fields)
    check("makeReceipt produces a valid receipt", verifyReceipt(receipt).valid)

    check("same pixels -> same pixel_digest",
        pixelDigestCanonical(colorfulPixels()) == pixelDigestCanonical(colorfulPixels())
    )

    val red = List(4) { Triple(255, 0, 0) }
    val blue = List(4) { Triple(0, 0, 255) }
    check("different pixels -> different pixel_digest", pixelDigestCanonical(red) != pixelDigestCanonical(blue))

    val tampered = receipt.copy(if_id = "IF-000000000000-000000000000")
    check("tampered if_id is detected", !verifyReceipt(tampered).valid)

    check("parent_if_id is null for original", receipt.parent_if_id == null)

    check("if_version is IF-CAPTURE-1.0.0", receipt.if_version == "IF-CAPTURE-1.0.0")

    println("\n$passed passed, $failed failed")
    if (failed > 0) exitProcess(1)
}
