package ifreceipt

import "testing"

// colorfulPixels returns the 4x4 canonical test image from SPEC-IF-1.0.md A.1.
func colorfulPixels() [][3]int {
   colors := [][3]int{{220,50,50},{50,180,80},{50,90,220},{240,200,40}}
   pixels := make([][3]int, 16)
   for i := range pixels {
   pixels[i] = colors[(i%4+i/4)%4]
   }
   return pixels
}

func TestCanonicalPixelDigestVector(t *testing.T) {
   got := PixelDigestCanonical(colorfulPixels())
   want := "sha256:2a40854db5a950bbdc8d921620a2ee8074fb8b950102150b6be7b10990e1ddb6"
   if got != want {
   t.Errorf("pixel_digest = %s, want %s", got, want)
   }
}

func TestHash12(t *testing.T) {
   got := Hash12("sha256:2a40854db5a950bbdc8d921620a2ee8074fb8b950102150b6be7b10990e1ddb6")
   if got != "2A40854DB5A9" {
   t.Errorf("Hash12 = %s, want 2A40854DB5A9", got)
   }
}

func TestIFIDFormat(t *testing.T) {
   pd := "sha256:2a40854db5a950bbdc8d921620a2ee8074fb8b950102150b6be7b10990e1ddb6"
   rd := "sha256:" + "0000000000000000000000000000000000000000000000000000000000000000"
   id := "IF-" + Hash12(pd) + "-" + Hash12(rd)
   if len(id) != 28 {
   t.Errorf("IF-ID length = %d, want 28", len(id))
   }
   if id[:3] != "IF-" {
   t.Errorf("IF-ID does not start with IF-")
   }
}

func TestMakeReceiptIsValid(t *testing.T) {
   fields := DefaultFields(colorfulPixels(),
   "sha256:"+"0000000000000000000000000000000000000000000000000000000000000000",
   "png", 4, 4)
   r := MakeReceipt(fields)
   result := VerifyReceipt(r)
   if !result.Valid {
   t.Errorf("receipt invalid: %s", result.Reason)
   }
}

func TestSamePixelsSameDigest(t *testing.T) {
   if PixelDigestCanonical(colorfulPixels()) != PixelDigestCanonical(colorfulPixels()) {
   t.Error("same pixels produced different digest")
   }
}

func TestDifferentPixelsDifferentDigest(t *testing.T) {
   red := [][3]int{{255,0,0},{255,0,0},{255,0,0},{255,0,0}}
   blue := [][3]int{{0,0,255},{0,0,255},{0,0,255},{0,0,255}}
   if PixelDigestCanonical(red) == PixelDigestCanonical(blue) {
   t.Error("different pixels produced same digest")
   }
}

func TestTamperedReceiptDetected(t *testing.T) {
   fields := DefaultFields(colorfulPixels(),
   "sha256:"+"0000000000000000000000000000000000000000000000000000000000000000",
   "png", 4, 4)
   r := MakeReceipt(fields)
   r.IFID = "IF-000000000000-000000000000"
   result := VerifyReceipt(r)
   if result.Valid {
   t.Error("tampered receipt should be invalid")
   }
}

func TestParentIFIDNilForOriginal(t *testing.T) {
   fields := DefaultFields([][3]int{{255,0,0}},
   "sha256:"+"0000000000000000000000000000000000000000000000000000000000000000",
   "png", 1, 1)
   r := MakeReceipt(fields)
   if r.ParentIFID != nil {
   t.Error("ParentIFID should be nil for original")
   }
}

func TestIFVersion(t *testing.T) {
   fields := DefaultFields([][3]int{{255,0,0}},
   "sha256:"+"0000000000000000000000000000000000000000000000000000000000000000",
   "png", 1, 1)
   r := MakeReceipt(fields)
   if r.IFVersion != "IF-CAPTURE-1.0.0" {
   t.Errorf("IFVersion = %s, want IF-CAPTURE-1.0.0", r.IFVersion)
   }
}
