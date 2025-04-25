use alloy_primitives::{FixedBytes, U256};
use core::cmp::Ordering;

mod i256;
mod macros;

#[derive(Debug)]
pub struct Memory {
    inner: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn get_byte(&self, index: usize) -> u8 {
        // let index: usize = U256::try_into(index).unwrap();
        self.inner.get(index).cloned().unwrap_or_default()
    }

    pub fn set_byte(&mut self, index: usize, value: u8) {
        // let index: usize = U256::try_into(index).unwrap();
        if index >= self.inner.len() {
            self.inner.resize(index + 1, 0);
        }
        self.inner[index] = value;
    }

    pub fn load(&self, address: U256, length: U256) -> Vec<u8> {
        let address: usize = U256::try_into(address).unwrap();
        let length: usize = U256::try_into(length).unwrap();
        let mut result = Vec::new();

        for i in 0..length {
            result.push(self.get_byte(address + i));
        }

        result
    }

    pub fn store(&mut self, address: U256, value: Vec<u8>) {
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

pub fn add(_memory: &Memory, x: U256, y: U256) -> YulOutput<U256> {
    Ok(x.wrapping_add(y))
}

pub fn sub(_memory: &Memory, x: U256, y: U256) -> YulOutput<U256> {
    Ok(x.wrapping_sub(y))
}

pub fn mul(_memory: &Memory, x: U256, y: U256) -> YulOutput<U256> {
    Ok(x.wrapping_mul(y))
}

pub fn div(_memory: &Memory, x: U256, y: U256) -> YulOutput<U256> {
    Ok(if y == U256::ZERO {
        U256::ZERO
    } else {
        x.wrapping_div(y)
    })
}

pub fn sdiv(_memory: &Memory, x: U256, y: U256) -> YulOutput<U256> {
    Ok(i256::i256_div(x, y))
}

pub fn mod_(_memory: &Memory, x: U256, y: U256) -> YulOutput<U256> {
    Ok(if y == U256::ZERO {
        U256::ZERO
    } else {
        x.wrapping_rem(y)
    })
}

pub fn smod(_memory: &Memory, x: U256, y: U256) -> YulOutput<U256> {
    Ok(i256::i256_mod(x, y))
}

pub fn exp(_memory: &Memory, x: U256, y: U256) -> YulOutput<U256> {
    Ok(x.pow(y))
}

pub fn not(_memory: &Memory, x: U256) -> YulOutput<U256> {
    Ok(!x)
}

pub fn lt(_memory: &Memory, x: U256, y: U256) -> YulOutput<U256> {
    Ok(U256::from(x < y))
}

pub fn gt(_memory: &Memory, x: U256, y: U256) -> YulOutput<U256> {
    Ok(U256::from(x > y))
}

pub fn slt(_memory: &Memory, x: U256, y: U256) -> YulOutput<U256> {
    Ok(U256::from(i256::i256_cmp(&x, &y) == Ordering::Less))
}

pub fn sgt(_memory: &Memory, x: U256, y: U256) -> YulOutput<U256> {
    Ok(U256::from(i256::i256_cmp(&x, &y) == Ordering::Greater))
}

pub fn eq(_memory: &Memory, x: U256, y: U256) -> YulOutput<U256> {
    Ok(U256::from(x == y))
}

pub fn iszero(_memory: &Memory, x: U256) -> YulOutput<U256> {
    Ok(U256::from(x.is_zero()))
}

pub fn and(_memory: &Memory, x: U256, y: U256) -> YulOutput<U256> {
    Ok(x & y)
}

pub fn or(_memory: &Memory, x: U256, y: U256) -> YulOutput<U256> {
    Ok(x | y)
}

pub fn xor(_memory: &Memory, x: U256, y: U256) -> YulOutput<U256> {
    Ok(x ^ y)
}

pub fn byte(_memory: &Memory, op1: U256, op2: U256) -> YulOutput<U256> {
    let o1 = as_usize_saturated!(op1);
    if o1 < 32 {
        // `31 - o1` because `byte` returns LE, while we want BE
        Ok(U256::from(op2.byte(31 - o1)))
    } else {
        Ok(U256::ZERO)
    }
}

pub fn shl(_memory: &Memory, op1: U256, op2: U256) -> YulOutput<U256> {
    let shift = as_usize_saturated!(op1);
    if shift < 256 {
        Ok(op2 << shift)
    } else {
        Ok(U256::ZERO)
    }
}

pub fn shr(_memory: &Memory, op1: U256, op2: U256) -> YulOutput<U256> {
    let shift = as_usize_saturated!(op1);
    if shift < 256 {
        Ok(op2 >> shift)
    } else {
        Ok(U256::ZERO)
    }
}

pub fn sar(_memory: &Memory, op1: U256, op2: U256) -> YulOutput<U256> {
    let shift = as_usize_saturated!(op1);
    if shift < 256 {
        Ok(op2.arithmetic_shr(shift))
    } else if op2.bit(255) {
        Ok(U256::MAX)
    } else {
        Ok(U256::ZERO)
    }
}

pub fn addmod(_memory: &Memory, op1: U256, op2: U256, op3: U256) -> YulOutput<U256> {
    Ok(op1.add_mod(op2, op3))
}

pub fn mulmod(_memory: &Memory, op1: U256, op2: U256, op3: U256) -> YulOutput<U256> {
    Ok(op1.mul_mod(op2, op3))
}

pub fn signextend(_memory: &Memory, ext: U256, x: U256) -> YulOutput<U256> {
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

// Memory opcodes

pub fn mload(memory: &Memory, address: U256) -> YulOutput<U256> {
    let bytes: Vec<u8> = memory.load(address, U256::from(32));
    let bytes: [u8; 32] = bytes.try_into().unwrap();
    let bytes: FixedBytes<32> = bytes.into();
    Ok(U256::try_from(bytes).unwrap())
}

pub fn mstore(memory: &mut Memory, address: U256, value: U256) -> YulOutput<()> {
    let bytes: [u8; 32] = value.to_be_bytes::<32>();
    memory.store(address, bytes.to_vec());
    Ok(())
}

pub fn mstore8(memory: &mut Memory, address: U256, value: U256) -> YulOutput<()> {
    memory.store(address, vec![value.byte(0)]);
    Ok(())
}

// Precompiles
pub fn staticcall(
    memory: &Memory,
    _gas: U256,
    address: U256,
    argsOffset: U256,
    argsSize: U256,
    retOffset: U256,
    retSize: U256,
) -> YulOutput<U256> {
    // Pre-compiles for the ZK verifier: 2, 5, 6, 7, 8
    Ok(U256::ONE)
}

pub fn main() {
    println!("Hello, world!");
}
