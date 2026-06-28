// IFReceipt.kt -- IF-Protocol-1.0.0 image identity in Kotlin.
// Pure Kotlin, zero third-party dependencies (java.security.MessageDigest).
// Conformant with SPEC-IF-1.0.md.

package com.imagecapture.ifreceipt

import java.security.MessageDigest
import kotlin.math.min
import kotlin.math.max

data class IFPalette(val hex: String, val cf_id: String)

data class IFSource(
    val device_claim: String? = null,
    val sensor_profile_digest: String? = null,
    val lens_profile_digest: String? = null,
    val timestamp: String? = null,
    val location_claim: String? = null
) { companion object { val empty = IFSource() } }

data class IFReceipt(
    val byte_digest: String, val colorspace: String, val format: String,
    val height: Int, val icc_digest: String?, val if_id: String,
    val if_version: String, val operation: String, val orientation: Int,
    val palette: List<IFPalette>, val params: String?, val parent_if_id: String?,
    val pixel_digest: String, val pixel_sample_stride: Int,
    val receipt_digest: String, val source: IFSource, val width: Int
)

data class IFVerifyResult(val valid: Boolean, val reason: String)

fun sha256Text(input: String): String {
    val digest = MessageDigest.getInstance("SHA-256").digest(input.toByteArray(Charsets.UTF_8))
    return "sha256:" + digest.joinToString("") { "%02x".format(it) }
}

fun sha256Bytes(data: ByteArray): String {
    val digest = MessageDigest.getInstance("SHA-256").digest(data)
    return "sha256:" + digest.joinToString("") { "%02x".format(it) }
}

fun hash12(digest: String): String = digest.drop(7).take(12).uppercase()

fun pixelDigestCanonical(pixels: List<Triple<Int, Int, Int>>): String {
    val sb = StringBuilder(pixels.size * 12)
    for ((r, g, b) in pixels) { sb.append(r).append(',').append(g).append(',').append(b).append(',') }
    return sha256Text(sb.toString())
}

fun pixelDigestFromFloats(pixels: List<Triple<Double, Double, Double>>): String =
    pixelDigestCanonical(pixels.map { (r, g, b) ->
        Triple(min(255,max(0,(r*255.0+0.5).toInt())), min(255,max(0,(g*255.0+0.5).toInt())), min(255,max(0,(b*255.0+0.5).toInt())))
    })

private fun js(s: String?): String {
    if (s == null) return "null"
    val e = s.replace("\\", "\\\\").replace("\"", "\\\"")
    return "\"" + e + "\""
}
private fun ji(n: Int) = n.toString()
private fun jp(p: List<IFPalette>): String =
    "[" + p.joinToString(",") { "{" + "\"cf_id\":" + js(it.cf_id) + ",\"hex\":" + js(it.hex) + "}" } + "]"
private fun jsrc(s: IFSource): String =
    "{" + "\"device_claim\":" + js(s.device_claim) + ",\"lens_profile_digest\":" + js(s.lens_profile_digest) +
    ",\"location_claim\":" + js(s.location_claim) + ",\"sensor_profile_digest\":" + js(s.sensor_profile_digest) +
    ",\"timestamp\":" + js(s.timestamp) + "}"
private fun preimageJSON(r: IFReceipt): String =
    "{" + "\"byte_digest\":" + js(r.byte_digest) + ",\"colorspace\":" + js(r.colorspace) +
    ",\"format\":" + js(r.format) + ",\"height\":" + ji(r.height) + ",\"icc_digest\":" + js(r.icc_digest) +
    ",\"if_version\":" + js(r.if_version) + ",\"operation\":" + js(r.operation) +
    ",\"orientation\":" + ji(r.orientation) + ",\"palette\":" + jp(r.palette) +
    ",\"params\":" + js(r.params) + ",\"parent_if_id\":" + js(r.parent_if_id) +
    ",\"pixel_digest\":" + js(r.pixel_digest) + ",\"pixel_sample_stride\":" + ji(r.pixel_sample_stride) +
    ",\"source\":" + jsrc(r.source) + ",\"width\":" + ji(r.width) + "}"

data class IFReceiptFields(
    val pixels: List<Triple<Int, Int, Int>>, val byteDigest: String,
    val format: String, val width: Int, val height: Int,
    val colorspace: String = "sRGB", val iccDigest: String? = null,
    val orientation: Int = 1, val palette: List<IFPalette> = emptyList(),
    val source: IFSource = IFSource.empty, val parentIfId: String? = null,
    val operation: String = "original", val params: String? = null
)

fun makeReceipt(fields: IFReceiptFields): IFReceipt {
    val pd = pixelDigestCanonical(fields.pixels)
    val partial = IFReceipt(
        byte_digest=fields.byteDigest, colorspace=fields.colorspace, format=fields.format,
        height=fields.height, icc_digest=fields.iccDigest, if_id="",
        if_version="IF-CAPTURE-1.0.0", operation=fields.operation, orientation=fields.orientation,
        palette=fields.palette, params=fields.params, parent_if_id=fields.parentIfId,
        pixel_digest=pd, pixel_sample_stride=1, receipt_digest="",
        source=fields.source, width=fields.width
    )
    val rd = sha256Text(preimageJSON(partial))
    val ifId = "IF-" + hash12(pd) + "-" + hash12(rd)
    return partial.copy(if_id=ifId, receipt_digest=rd)
}

fun verifyReceipt(receipt: IFReceipt): IFVerifyResult {
    val expectedRd = sha256Text(preimageJSON(receipt))
    val expectedId = "IF-" + hash12(receipt.pixel_digest) + "-" + hash12(expectedRd)
    if (receipt.receipt_digest != expectedRd) return IFVerifyResult(false, "receipt_digest mismatch")
    if (receipt.if_id != expectedId) return IFVerifyResult(false, "if_id mismatch")
    return IFVerifyResult(true, "ok")
}
