// SPDX-License-Identifier: GPL-3.0
pragma solidity >=0.4.16 <0.9.0;

contract Minimal {
	function add_one(uint x) pure public returns (uint) {
		return x + 1;
	}
}
