import XCTest
@testable import IFReceipt

final class IFReceiptTests: XCTestCase {
    func colorfulPixels() -> [(r: Int, g: Int, b: Int)] {
        let c: [(r:Int,g:Int,b:Int)] = [(220,50,50),(50,180,80),(50,90,220),(240,200,40)]
        return (0..<16).map { i in c[(i%4+i/4)%4] }
    }
    func testCanonicalPixelDigestVector() {
        XCTAssertEqual(pixelDigestCanonical(colorfulPixels()),
            "sha256:2a40854db5a950bbdc8d921620a2ee8074fb8b950102150b6be7b10990e1ddb6")
    }
    func testHash12() {
        XCTAssertEqual(hash12("sha256:2a40854db5a950bbdc8d921620a2ee8074fb8b950102150b6be7b10990e1ddb6"), "2A40854DB5A9")
    }
    func testIfIdFormat() {
        let id = "IF-\(hash12("sha256:2a40854db5a950bbdc8d921620a2ee8074fb8b950102150b6be7b10990e1ddb6"))-\(hash12("sha256:" + String(repeating: "0", count: 64)))"
        XCTAssertEqual(id.count, 28); XCTAssertTrue(id.hasPrefix("IF-"))
    }
    func testMakeReceiptIsValid() {
        let r = makeReceipt(IFReceiptFields(pixels:colorfulPixels(),
            byteDigest:"sha256:" + String(repeating: "0", count: 64), format:"png", width:4, height:4))
        XCTAssertTrue(verifyReceipt(r).valid)
    }
    func testSamePixelsSameDigest() {
        let p = colorfulPixels()
        XCTAssertEqual(pixelDigestCanonical(p), pixelDigestCanonical(p))
    }
    func testDifferentPixelsDifferentDigest() {
        XCTAssertNotEqual(
            pixelDigestCanonical([(r:Int,g:Int,b:Int)](repeating:(255,0,0),count:4)),
            pixelDigestCanonical([(r:Int,g:Int,b:Int)](repeating:(0,0,255),count:4)))
    }
    func testTamperedReceiptDetected() {
        let r = makeReceipt(IFReceiptFields(pixels:colorfulPixels(),
            byteDigest:"sha256:" + String(repeating: "0", count: 64), format:"png", width:4, height:4))
        let t = IFReceipt(byte_digest:r.byte_digest, colorspace:r.colorspace,
            format:r.format, height:r.height, icc_digest:r.icc_digest,
            if_id:"IF-000000000000-000000000000", if_version:r.if_version,
            operation:r.operation, orientation:r.orientation, palette:r.palette,
            params:r.params, parent_if_id:r.parent_if_id, pixel_digest:r.pixel_digest,
            pixel_sample_stride:r.pixel_sample_stride, receipt_digest:r.receipt_digest,
            source:r.source, width:r.width)
        XCTAssertFalse(verifyReceipt(t).valid)
    }
    func testParentIfIdNullForOriginal() {
        let r = makeReceipt(IFReceiptFields(pixels:[(255,0,0)],
            byteDigest:"sha256:" + String(repeating: "0", count: 64), format:"png", width:1, height:1))
        XCTAssertNil(r.parent_if_id)
    }
    func testIfVersion() {
        let r = makeReceipt(IFReceiptFields(pixels:[(255,0,0)],
            byteDigest:"sha256:" + String(repeating: "0", count: 64), format:"png", width:1, height:1))
        XCTAssertEqual(r.if_version, "IF-CAPTURE-1.0.0")
    }
}
