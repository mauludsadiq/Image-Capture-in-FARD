#!/bin/bash
# tools/capture_macbook.sh -- MacBook camera capture bridge for IF-Protocol.
# Captures a single JPEG frame from the default camera using imagesnap,
# then runs image_explain and creates an IF-Claim.
#
# Usage: bash tools/capture_macbook.sh [label]
# Output: out/captures/capture_YYYYMMDD_HHMMSS.jpg + claim JSON

set -e

LABEL=${1:-"macbook_camera_capture"}
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
OUTDIR="out/captures"
IMGFILE="$OUTDIR/capture_${TIMESTAMP}.jpg"
CLAIMFILE="$OUTDIR/capture_${TIMESTAMP}_claim.json"

mkdir -p "$OUTDIR"

echo "==> Capturing from MacBook camera..."
imagesnap -w 1.5 "$IMGFILE"
echo "    saved: $IMGFILE"

echo ""
echo "==> Image identity:"
fardrun run --program apps/image_explain.fard --out out/capture_explain -- "$IMGFILE" 6

echo ""
echo "==> Creating IF-Claim..."
fardrun run --program apps/image_claim.fard --out out/capture_claim -- create "$IMGFILE"

echo ""
echo "==> Done. Files:"
echo "    image:  $IMGFILE"
ls "$OUTDIR"/capture_${TIMESTAMP}* 2>/dev/null || true
