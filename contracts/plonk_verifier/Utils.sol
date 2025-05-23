// SPDX-License-Identifier: Apache-2.0

// Copyright 2023 Consensys Software Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// Code generated by gnark DO NOT EDIT

pragma solidity ^0.8.0;

library Utils {
  uint256 private constant r_mod = 21888242871839275222246405745257275088548364400416034343698204186575808495617;
  uint256 private constant bb = 340282366920938463463374607431768211456; // 2**128
  uint256 private constant error_string_id = 0x08c379a000000000000000000000000000000000000000000000000000000000; // selector for function Error(string)
  uint256 private constant zero_uint256 = 0;

  uint8 private constant lenInBytes = 48;
  uint8 private constant sizeDomain = 11;
  uint8 private constant one = 1;
  uint8 private constant two = 2;

  /**
   * @dev xmsg expands msg to a slice of lenInBytes bytes.
   *      https://tools.ietf.org/html/draft-irtf-cfrg-hash-to-curve-06#section-5
   *      https://tools.ietf.org/html/rfc8017#section-4.1 (I2OSP/O2ISP)
   * @dev cf https://tools.ietf.org/html/draft-irtf-cfrg-hash-to-curve-06#section-5.2
   * corresponds to https://github.com/ConsenSys/gnark-crypto/blob/develop/ecc/bn254/fr/element.go
   */
  function hash_fr(uint256 x, uint256 y) internal view returns (uint256 res) {
    assembly {
      function error_sha2_256() {
        let ptError := mload(0x40)
        mstore(ptError, error_string_id) // selector for function Error(string)
        mstore(add(ptError, 0x4), 0x20)
        mstore(add(ptError, 0x24), 0x19)
        mstore(add(ptError, 0x44), "error staticcall sha2-256")
        revert(ptError, 0x64)
      }

      // [0x00, .. , 0x00 || x, y, || 0, 48, 0, dst, sizeDomain]
      // <-  64 bytes  ->  <-64b -> <-       1 bytes each     ->
      let mPtr := mload(0x40)

      // [0x00, .., 0x00] 64 bytes of zero
      mstore(mPtr, zero_uint256)
      mstore(add(mPtr, 0x20), zero_uint256)

      // msg =  x || y , both on 32 bytes
      mstore(add(mPtr, 0x40), x)
      mstore(add(mPtr, 0x60), y)

      // 0 || 48 || 0 all on 1 byte
      mstore8(add(mPtr, 0x80), 0)
      mstore8(add(mPtr, 0x81), lenInBytes)
      mstore8(add(mPtr, 0x82), 0)

      // "BSB22-Plonk" = [42, 53, 42, 32, 32, 2d, 50, 6c, 6f, 6e, 6b,]
      mstore8(add(mPtr, 0x83), 0x42)
      mstore8(add(mPtr, 0x84), 0x53)
      mstore8(add(mPtr, 0x85), 0x42)
      mstore8(add(mPtr, 0x86), 0x32)
      mstore8(add(mPtr, 0x87), 0x32)
      mstore8(add(mPtr, 0x88), 0x2d)
      mstore8(add(mPtr, 0x89), 0x50)
      mstore8(add(mPtr, 0x8a), 0x6c)
      mstore8(add(mPtr, 0x8b), 0x6f)
      mstore8(add(mPtr, 0x8c), 0x6e)
      mstore8(add(mPtr, 0x8d), 0x6b)

      // size domain
      mstore8(add(mPtr, 0x8e), sizeDomain)

      let success := staticcall(gas(), 0x2, mPtr, 0x8f, mPtr, 0x20)
      if iszero(success) {
        error_sha2_256()
      }

      let b0 := mload(mPtr)

      // [b0         || one || dst || sizeDomain]
      // <-64bytes ->  <-    1 byte each      ->
      mstore8(add(mPtr, 0x20), one) // 1

      mstore8(add(mPtr, 0x21), 0x42) // dst
      mstore8(add(mPtr, 0x22), 0x53)
      mstore8(add(mPtr, 0x23), 0x42)
      mstore8(add(mPtr, 0x24), 0x32)
      mstore8(add(mPtr, 0x25), 0x32)
      mstore8(add(mPtr, 0x26), 0x2d)
      mstore8(add(mPtr, 0x27), 0x50)
      mstore8(add(mPtr, 0x28), 0x6c)
      mstore8(add(mPtr, 0x29), 0x6f)
      mstore8(add(mPtr, 0x2a), 0x6e)
      mstore8(add(mPtr, 0x2b), 0x6b)

      mstore8(add(mPtr, 0x2c), sizeDomain) // size domain
      success := staticcall(gas(), 0x2, mPtr, 0x2d, mPtr, 0x20)
      if iszero(success) {
        error_sha2_256()
      }

      // b1 is located at mPtr. We store b2 at add(mPtr, 0x20)

      // [b0^b1      || two || dst || sizeDomain]
      // <-64bytes ->  <-    1 byte each      ->
      mstore(add(mPtr, 0x20), xor(mload(mPtr), b0))
      mstore8(add(mPtr, 0x40), two)

      mstore8(add(mPtr, 0x41), 0x42) // dst
      mstore8(add(mPtr, 0x42), 0x53)
      mstore8(add(mPtr, 0x43), 0x42)
      mstore8(add(mPtr, 0x44), 0x32)
      mstore8(add(mPtr, 0x45), 0x32)
      mstore8(add(mPtr, 0x46), 0x2d)
      mstore8(add(mPtr, 0x47), 0x50)
      mstore8(add(mPtr, 0x48), 0x6c)
      mstore8(add(mPtr, 0x49), 0x6f)
      mstore8(add(mPtr, 0x4a), 0x6e)
      mstore8(add(mPtr, 0x4b), 0x6b)

      mstore8(add(mPtr, 0x4c), sizeDomain) // size domain

      let offset := add(mPtr, 0x20)
      success := staticcall(gas(), 0x2, offset, 0x2d, offset, 0x20)
      if iszero(success) {
        error_sha2_256()
      }

      // at this point we have mPtr = [ b1 || b2] where b1 is on 32byes and b2 in 16bytes.
      // we interpret it as a big integer mod r in big endian (similar to regular decimal notation)
      // the result is then 2**(8*16)*mPtr[:32] + mPtr[32:48]
      res := mulmod(mload(mPtr), bb, r_mod) // <- res = 2**128 * mPtr[:32]
      offset := add(mPtr, 0x10)
      for {
        let i := 0
      } lt(i, 0x10) {
        i := add(i, 1)
      } {
        // mPtr <- [xx, xx, ..,  | 0, 0, .. 0  ||    b2   ]
        mstore8(offset, 0x00)
        offset := add(offset, 0x1)
      }
      let b1 := mload(add(mPtr, 0x10)) // b1 <- [0, 0, .., 0 ||  b2[:16] ]
      res := addmod(res, b1, r_mod)
    }
  }
}
