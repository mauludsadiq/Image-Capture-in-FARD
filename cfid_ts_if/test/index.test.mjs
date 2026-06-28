// Tests for IFReceipt TypeScript port
// Run with: node --test test/index.test.mjs

import { strictEqual, notStrictEqual, ok } from "node:assert";
import { test } from "node:test";
import { pixelDigestCanonical, hash12, makeReceipt, verifyReceipt, emptySource } from "../dist/index.js";

function colorfulPixels() {
  const colors = [[220,50,50],[50,180,80],[50,90,220],[240,200,40]];
  return Array.from({length:16}, (_,i) => colors[(i%4+Math.floor(i/4))%4]);
}

test("canonical pixel_digest vector: colorful 4x4 PNG", () => {
  strictEqual(
    pixelDigestCanonical(colorfulPixels()),
    "sha256:2a40854db5a950bbdc8d921620a2ee8074fb8b950102150b6be7b10990e1ddb6"
  );
});

test("hash12 extracts 12 uppercase hex chars", () => {
  strictEqual(
    hash12("sha256:2a40854db5a950bbdc8d921620a2ee8074fb8b950102150b6be7b10990e1ddb6"),
    "2A40854DB5A9"
  );
});

test("IF-ID format: 28 chars, starts with IF-", () => {
  const pd = "sha256:2a40854db5a950bbdc8d921620a2ee8074fb8b950102150b6be7b10990e1ddb6";
  const rd = "sha256:" + "0".repeat(64);
  const id = `IF-${hash12(pd)}-${hash12(rd)}`;
  strictEqual(id.length, 28);
  ok(id.startsWith("IF-"));
});

test("makeReceipt produces a valid receipt", () => {
  const r = makeReceipt({ pixels: colorfulPixels(), byteDigest: "sha256:" + "0".repeat(64), format: "png", width: 4, height: 4 });
  ok(verifyReceipt(r).valid);
});

test("same pixels -> same pixel_digest", () => {
  strictEqual(pixelDigestCanonical(colorfulPixels()), pixelDigestCanonical(colorfulPixels()));
});

test("different pixels -> different pixel_digest", () => {
  const red = Array(4).fill([255,0,0]);
  const blue = Array(4).fill([0,0,255]);
  notStrictEqual(pixelDigestCanonical(red), pixelDigestCanonical(blue));
});

test("tampered if_id is detected", () => {
  const r = makeReceipt({ pixels: colorfulPixels(), byteDigest: "sha256:" + "0".repeat(64), format: "png", width: 4, height: 4 });
  const tampered = { ...r, if_id: "IF-000000000000-000000000000" };
  ok(!verifyReceipt(tampered).valid);
});

test("parent_if_id is null for original", () => {
  const r = makeReceipt({ pixels: [[255,0,0]], byteDigest: "sha256:" + "0".repeat(64), format: "png", width: 1, height: 1 });
  strictEqual(r.parent_if_id, null);
});

test("if_version is IF-CAPTURE-1.0.0", () => {
  const r = makeReceipt({ pixels: [[255,0,0]], byteDigest: "sha256:" + "0".repeat(64), format: "png", width: 1, height: 1 });
  strictEqual(r.if_version, "IF-CAPTURE-1.0.0");
});
