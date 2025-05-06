// SPDX-License-Identifier: GPL-3.0
pragma solidity <0.9.0;

contract TestOpcodes {
    function testAdd(uint x, uint y) public pure returns (uint result) {
        assembly {
            result := add(x, y)
        }
    }

    function testSub(uint x, uint y) public pure returns (uint result) {
        assembly {
            result := sub(x, y)
        }
    }

    function testMul(uint x, uint y) public pure returns (uint result) {
        assembly {
            result := mul(x, y)
        }
    }

    function testDiv(uint x, uint y) public pure returns (uint result) {
        assembly {
            result := div(x, y)
        }
    }

    function testSdiv(int x, int y) public pure returns (int result) {
        assembly {
            result := sdiv(x, y)
        }
    }

    function testMod(uint x, uint y) public pure returns (uint result) {
        assembly {
            result := mod(x, y)
        }
    }

    function testSmod(int x, int y) public pure returns (int result) {
        assembly {
            result := smod(x, y)
        }
    }

    function testExp(uint x, uint y) public pure returns (uint result) {
        assembly {
            result := exp(x, y)
        }
    }

    function testNot(uint x) public pure returns (uint result) {
        assembly {
            result := not(x)
        }
    }

    function testLt(uint x, uint y) public pure returns (uint result) {
        assembly {
            result := lt(x, y)
        }
    }

    function testGt(uint x, uint y) public pure returns (uint result) {
        assembly {
            result := gt(x, y)
        }
    }

    function testSlt(int x, int y) public pure returns (uint result) {
        assembly {
            result := slt(x, y)
        }
    }

    function testSgt(int x, int y) public pure returns (uint result) {
        assembly {
            result := sgt(x, y)
        }
    }

    function testEq(uint x, uint y) public pure returns (uint result) {
        assembly {
            result := eq(x, y)
        }
    }

    function testIszero(uint x) public pure returns (uint result) {
        assembly {
            result := iszero(x)
        }
    }

    function testAnd(uint x, uint y) public pure returns (uint result) {
        assembly {
            result := and(x, y)
        }
    }

    function testOr(uint x, uint y) public pure returns (uint result) {
        assembly {
            result := or(x, y)
        }
    }

    function testXor(uint x, uint y) public pure returns (uint result) {
        assembly {
            result := xor(x, y)
        }
    }

    function testByte(uint x, uint y) public pure returns (uint result) {
        assembly {
            result := byte(x, y)
        }
    }

    function testShl(uint x, uint y) public pure returns (uint result) {
        assembly {
            result := shl(x, y)
        }
    }

    function testShr(uint x, uint y) public pure returns (uint result) {
        assembly {
            result := shr(x, y)
        }
    }

    function testSar(uint x, int y) public pure returns (int result) {
        assembly {
            result := sar(x, y)
        }
    }

    function testAddmod(
        uint x,
        uint y,
        uint z
    ) public pure returns (uint result) {
        assembly {
            result := addmod(x, y, z)
        }
    }

    function testMulmod(
        uint x,
        uint y,
        uint z
    ) public pure returns (uint result) {
        assembly {
            result := mulmod(x, y, z)
        }
    }

    function testSignextend(uint x, uint y) public pure returns (uint result) {
        assembly {
            result := signextend(x, y)
        }
    }

    function testKeccak256(
        bytes memory data
    ) public pure returns (bytes32 result) {
        assembly {
            result := keccak256(add(data, 32), mload(data))
        }
    }

    function testPop(uint x) public pure {
        assembly {
            pop(x)
        }
    }

    function testMload(uint x) public pure returns (uint result) {
        assembly {
            result := mload(x)
        }
    }

    function testMstore(uint x, uint y) public pure {
        assembly {
            mstore(x, y)
        }
    }

    function testMstore8(uint x, uint y) public pure {
        assembly {
            mstore8(x, y)
        }
    }

    function testGas() public view returns (uint result) {
        assembly {
            result := gas()
        }
    }

    function testCalldataload(uint x) public pure returns (uint result) {
        assembly {
            result := calldataload(x)
        }
    }

    function testCalldatasize() public pure returns (uint result) {
        assembly {
            result := calldatasize()
        }
    }

    function testCalldatacopy(uint t, uint f, uint s) public pure {
        assembly {
            calldatacopy(t, f, s)
        }
    }

    function runTests() public view {
        // Arithmetic tests
        assert(testAdd(0, 0) == 0);
        assert(testAdd(1, 2) == 3);
        assert(testSub(0, 0) == 0);
        assert(testSub(2, 1) == 1);
        assert(testMul(2, 3) == 6);
        assert(testDiv(6, 2) == 3);
        assert(testSdiv(-6, 2) == -3);
        assert(testMod(7, 3) == 1);
        assert(testSmod(-7, 3) == -1);
        assert(testExp(2, 3) == 8);

        // Bitwise tests
        assert(testNot(0) == type(uint).max);
        assert(testLt(1, 2) == 1);
        assert(testGt(2, 1) == 1);
        assert(testSlt(-1, 0) == 1);
        assert(testSgt(1, -1) == 1);
        assert(testEq(1, 1) == 1);
        assert(testIszero(0) == 1);
        assert(testAnd(3, 2) == 2);
        assert(testOr(1, 2) == 3);
        assert(testXor(3, 1) == 2);

        // Bit manipulation tests
        assert(testByte(0, 0x1234) == 0);
        assert(testShl(2, 1) == 4);
        assert(testShr(2, 4) == 1);
        assert(testSar(2, -4) == -1);

        // Modular arithmetic tests
        assert(testAddmod(2, 3, 4) == 1);
        assert(testMulmod(2, 3, 4) == 2);

        // Sign extension test
        assert(testSignextend(0, 0x7f) == 0x7f);
        assert(
            testSignextend(0, 0x80) ==
                0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff80
        );

        // Keccak256 tests
        assert(testKeccak256(abi.encode(0x1234)) == keccak256(abi.encode(0x1234)));

        // Stack tests
        testPop(1);

        // Memory tests
        testMstore(0, 0x1234);
        assert(testMload(0) == 0x1234);
        testMstore8(32 + 31, 0x12);
        assert(testMload(32) == 0x12);

        // Gas test
        assert(testGas() > 0);

        // Calldata tests
        assert(testCalldatasize() == 0);
    }
}
