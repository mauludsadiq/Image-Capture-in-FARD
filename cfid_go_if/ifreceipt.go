// Package ifreceipt implements IF-Protocol-1.0.0 image identity.
//
// Pure Go, zero dependencies (crypto/sha256 from the standard library).
// This is the third language port of IF-Protocol, after Swift and Kotlin,
// conformant with SPEC-IF-1.0.md.
package ifreceipt

import (
   "crypto/sha256"
   "fmt"
   "strings"
)

// IFPalette is a dominant colour entry in an image receipt.
type IFPalette struct {
   Hex   string `json:"hex"`
   CfID  string `json:"cf_id"`
}

// IFSource holds unverified capture device claims.
type IFSource struct {
   DeviceClaim          *string `json:"device_claim"`
   SensorProfileDigest  *string `json:"sensor_profile_digest"`
   LensProfileDigest    *string `json:"lens_profile_digest"`
   Timestamp            *string `json:"timestamp"`
   LocationClaim        *string `json:"location_claim"`
}

// EmptySource returns an IFSource with all fields nil.
func EmptySource() IFSource { return IFSource{} }

// IFReceipt is a complete IF-Protocol-1.0.0 image receipt.
type IFReceipt struct {
   ByteDigest        string      `json:"byte_digest"`
   Colorspace        string      `json:"colorspace"`
   Format            string      `json:"format"`
   Height            int         `json:"height"`
   ICCDigest         *string     `json:"icc_digest"`
   IFID              string      `json:"if_id"`
   IFVersion         string      `json:"if_version"`
   Operation         string      `json:"operation"`
   Orientation       int         `json:"orientation"`
   Palette           []IFPalette `json:"palette"`
   Params            *string     `json:"params"`
   ParentIFID        *string     `json:"parent_if_id"`
   PixelDigest       string      `json:"pixel_digest"`
   PixelSampleStride int         `json:"pixel_sample_stride"`
   ReceiptDigest     string      `json:"receipt_digest"`
   Source            IFSource    `json:"source"`
   Width             int         `json:"width"`
}

// IFVerifyResult is the result of verifying a receipt.
type IFVerifyResult struct {
   Valid  bool
   Reason string
}

// IFReceiptFields are the inputs to MakeReceipt.
type IFReceiptFields struct {
   Pixels      [][3]int
   ByteDigest  string
   Format      string
   Width       int
   Height      int
   Colorspace  string
   ICCDigest   *string
   Orientation int
   Palette     []IFPalette
   Source      IFSource
   ParentIFID  *string
   Operation   string
   Params      *string
}

// DefaultFields returns an IFReceiptFields with sensible defaults applied.
func DefaultFields(pixels [][3]int, byteDigest, format string, width, height int) IFReceiptFields {
   return IFReceiptFields{
   Pixels:      pixels,
   ByteDigest:  byteDigest,
   Format:      format,
   Width:       width,
   Height:      height,
   Colorspace:  "sRGB",
   Orientation: 1,
   Palette:     []IFPalette{},
   Source:      EmptySource(),
   Operation:   "original",
   }
}

// SHA256Text computes SHA-256 of a string and returns "sha256:<hex>".
func SHA256Text(input string) string {
   sum := sha256.Sum256([]byte(input))
   return "sha256:" + fmt.Sprintf("%x", sum)
}

// SHA256Bytes computes SHA-256 of raw bytes and returns "sha256:<hex>".
func SHA256Bytes(data []byte) string {
   sum := sha256.Sum256(data)
   return "sha256:" + fmt.Sprintf("%x", sum)
}

// Hash12 extracts the first 12 uppercase hex chars from a "sha256:..." digest.
func Hash12(digest string) string {
   return strings.ToUpper(digest[7:19])
}

// PixelDigestCanonical computes the canonical pixel digest per SPEC-IF-1.0.md
// section 3. Preimage: "r0,g0,b0,r1,g1,b1,...,rN,gN,bN," (trailing comma).
func PixelDigestCanonical(pixels [][3]int) string {
   var sb strings.Builder
   sb.Grow(len(pixels) * 12)
   for _, p := range pixels {
   fmt.Fprintf(&sb, "%d,%d,%d,", p[0], p[1], p[2])
   }
   return SHA256Text(sb.String())
}

// PixelDigestFromFloats converts float [0,1] pixels and computes pixel digest.
func PixelDigestFromFloats(pixels [][3]float64) string {
   converted := make([][3]int, len(pixels))
   for i, p := range pixels {
   clamp := func(v float64) int {
   n := int(v*255.0 + 0.5)
   if n < 0 { return 0 }
   if n > 255 { return 255 }
   return n
   }
   converted[i] = [3]int{clamp(p[0]), clamp(p[1]), clamp(p[2])}
   }
   return PixelDigestCanonical(converted)
}

// Canonical JSON helpers (alphabetical key order, no whitespace).
func jsStr(s *string) string {
   if s == nil { return "null" }
   e := strings.ReplaceAll(*s, "\\", "\\\\")
   e = strings.ReplaceAll(e, "\"", "\\\"")
   return "\"" + e + "\""
}
func jsInt(n int) string { return fmt.Sprintf("%d", n) }
func jsPalette(p []IFPalette) string {
   parts := make([]string, len(p))
   for i, e := range p {
   cf := e.CfID; hex := e.Hex
   parts[i] = "{\"cf_id\":" + jsStr(&cf) + ",\"hex\":" + jsStr(&hex) + "}"
   }
   return "[" + strings.Join(parts, ",") + "]"
}
func jsSource(s IFSource) string {
   return "{\"device_claim\":" + jsStr(s.DeviceClaim) +
   ",\"lens_profile_digest\":" + jsStr(s.LensProfileDigest) +
   ",\"location_claim\":" + jsStr(s.LocationClaim) +
   ",\"sensor_profile_digest\":" + jsStr(s.SensorProfileDigest) +
   ",\"timestamp\":" + jsStr(s.Timestamp) + "}"
}
func preimageJSON(r IFReceipt) string {
   return "{\"byte_digest\":" + jsStr(&r.ByteDigest) +
   ",\"colorspace\":" + jsStr(&r.Colorspace) +
   ",\"format\":" + jsStr(&r.Format) +
   ",\"height\":" + jsInt(r.Height) +
   ",\"icc_digest\":" + jsStr(r.ICCDigest) +
   ",\"if_version\":" + jsStr(&r.IFVersion) +
   ",\"operation\":" + jsStr(&r.Operation) +
   ",\"orientation\":" + jsInt(r.Orientation) +
   ",\"palette\":" + jsPalette(r.Palette) +
   ",\"params\":" + jsStr(r.Params) +
   ",\"parent_if_id\":" + jsStr(r.ParentIFID) +
   ",\"pixel_digest\":" + jsStr(&r.PixelDigest) +
   ",\"pixel_sample_stride\":" + jsInt(r.PixelSampleStride) +
   ",\"source\":" + jsSource(r.Source) +
   ",\"width\":" + jsInt(r.Width) + "}"
}

// MakeReceipt builds a complete IF-Protocol-1.0.0 receipt.
func MakeReceipt(fields IFReceiptFields) IFReceipt {
   pd := PixelDigestCanonical(fields.Pixels)
   palette := fields.Palette
   if palette == nil { palette = []IFPalette{} }
   partial := IFReceipt{
   ByteDigest: fields.ByteDigest, Colorspace: fields.Colorspace,
   Format: fields.Format, Height: fields.Height,
   ICCDigest: fields.ICCDigest, IFID: "", IFVersion: "IF-CAPTURE-1.0.0",
   Operation: fields.Operation, Orientation: fields.Orientation,
   Palette: palette, Params: fields.Params, ParentIFID: fields.ParentIFID,
   PixelDigest: pd, PixelSampleStride: 1, ReceiptDigest: "",
   Source: fields.Source, Width: fields.Width,
   }
   rd := SHA256Text(preimageJSON(partial))
   ifID := "IF-" + Hash12(pd) + "-" + Hash12(rd)
   partial.IFID = ifID
   partial.ReceiptDigest = rd
   return partial
}

// VerifyReceipt verifies an existing receipt by recomputing if_id and receipt_digest.
func VerifyReceipt(receipt IFReceipt) IFVerifyResult {
   expectedRd := SHA256Text(preimageJSON(receipt))
   expectedID := "IF-" + Hash12(receipt.PixelDigest) + "-" + Hash12(expectedRd)
   if receipt.ReceiptDigest != expectedRd {
   return IFVerifyResult{false, "receipt_digest mismatch"}
   }
   if receipt.IFID != expectedID {
   return IFVerifyResult{false, "if_id mismatch"}
   }
   return IFVerifyResult{true, "ok"}
}
