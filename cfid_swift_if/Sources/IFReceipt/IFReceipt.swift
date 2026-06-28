import Foundation
import CryptoKit

public struct IFPalette: Codable, Equatable {
    public let hex: String
    public let cf_id: String
    public init(hex: String, cf_id: String) { self.hex = hex; self.cf_id = cf_id }
}

public struct IFSource: Codable, Equatable {
    public let device_claim: String?
    public let sensor_profile_digest: String?
    public let lens_profile_digest: String?
    public let timestamp: String?
    public let location_claim: String?
    public init(device_claim: String? = nil, sensor_profile_digest: String? = nil,
                lens_profile_digest: String? = nil, timestamp: String? = nil,
                location_claim: String? = nil) {
        self.device_claim = device_claim; self.sensor_profile_digest = sensor_profile_digest
        self.lens_profile_digest = lens_profile_digest; self.timestamp = timestamp
        self.location_claim = location_claim
    }
    public static let empty = IFSource()
}

public struct IFReceipt: Codable, Equatable {
    public let byte_digest: String
    public let colorspace: String
    public let format: String
    public let height: Int
    public let icc_digest: String?
    public let if_id: String
    public let if_version: String
    public let operation: String
    public let orientation: Int
    public let palette: [IFPalette]
    public let params: String?
    public let parent_if_id: String?
    public let pixel_digest: String
    public let pixel_sample_stride: Int
    public let receipt_digest: String
    public let source: IFSource
    public let width: Int
}

public func sha256Text(_ input: String) -> String {
    let d = SHA256.hash(data: Data(input.utf8))
    return "sha256:" + d.map { String(format: "%02x", $0) }.joined()
}

public func sha256Bytes(_ data: Data) -> String {
    let d = SHA256.hash(data: data)
    return "sha256:" + d.map { String(format: "%02x", $0) }.joined()
}

public func hash12(_ digest: String) -> String {
    String(digest.dropFirst(7).prefix(12)).uppercased()
}

public func pixelDigestCanonical(_ pixels: [(r: Int, g: Int, b: Int)]) -> String {
    var s = ""; s.reserveCapacity(pixels.count * 12)
    for p in pixels { s += "\(p.r),\(p.g),\(p.b)," }
    return sha256Text(s)
}

public func pixelDigestFromFloats(_ pixels: [(r: Double, g: Double, b: Double)]) -> String {
    pixelDigestCanonical(pixels.map {(
        r: min(255, max(0, Int($0.r * 255.0 + 0.5))),
        g: min(255, max(0, Int($0.g * 255.0 + 0.5))),
        b: min(255, max(0, Int($0.b * 255.0 + 0.5))))})
}

private func js(_ s: String?) -> String {
    guard let s = s else { return "null" }
    return "\"" + s.replacingOccurrences(of: "\\", with: "\\\\")
                    .replacingOccurrences(of: "\"", with: "\\\"") + "\""
}
private func ji(_ n: Int) -> String { "\(n)" }
private func jp(_ p: [IFPalette]) -> String {
    "[" + p.map { "{\"cf_id\":\(js($0.cf_id)),\"hex\":\(js($0.hex))}" }.joined(separator:",") + "]"
}
private func jsrc(_ s: IFSource) -> String {
    "{\"device_claim\":\(js(s.device_claim)),\"lens_profile_digest\":\(js(s.lens_profile_digest)),\"location_claim\":\(js(s.location_claim)),\"sensor_profile_digest\":\(js(s.sensor_profile_digest)),\"timestamp\":\(js(s.timestamp))}"
}
private func preimageJSON(_ r: IFReceipt) -> String {
    "{\"byte_digest\":\(js(r.byte_digest)),\"colorspace\":\(js(r.colorspace)),\"format\":\(js(r.format)),\"height\":\(ji(r.height)),\"icc_digest\":\(js(r.icc_digest)),\"if_version\":\(js(r.if_version)),\"operation\":\(js(r.operation)),\"orientation\":\(ji(r.orientation)),\"palette\":\(jp(r.palette)),\"params\":\(js(r.params)),\"parent_if_id\":\(js(r.parent_if_id)),\"pixel_digest\":\(js(r.pixel_digest)),\"pixel_sample_stride\":\(ji(r.pixel_sample_stride)),\"source\":\(jsrc(r.source)),\"width\":\(ji(r.width))}"
}

public struct IFReceiptFields {
    public var pixels: [(r: Int, g: Int, b: Int)]
    public var byteDigest: String
    public var format: String
    public var width: Int
    public var height: Int
    public var colorspace: String
    public var iccDigest: String?
    public var orientation: Int
    public var palette: [IFPalette]
    public var source: IFSource
    public var parentIfId: String?
    public var operation: String
    public var params: String?
    public init(pixels: [(r: Int, g: Int, b: Int)], byteDigest: String,
                format: String, width: Int, height: Int,
                colorspace: String = "sRGB", iccDigest: String? = nil,
                orientation: Int = 1, palette: [IFPalette] = [],
                source: IFSource = .empty, parentIfId: String? = nil,
                operation: String = "original", params: String? = nil) {
        self.pixels=pixels; self.byteDigest=byteDigest; self.format=format
        self.width=width; self.height=height; self.colorspace=colorspace
        self.iccDigest=iccDigest; self.orientation=orientation; self.palette=palette
        self.source=source; self.parentIfId=parentIfId; self.operation=operation; self.params=params
    }
}

public func makeReceipt(_ fields: IFReceiptFields) -> IFReceipt {
    let pd = pixelDigestCanonical(fields.pixels)
    let partial = IFReceipt(byte_digest:fields.byteDigest, colorspace:fields.colorspace,
        format:fields.format, height:fields.height, icc_digest:fields.iccDigest,
        if_id:"", if_version:"IF-CAPTURE-1.0.0", operation:fields.operation,
        orientation:fields.orientation, palette:fields.palette, params:fields.params,
        parent_if_id:fields.parentIfId, pixel_digest:pd, pixel_sample_stride:1,
        receipt_digest:"", source:fields.source, width:fields.width)
    let rd = sha256Text(preimageJSON(partial))
    let ifId = "IF-\(hash12(pd))-\(hash12(rd))"
    return IFReceipt(byte_digest:partial.byte_digest, colorspace:partial.colorspace,
        format:partial.format, height:partial.height, icc_digest:partial.icc_digest,
        if_id:ifId, if_version:partial.if_version, operation:partial.operation,
        orientation:partial.orientation, palette:partial.palette, params:partial.params,
        parent_if_id:partial.parent_if_id, pixel_digest:pd, pixel_sample_stride:1,
        receipt_digest:rd, source:partial.source, width:partial.width)
}

public struct IFVerifyResult { public let valid: Bool; public let reason: String }

public func verifyReceipt(_ receipt: IFReceipt) -> IFVerifyResult {
    let expectedRd = sha256Text(preimageJSON(receipt))
    let expectedId = "IF-\(hash12(receipt.pixel_digest))-\(hash12(expectedRd))"
    if receipt.receipt_digest != expectedRd { return IFVerifyResult(valid:false, reason:"receipt_digest mismatch") }
    if receipt.if_id != expectedId { return IFVerifyResult(valid:false, reason:"if_id mismatch") }
    return IFVerifyResult(valid:true, reason:"ok")
}
