// IF-Protocol-1.0.0 image identity in TypeScript.
// Pure TypeScript, zero runtime dependencies (node:crypto for SHA-256).
// Conformant with SPEC-IF-1.0.md.

import { createHash } from "node:crypto";

export interface IFPalette { hex: string; cf_id: string; }

export interface IFSource {
  device_claim: string | null;
  sensor_profile_digest: string | null;
  lens_profile_digest: string | null;
  timestamp: string | null;
  location_claim: string | null;
}

export const emptySource = (): IFSource => ({
  device_claim: null, sensor_profile_digest: null,
  lens_profile_digest: null, timestamp: null, location_claim: null
});

export interface IFReceipt {
  byte_digest: string; colorspace: string; format: string;
  height: number; icc_digest: string | null; if_id: string;
  if_version: string; operation: string; orientation: number;
  palette: IFPalette[]; params: string | null; parent_if_id: string | null;
  pixel_digest: string; pixel_sample_stride: number;
  receipt_digest: string; source: IFSource; width: number;
}

export interface IFReceiptFields {
  pixels: [number, number, number][];
  byteDigest: string; format: string; width: number; height: number;
  colorspace?: string; iccDigest?: string | null; orientation?: number;
  palette?: IFPalette[]; source?: IFSource;
  parentIfId?: string | null; operation?: string; params?: string | null;
}

export interface IFVerifyResult { valid: boolean; reason: string; }

export function sha256Text(input: string): string {
  return "sha256:" + createHash("sha256").update(input, "utf8").digest("hex");
}

export function sha256Bytes(data: Buffer | Uint8Array): string {
  return "sha256:" + createHash("sha256").update(data).digest("hex");
}

export function hash12(digest: string): string {
  return digest.slice(7, 19).toUpperCase();
}

/** Canonical pixel digest per SPEC-IF-1.0.md section 3.
 *  Preimage: "r0,g0,b0,r1,g1,b1,...,rN,gN,bN," (trailing comma). */
export function pixelDigestCanonical(pixels: [number, number, number][]): string {
  let s = "";
  for (const [r, g, b] of pixels) s += `${r},${g},${b},`;
  return sha256Text(s);
}

export function pixelDigestFromFloats(pixels: [number, number, number][]): string {
  const clamp = (v: number) => Math.min(255, Math.max(0, Math.trunc(v * 255.0 + 0.5)));
  return pixelDigestCanonical(pixels.map(([r,g,b]) => [clamp(r), clamp(g), clamp(b)]));
}

// Canonical JSON helpers
function js(s: string | null | undefined): string {
  if (s == null) return "null";
  const e = s.replace(/\\/g, "\\\\").replace(/"/g, '\\"');
  return `"${e}"`;
}
function ji(n: number): string { return String(n); }
function jp(p: IFPalette[]): string {
  return "[" + p.map(e => `{"cf_id":${js(e.cf_id)},"hex":${js(e.hex)}}`).join(",") + "]";
}
function jsrc(s: IFSource): string {
  return `{"device_claim":${js(s.device_claim)},"lens_profile_digest":${js(s.lens_profile_digest)},"location_claim":${js(s.location_claim)},"sensor_profile_digest":${js(s.sensor_profile_digest)},"timestamp":${js(s.timestamp)}}`;
}
function preimageJSON(r: IFReceipt): string {
  return `{"byte_digest":${js(r.byte_digest)},"colorspace":${js(r.colorspace)},"format":${js(r.format)},"height":${ji(r.height)},"icc_digest":${js(r.icc_digest)},"if_version":${js(r.if_version)},"operation":${js(r.operation)},"orientation":${ji(r.orientation)},"palette":${jp(r.palette)},"params":${js(r.params)},"parent_if_id":${js(r.parent_if_id)},"pixel_digest":${js(r.pixel_digest)},"pixel_sample_stride":${ji(r.pixel_sample_stride)},"source":${jsrc(r.source)},"width":${ji(r.width)}}`;
}

export function makeReceipt(fields: IFReceiptFields): IFReceipt {
  const pd = pixelDigestCanonical(fields.pixels);
  const partial: IFReceipt = {
    byte_digest: fields.byteDigest, colorspace: fields.colorspace ?? "sRGB",
    format: fields.format, height: fields.height,
    icc_digest: fields.iccDigest ?? null, if_id: "", if_version: "IF-CAPTURE-1.0.0",
    operation: fields.operation ?? "original", orientation: fields.orientation ?? 1,
    palette: fields.palette ?? [], params: fields.params ?? null,
    parent_if_id: fields.parentIfId ?? null, pixel_digest: pd,
    pixel_sample_stride: 1, receipt_digest: "",
    source: fields.source ?? emptySource(), width: fields.width,
  };
  const rd = sha256Text(preimageJSON(partial));
  const ifId = `IF-${hash12(pd)}-${hash12(rd)}`;
  return { ...partial, if_id: ifId, receipt_digest: rd };
}

export function verifyReceipt(receipt: IFReceipt): IFVerifyResult {
  const expectedRd = sha256Text(preimageJSON(receipt));
  const expectedId = `IF-${hash12(receipt.pixel_digest)}-${hash12(expectedRd)}`;
  if (receipt.receipt_digest !== expectedRd) return { valid: false, reason: "receipt_digest mismatch" };
  if (receipt.if_id !== expectedId) return { valid: false, reason: "if_id mismatch" };
  return { valid: true, reason: "ok" };
}
