use crate::i256;
use alloy_primitives::{FixedBytes, U256};
use core::cmp::Ordering;
use alloy_primitives::hex::FromHex;

pub fn from_hex(hex: &str) -> U256 {
    let bytes: FixedBytes<32> = FixedBytes::from_hex(hex).unwrap();
    bytes.into()
}

#[derive(Debug)]
pub struct Memory {
    inner: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        let mut memory = Self { inner: Vec::new() };
        // Initialize the free memory pointer
        memory.set_byte(0x40 + 0x20 - 1, 0x80);
        memory
    }

    fn get_byte(&self, index: usize) -> u8 {
        // let index: usize = U256::try_into(index).unwrap();
        self.inner.get(index).cloned().unwrap_or_default()
    }

    fn set_byte(&mut self, index: usize, value: u8) {
        // let index: usize = U256::try_into(index).unwrap();
        if index >= self.inner.len() {
            self.inner.resize(index + 1, 0);
        }
        self.inner[index] = value;
    }

    fn load(&self, address: U256, length: U256) -> Vec<u8> {
        let address: usize = U256::try_into(address).unwrap();
        let length: usize = U256::try_into(length).unwrap();
        let mut result = Vec::new();

        for i in 0..length {
            result.push(self.get_byte(address + i));
        }

        result
    }

    fn store(&mut self, address: U256, value: Vec<u8>) {
        let address: usize = U256::try_into(address).unwrap();
        for (i, byte) in value.iter().enumerate() {
            self.set_byte(address + i, *byte);
        }
    }
}

#[derive(Debug)]
pub enum ReturnOrRevert {
    Return { start: U256, length: U256 },
    Revert { start: U256, length: U256 },
}

pub type YulOutput<A> = Result<A, ReturnOrRevert>;

// Pure opcodes

pub fn add(x: U256, y: U256, _memory: &Memory) -> YulOutput<U256> {
    Ok(x.wrapping_add(y))
}

pub fn sub(x: U256, y: U256, _memory: &Memory) -> YulOutput<U256> {
    Ok(x.wrapping_sub(y))
}

pub fn mul(x: U256, y: U256, _memory: &Memory) -> YulOutput<U256> {
    Ok(x.wrapping_mul(y))
}

pub fn div(x: U256, y: U256, _memory: &Memory) -> YulOutput<U256> {
    Ok(if y == U256::ZERO {
        U256::ZERO
    } else {
        x.wrapping_div(y)
    })
}

pub fn sdiv(x: U256, y: U256, _memory: &Memory) -> YulOutput<U256> {
    Ok(i256::i256_div(x, y))
}

pub fn mod_(x: U256, y: U256, _memory: &Memory) -> YulOutput<U256> {
    Ok(if y == U256::ZERO {
        U256::ZERO
    } else {
        x.wrapping_rem(y)
    })
}

pub fn smod(x: U256, y: U256, _memory: &Memory) -> YulOutput<U256> {
    Ok(i256::i256_mod(x, y))
}

pub fn exp(x: U256, y: U256, _memory: &Memory) -> YulOutput<U256> {
    Ok(x.pow(y))
}

pub fn not(x: U256, _memory: &Memory) -> YulOutput<U256> {
    Ok(!x)
}

pub fn lt(x: U256, y: U256, _memory: &Memory) -> YulOutput<U256> {
    Ok(U256::from(x < y))
}

pub fn gt(x: U256, y: U256, _memory: &Memory) -> YulOutput<U256> {
    Ok(U256::from(x > y))
}

pub fn slt(x: U256, y: U256, _memory: &Memory) -> YulOutput<U256> {
    Ok(U256::from(i256::i256_cmp(&x, &y) == Ordering::Less))
}

pub fn sgt(x: U256, y: U256, _memory: &Memory) -> YulOutput<U256> {
    Ok(U256::from(i256::i256_cmp(&x, &y) == Ordering::Greater))
}

pub fn eq(x: U256, y: U256, _memory: &Memory) -> YulOutput<U256> {
    Ok(U256::from(x == y))
}

pub fn iszero(x: U256, _memory: &Memory) -> YulOutput<U256> {
    Ok(U256::from(x.is_zero()))
}

pub fn and(x: U256, y: U256, _memory: &Memory) -> YulOutput<U256> {
    Ok(x & y)
}

pub fn or(x: U256, y: U256, _memory: &Memory) -> YulOutput<U256> {
    Ok(x | y)
}

pub fn xor(x: U256, y: U256, _memory: &Memory) -> YulOutput<U256> {
    Ok(x ^ y)
}

pub fn byte(op1: U256, op2: U256, _memory: &Memory) -> YulOutput<U256> {
    let o1 = as_usize_saturated!(op1);
    if o1 < 32 {
        // `31 - o1` because `byte` returns LE, while we want BE
        Ok(U256::from(op2.byte(31 - o1)))
    } else {
        Ok(U256::ZERO)
    }
}

pub fn shl(op1: U256, op2: U256, _memory: &Memory) -> YulOutput<U256> {
    let shift = as_usize_saturated!(op1);
    if shift < 256 {
        Ok(op2 << shift)
    } else {
        Ok(U256::ZERO)
    }
}

pub fn shr(op1: U256, op2: U256, _memory: &Memory) -> YulOutput<U256> {
    let shift = as_usize_saturated!(op1);
    if shift < 256 {
        Ok(op2 >> shift)
    } else {
        Ok(U256::ZERO)
    }
}

pub fn sar(op1: U256, op2: U256, _memory: &Memory) -> YulOutput<U256> {
    let shift = as_usize_saturated!(op1);
    if shift < 256 {
        Ok(op2.arithmetic_shr(shift))
    } else if op2.bit(255) {
        Ok(U256::MAX)
    } else {
        Ok(U256::ZERO)
    }
}

pub fn addmod(op1: U256, op2: U256, op3: U256, _memory: &Memory) -> YulOutput<U256> {
    Ok(op1.add_mod(op2, op3))
}

pub fn mulmod(op1: U256, op2: U256, op3: U256, _memory: &Memory) -> YulOutput<U256> {
    Ok(op1.mul_mod(op2, op3))
}

pub fn signextend(ext: U256, x: U256, _memory: &Memory) -> YulOutput<U256> {
    // For 31 we also don't need to do anything.
    if ext < U256::from(31) {
        let ext = ext.as_limbs()[0];
        let bit_index = (8 * ext + 7) as usize;
        let bit = x.bit(bit_index);
        let mask = (U256::from(1) << bit_index) - U256::from(1);
        if bit {
            Ok(x | !mask)
        } else {
            Ok(x & mask)
        }
    } else {
        Ok(x)
    }
}

pub fn keccak256(_p: U256, _n: U256, _memory: &Memory) -> YulOutput<U256> {
    unimplemented!()
}

pub fn pop(_x : U256, _memory: &Memory) -> YulOutput<()> {
    Ok(())
}

// Memory opcodes

pub fn mload(address: U256, memory: &Memory) -> YulOutput<U256> {
    let bytes: Vec<u8> = memory.load(address, U256::from(32));
    let bytes: [u8; 32] = bytes.try_into().unwrap();
    let bytes: FixedBytes<32> = bytes.into();
    Ok(U256::try_from(bytes).unwrap())
}

pub fn mstore(address: U256, value: U256, memory: &mut Memory) -> YulOutput<()> {
    let bytes: [u8; 32] = value.to_be_bytes::<32>();
    memory.store(address, bytes.to_vec());
    Ok(())
}

pub fn mstore8(address: U256, value: U256, memory: &mut Memory) -> YulOutput<()> {
    memory.store(address, vec![value.byte(0)]);
    Ok(())
}

pub fn sload(_p: U256, _memory: &Memory) -> YulOutput<U256> {
    unimplemented!()
}

pub fn sstore(_p: U256, _v: U256, _memory: &mut Memory) -> YulOutput<()> {
    unimplemented!()
}

pub fn tload(_p: U256, _memory: &Memory) -> YulOutput<U256> {
    unimplemented!()
}

pub fn tstore(_p: U256, _v: U256, _memory: &mut Memory) -> YulOutput<()> {
    unimplemented!()
}

pub fn msize(_memory: &Memory) -> YulOutput<U256> {
    unimplemented!()
}

pub fn gas(_memory: &Memory) -> YulOutput<U256> {
    Ok(U256::from(100 * 1000))
}

pub fn address(_memory: &Memory) -> YulOutput<U256> {
    unimplemented!()
}

pub fn balance(_memory: &Memory, _address: U256) -> YulOutput<U256> {
    unimplemented!()
}

pub fn selfbalance(_memory: &Memory) -> YulOutput<U256> {
    unimplemented!()
}

pub fn caller(_memory: &Memory) -> YulOutput<U256> {
    unimplemented!()
}

pub fn callvalue(_memory: &Memory) -> YulOutput<U256> {
    unimplemented!()
}

pub fn calldataload(_p: U256, _memory: &Memory) -> YulOutput<U256> {
    unimplemented!()
}

pub fn calldatasize(_memory: &Memory) -> YulOutput<U256> {
    unimplemented!()
}

pub fn calldatacopy(_t: U256, _f: U256, _s: U256, _memory: &Memory) -> YulOutput<()> {
    unimplemented!()
}

pub fn codesize(_memory: &Memory) -> YulOutput<U256> {
    unimplemented!()
}

pub fn codecopy(_t: U256, _f: U256, _s: U256, _memory: &Memory) -> YulOutput<()> {
    unimplemented!()
}

pub fn extcodesize(_a: U256, _memory: &Memory) -> YulOutput<U256> {
    unimplemented!()
}

pub fn extcodecopy(_a: U256, _t: U256, _f: U256, _s: U256, _memory: &Memory) -> YulOutput<()> {
    unimplemented!()
}

pub fn returndatasize(_memory: &Memory) -> YulOutput<U256> {
    unimplemented!()
}

pub fn returndatacopy(_t: U256, _f: U256, _s: U256, _memory: &Memory) -> YulOutput<()> {
    unimplemented!()
}

pub fn mcopy(_t: U256, _f: U256, _s: U256, _memory: &Memory) -> YulOutput<()> {
    unimplemented!()
}

pub fn extcodehash(_a: U256, _memory: &Memory) -> YulOutput<U256> {
    unimplemented!()
}

pub fn create(_v: U256, _p: U256, _n: U256, _memory: &Memory) -> YulOutput<U256> {
    unimplemented!()
}

pub fn create2(_v: U256, _p: U256, _n: U256, _s: U256, _memory: &Memory) -> YulOutput<U256> {
    unimplemented!()
}

pub fn call(
    _g: U256,
    _a: U256,
    _v: U256,
    _in_: U256,
    _insize: U256,
    _out: U256,
    _outsize: U256,
    _memory: &Memory,
) -> YulOutput<U256> {
    unimplemented!()
}

pub fn callcode(
    _g: U256,
    _a: U256,
    _v: U256,
    _in_: U256,
    _insize: U256,
    _out: U256,
    _outsize: U256,
    _memory: &Memory,
) -> YulOutput<U256> {
    unimplemented!()
}

pub fn delegatecall(
    _g: U256,
    _a: U256,
    _in_: U256,
    _insize: U256,
    _out: U256,
    _outsize: U256,
    _memory: &Memory,
) -> YulOutput<U256> {
    unimplemented!()
}

pub fn staticcall(
    _gas: U256,
    _address: U256,
    _args_offset: U256,
    _args_size: U256,
    _ret_offset: U256,
    _ret_size: U256,
    _memory: &Memory,
) -> YulOutput<U256> {
    // Pre-compiles for the ZK verifier: 2, 5, 6, 7, 8
    Ok(U256::ONE)
}

pub fn return_(offset: U256, size: U256, _memory: &mut Memory) -> YulOutput<()> {
    Err(ReturnOrRevert::Return {
        start: offset,
        length: size,
    })
}

pub fn revert(offset: U256, size: U256, _memory: &mut Memory) -> YulOutput<()> {
    Err(ReturnOrRevert::Revert {
        start: offset,
        length: size,
    })
}

pub fn selfdestruct(_a: U256, _memory: &mut Memory) -> YulOutput<()> {
    unimplemented!()
}

pub fn invalid() -> YulOutput<()> {
    unimplemented!()
}

pub fn log0(_p: U256, _s: U256, _memory: &mut Memory) -> YulOutput<()> {
    unimplemented!()
}

pub fn log1(_p: U256, _s: U256, _t1: U256, _memory: &mut Memory) -> YulOutput<()> {
    unimplemented!()
}

pub fn log2(_p: U256, _s: U256, _t1: U256, _t2: U256, _memory: &mut Memory) -> YulOutput<()> {
    unimplemented!()
}

pub fn log3(_p: U256, _s: U256, _t1: U256, _t2: U256, _t3: U256, _memory: &mut Memory) -> YulOutput<()> {
    unimplemented!()
}

pub fn log4(_p: U256, _s: U256, _t1: U256, _t2: U256, _t3: U256, _t4: U256, _memory: &mut Memory) -> YulOutput<()> {
    unimplemented!()
}

pub fn chainid() -> YulOutput<U256> {
    unimplemented!()
}

pub fn basefee() -> YulOutput<U256> {
    unimplemented!()
}

pub fn blobbasefee() -> YulOutput<U256> {
    unimplemented!()
}

pub fn origin() -> YulOutput<U256> {
    unimplemented!()
}

pub fn gasprice() -> YulOutput<U256> {
    unimplemented!()
}

pub fn blockhash(_memory: &Memory, _b: U256) -> YulOutput<U256> {
    unimplemented!()
}

pub fn blobhash(_memory: &Memory, _i: U256) -> YulOutput<U256> {
    unimplemented!()
}

pub fn coinbase() -> YulOutput<U256> {
    unimplemented!()
}

pub fn timestamp() -> YulOutput<U256> {
    unimplemented!()
}

pub fn number() -> YulOutput<U256> {
    unimplemented!()
}

pub fn difficulty() -> YulOutput<U256> {
    unimplemented!()
}

pub fn prevrandao() -> YulOutput<U256> {
    unimplemented!()
}

pub fn gaslimit() -> YulOutput<U256> {
    unimplemented!()
}

// Special opcodes

pub fn memoryguard(size: U256, _memory: &mut Memory) -> YulOutput<U256> {
    Ok(size)
}

pub fn datasize(_x: U256, _memory: &Memory) -> YulOutput<U256> {
    unimplemented!()
}

pub fn dataoffset(_x: U256, _memory: &Memory) -> YulOutput<U256> {
    unimplemented!()
}

pub fn datacopy(t: U256, f: U256, s: U256, _memory: &Memory) -> YulOutput<()> {
    codecopy(t, f, s, _memory)
}
