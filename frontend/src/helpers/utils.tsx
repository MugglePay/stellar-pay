import BigNumber from "bignumber.js";
import * as StellarSdk from "@stellar/stellar-sdk";
import { I128 } from "./xdr";
import { Address, xdr } from "@stellar/stellar-sdk";

function bigNumberFromBytes(
  signed: boolean,
  ...bytes: (string | number | bigint)[]
): BigNumber {
  let sign = 1;
  if (signed && bytes[0] === 0x80) {
    // top bit is set, negative number.
    sign = -1;
    bytes[0] &= 0x7f;
  }
  let b = BigInt(0);
  for (let byte of bytes) {
    b <<= BigInt(8);
    b |= BigInt(byte);
  }
  return BigNumber(b.toString()).multipliedBy(sign);
}

export function bigNumberToI128(value: BigNumber): StellarSdk.xdr.ScVal {
  const b: bigint = BigInt(value.toFixed(0));
  const buf = bigintToBuf(b);
  if (buf.length > 16) {
    throw new Error("BigNumber overflows i128");
  }

  if (value.isNegative()) {
    // Clear the top bit
    buf[0] &= 0x7f;
  }

  // left-pad with zeros up to 16 bytes
  let padded = Buffer.alloc(16);
  buf.copy(padded, padded.length - buf.length);

  if (value.isNegative()) {
    // Set the top bit
    padded[0] |= 0x80;
  }

  const hi = new xdr.Int64([
    bigNumberFromBytes(false, ...padded.slice(4, 8)).toNumber(),
    bigNumberFromBytes(false, ...padded.slice(0, 4)).toNumber(),
  ]);
  const lo = new xdr.Uint64([
    bigNumberFromBytes(false, ...padded.slice(12, 16)).toNumber(),
    bigNumberFromBytes(false, ...padded.slice(8, 12)).toNumber(),
  ]);

  return xdr.ScVal.scvI128(new xdr.Int128Parts({ lo, hi }));
}

function bigintToBuf(bn: bigint): Buffer {
  var hex = BigInt(bn).toString(16).replace(/^-/, "");
  if (hex.length % 2) {
    hex = "0" + hex;
  }

  var len = hex.length / 2;
  var u8 = new Uint8Array(len);

  var i = 0;
  var j = 0;
  while (i < len) {
    u8[i] = parseInt(hex.slice(j, j + 2), 16);
    i += 1;
    j += 2;
  }

  if (bn < BigInt(0)) {
    // Set the top bit
    u8[0] |= 0x80;
  }

  return Buffer.from(u8);
}

export const addressToScVal = (addr: string): xdr.ScVal => {
  let addrObj = Address.fromString(addr);
  return addrObj.toScVal();
};

export const i128ToScVal = (i: bigint): xdr.ScVal => {
  return xdr.ScVal.scvI128(
    new xdr.Int128Parts({
      lo: xdr.Uint64.fromString((i & BigInt(0xffffffffffffffffn)).toString()),
      hi: xdr.Int64.fromString(
        ((i >> BigInt(64)) & BigInt(0xffffffffffffffffn)).toString()
      ),
    })
  );
};
