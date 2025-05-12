use alloy_primitives::hex::FromHex;
use alloy_primitives::{FixedBytes, U256};
use core::cmp::Ordering;
use std::collections::HashMap;

mod i256;
#[macro_use]
mod macros;

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

    fn slice_len(&mut self, from: usize, length: usize) -> &[u8] {
        self.inner.resize(from + length, 0);
        &self.inner[from..from + length]
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

    fn store(&mut self, address: U256, value: &[u8]) {
        let address: usize = U256::try_into(address).unwrap();
        for (i, byte) in value.iter().enumerate() {
            self.set_byte(address + i, *byte);
        }
    }
}

#[derive(Debug)]
pub struct Context<CI> {
    pub contract_interactions: std::marker::PhantomData<CI>,
    pub memory: Memory,
    pub immutables: HashMap<U256, U256>,
    pub address: U256,
    pub caller: U256,
    pub callvalue: U256,
    pub gas: U256,
    pub timestamp: U256,
    pub calldata: Vec<u8>,
    pub chain_id: U256,
}

pub trait ContractInteractions {
    fn call(&self, gas: U256, to: U256, payload: &[u8]) -> Vec<u8>;

    fn get_balance(&self, address: U256) -> U256;
}

#[derive(Debug)]
pub struct DummyContractInteractions;

impl ContractInteractions for Context<DummyContractInteractions> {
    fn call(&self, _gas: U256, _to: U256, _payload: &[u8]) -> Vec<u8> {
        vec![]
    }

    fn get_balance(&self, _address: U256) -> U256 {
        U256::ZERO
    }
}

#[derive(Debug)]
pub enum ReturnOrRevert {
    Return { start: U256, length: U256 },
    Revert { start: U256, length: U256 },
}

pub type YulOutput<A> = Result<A, ReturnOrRevert>;

// Pure opcodes

pub fn add<CI>(x: U256, y: U256, _context: &Context<CI>) -> YulOutput<U256> {
    Ok(x.wrapping_add(y))
}

pub fn sub<CI>(x: U256, y: U256, _context: &Context<CI>) -> YulOutput<U256> {
    Ok(x.wrapping_sub(y))
}

pub fn mul<CI>(x: U256, y: U256, _context: &Context<CI>) -> YulOutput<U256> {
    Ok(x.wrapping_mul(y))
}

pub fn div<CI>(x: U256, y: U256, _context: &Context<CI>) -> YulOutput<U256> {
    Ok(if y == U256::ZERO {
        U256::ZERO
    } else {
        x.wrapping_div(y)
    })
}

pub fn sdiv<CI>(x: U256, y: U256, _context: &Context<CI>) -> YulOutput<U256> {
    Ok(i256::i256_div(x, y))
}

pub fn mod_<CI>(x: U256, y: U256, _context: &Context<CI>) -> YulOutput<U256> {
    Ok(if y == U256::ZERO {
        U256::ZERO
    } else {
        x.wrapping_rem(y)
    })
}

pub fn smod<CI>(x: U256, y: U256, _context: &Context<CI>) -> YulOutput<U256> {
    Ok(i256::i256_mod(x, y))
}

pub fn exp<CI>(x: U256, y: U256, _context: &Context<CI>) -> YulOutput<U256> {
    Ok(x.pow(y))
}

pub fn not<CI>(x: U256, _context: &Context<CI>) -> YulOutput<U256> {
    Ok(!x)
}

pub fn lt<CI>(x: U256, y: U256, _context: &Context<CI>) -> YulOutput<U256> {
    Ok(U256::from(x < y))
}

pub fn gt<CI>(x: U256, y: U256, _context: &Context<CI>) -> YulOutput<U256> {
    Ok(U256::from(x > y))
}

pub fn slt<CI>(x: U256, y: U256, _context: &Context<CI>) -> YulOutput<U256> {
    Ok(U256::from(i256::i256_cmp(&x, &y) == Ordering::Less))
}

pub fn sgt<CI>(x: U256, y: U256, _context: &Context<CI>) -> YulOutput<U256> {
    Ok(U256::from(i256::i256_cmp(&x, &y) == Ordering::Greater))
}

pub fn eq<CI>(x: U256, y: U256, _context: &Context<CI>) -> YulOutput<U256> {
    Ok(U256::from(x == y))
}

pub fn iszero<CI>(x: U256, _context: &Context<CI>) -> YulOutput<U256> {
    Ok(U256::from(x.is_zero()))
}

pub fn and<CI>(x: U256, y: U256, _context: &Context<CI>) -> YulOutput<U256> {
    Ok(x & y)
}

pub fn or<CI>(x: U256, y: U256, _context: &Context<CI>) -> YulOutput<U256> {
    Ok(x | y)
}

pub fn xor<CI>(x: U256, y: U256, _context: &Context<CI>) -> YulOutput<U256> {
    Ok(x ^ y)
}

pub fn byte<CI>(op1: U256, op2: U256, _context: &Context<CI>) -> YulOutput<U256> {
    let o1 = as_usize_saturated!(op1);
    if o1 < 32 {
        // `31 - o1` because `byte` returns LE, while we want BE
        Ok(U256::from(op2.byte(31 - o1)))
    } else {
        Ok(U256::ZERO)
    }
}

pub fn shl<CI>(op1: U256, op2: U256, _context: &Context<CI>) -> YulOutput<U256> {
    let shift = as_usize_saturated!(op1);
    if shift < 256 {
        Ok(op2 << shift)
    } else {
        Ok(U256::ZERO)
    }
}

pub fn shr<CI>(op1: U256, op2: U256, _context: &Context<CI>) -> YulOutput<U256> {
    let shift = as_usize_saturated!(op1);
    if shift < 256 {
        Ok(op2 >> shift)
    } else {
        Ok(U256::ZERO)
    }
}

pub fn sar<CI>(op1: U256, op2: U256, _context: &Context<CI>) -> YulOutput<U256> {
    let shift = as_usize_saturated!(op1);
    if shift < 256 {
        Ok(op2.arithmetic_shr(shift))
    } else if op2.bit(255) {
        Ok(U256::MAX)
    } else {
        Ok(U256::ZERO)
    }
}

pub fn addmod<CI>(op1: U256, op2: U256, op3: U256, _context: &Context<CI>) -> YulOutput<U256> {
    Ok(op1.add_mod(op2, op3))
}

pub fn mulmod<CI>(op1: U256, op2: U256, op3: U256, _context: &Context<CI>) -> YulOutput<U256> {
    Ok(op1.mul_mod(op2, op3))
}

pub fn signextend<CI>(ext: U256, x: U256, _context: &Context<CI>) -> YulOutput<U256> {
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

pub fn keccak256<CI>(p: U256, n: U256, _context: &mut Context<CI>) -> YulOutput<U256> {
    let p: usize = U256::try_into(p).unwrap();
    let n: usize = U256::try_into(n).unwrap();
    let slice = _context.memory.slice_len(p, n);
    Ok(alloy_primitives::keccak256(slice).into())
}

pub fn pop<CI>(_x: U256, _context: &Context<CI>) -> YulOutput<()> {
    Ok(())
}

// Memory opcodes

pub fn mload<CI>(address: U256, context: &Context<CI>) -> YulOutput<U256> {
    let bytes: Vec<u8> = context.memory.load(address, U256::from(32));
    let bytes: [u8; 32] = bytes.try_into().unwrap();
    let bytes: FixedBytes<32> = bytes.into();
    Ok(U256::try_from(bytes).unwrap())
}

pub fn mstore<CI>(address: U256, value: U256, context: &mut Context<CI>) -> YulOutput<()> {
    let bytes: [u8; 32] = value.to_be_bytes::<32>();
    context.memory.store(address, &bytes);
    Ok(())
}

pub fn mstore8<CI>(address: U256, value: U256, context: &mut Context<CI>) -> YulOutput<()> {
    context.memory.store(address, &[value.byte(0)]);
    Ok(())
}

pub fn sload<CI>(_p: U256, _context: &Context<CI>) -> YulOutput<U256> {
    unimplemented!()
}

pub fn sstore<CI>(_p: U256, _v: U256, _context: &mut Context<CI>) -> YulOutput<()> {
    unimplemented!()
}

pub fn gas<CI>(_context: &Context<CI>) -> YulOutput<U256> {
    Ok(_context.gas)
}

pub fn address<CI>(context: &Context<CI>) -> YulOutput<U256> {
    Ok(context.address)
}

pub fn balance<CI>(address: U256, context: &Context<CI>) -> YulOutput<U256>
where
    Context<CI>: ContractInteractions,
{
    Ok(context.get_balance(address))
}

pub fn selfbalance<CI>(context: &Context<CI>) -> YulOutput<U256>
where
    Context<CI>: ContractInteractions,
{
    balance(address(context)?, context)
}

pub fn caller<CI>(context: &Context<CI>) -> YulOutput<U256> {
    Ok(context.caller)
}

pub fn callvalue<CI>(context: &Context<CI>) -> YulOutput<U256> {
    Ok(context.callvalue)
}

pub fn calldataload<CI>(p: U256, context: &Context<CI>) -> YulOutput<U256> {
    let p: usize = U256::try_into(p).unwrap();
    Ok(U256::from(context.calldata[p]))
}

pub fn calldatasize<CI>(context: &Context<CI>) -> YulOutput<U256> {
    Ok(U256::from(context.calldata.len()))
}

pub fn calldatacopy<CI>(t: U256, f: U256, s: U256, context: &mut Context<CI>) -> YulOutput<()> {
    let f: usize = U256::try_into(f).unwrap();
    let s: usize = U256::try_into(s).unwrap();
    let t: usize = U256::try_into(t).unwrap();
    for i in 0..s {
        if f + i < context.calldata.len() {
            context.memory.set_byte(t + i, context.calldata[f + i]);
        } else {
            context.memory.set_byte(t + i, 0);
        }
    }
    Ok(())
}

pub fn codesize<CI>(_context: &Context<CI>) -> YulOutput<U256> {
    unimplemented!()
}

pub fn codecopy<CI>(_t: U256, _f: U256, _s: U256, _context: &Context<CI>) -> YulOutput<()> {
    unimplemented!()
}

pub fn extcodesize<CI>(_a: U256, _context: &Context<CI>) -> YulOutput<U256> {
    unimplemented!()
}

pub fn extcodecopy<CI>(
    _a: U256,
    _t: U256,
    _f: U256,
    _s: U256,
    _context: &Context<CI>,
) -> YulOutput<()> {
    unimplemented!()
}

pub fn returndatasize<CI>(_context: &Context<CI>) -> YulOutput<U256> {
    unimplemented!()
}

pub fn returndatacopy<CI>(_t: U256, _f: U256, _s: U256, _context: &Context<CI>) -> YulOutput<()> {
    unimplemented!()
}

pub fn mcopy<CI>(_t: U256, _f: U256, _s: U256, _context: &Context<CI>) -> YulOutput<()> {
    unimplemented!()
}

pub fn extcodehash<CI>(_a: U256, _context: &Context<CI>) -> YulOutput<U256> {
    unimplemented!()
}

pub fn call<CI>(
    _g: U256,
    _a: U256,
    _v: U256,
    _in_: U256,
    _insize: U256,
    _out: U256,
    _outsize: U256,
    _context: &Context<CI>,
) -> YulOutput<U256> {
    unimplemented!()
}

pub fn callcode<CI>(
    _g: U256,
    _a: U256,
    _v: U256,
    _in_: U256,
    _insize: U256,
    _out: U256,
    _outsize: U256,
    _context: &Context<CI>,
) -> YulOutput<U256> {
    unimplemented!()
}

pub fn delegatecall<CI>(
    _g: U256,
    _a: U256,
    _in_: U256,
    _insize: U256,
    _out: U256,
    _outsize: U256,
    _context: &Context<CI>,
) -> YulOutput<U256> {
    unimplemented!()
}

pub fn staticcall<CI>(
    _gas: U256,
    _address: U256,
    _args_offset: U256,
    _args_size: U256,
    _ret_offset: U256,
    _ret_size: U256,
    _context: &Context<CI>,
) -> YulOutput<U256> {
    // Pre-compiles for the ZK verifier: 2, 5, 6, 7, 8
    Ok(U256::ONE)
}

pub fn return_<CI>(offset: U256, size: U256, _context: &mut Context<CI>) -> YulOutput<()> {
    Err(ReturnOrRevert::Return {
        start: offset,
        length: size,
    })
}

pub fn revert<CI>(offset: U256, size: U256, _context: &mut Context<CI>) -> YulOutput<()> {
    Err(ReturnOrRevert::Revert {
        start: offset,
        length: size,
    })
}

pub fn selfdestruct<CI>(_a: U256, _context: &mut Context<CI>) -> YulOutput<()> {
    unimplemented!()
}

pub fn invalid() -> YulOutput<()> {
    unimplemented!()
}

pub fn log0<CI>(_p: U256, _s: U256, _context: &mut Context<CI>) -> YulOutput<()> {
    unimplemented!()
}

pub fn log1<CI>(_p: U256, _s: U256, _t1: U256, _context: &mut Context<CI>) -> YulOutput<()> {
    unimplemented!()
}

pub fn log2<CI>(
    _p: U256,
    _s: U256,
    _t1: U256,
    _t2: U256,
    _context: &mut Context<CI>,
) -> YulOutput<()> {
    unimplemented!()
}

pub fn log3<CI>(
    _p: U256,
    _s: U256,
    _t1: U256,
    _t2: U256,
    _t3: U256,
    _context: &mut Context<CI>,
) -> YulOutput<()> {
    unimplemented!()
}

pub fn log4<CI>(
    _p: U256,
    _s: U256,
    _t1: U256,
    _t2: U256,
    _t3: U256,
    _t4: U256,
    _context: &mut Context<CI>,
) -> YulOutput<()> {
    unimplemented!()
}

pub fn chainid<CI>(context: &Context<CI>) -> YulOutput<U256> {
    Ok(context.chain_id)
}

pub fn basefee<CI>(_context: &Context<CI>) -> YulOutput<U256> {
    unimplemented!()
}

pub fn blobbasefee<CI>(_context: &Context<CI>) -> YulOutput<U256> {
    unimplemented!()
}

pub fn origin<CI>(_context: &Context<CI>) -> YulOutput<U256> {
    unimplemented!()
}

pub fn gasprice<CI>(_context: &Context<CI>) -> YulOutput<U256> {
    unimplemented!()
}

pub fn blockhash<CI>(_b: U256, _context: &Context<CI>) -> YulOutput<U256> {
    unimplemented!()
}

pub fn blobhash<CI>(_i: U256, _context: &Context<CI>) -> YulOutput<U256> {
    unimplemented!()
}

pub fn coinbase<CI>() -> YulOutput<U256> {
    unimplemented!()
}

pub fn timestamp<CI>(context: &Context<CI>) -> YulOutput<U256> {
    Ok(context.timestamp)
}

pub fn number<CI>(_context: &Context<CI>) -> YulOutput<U256> {
    unimplemented!()
}

pub fn difficulty<CI>(_context: &Context<CI>) -> YulOutput<U256> {
    unimplemented!()
}

pub fn prevrandao<CI>(_context: &Context<CI>) -> YulOutput<U256> {
    unimplemented!()
}

pub fn gaslimit<CI>(_context: &Context<CI>) -> YulOutput<U256> {
    unimplemented!()
}

// Special opcodes

pub fn memoryguard<CI>(size: U256, _context: &mut Context<CI>) -> YulOutput<U256> {
    Ok(size)
}

pub fn datasize<CI>(_x: U256, _context: &Context<CI>) -> YulOutput<U256> {
    unimplemented!()
}

pub fn dataoffset<CI>(_x: U256, _context: &Context<CI>) -> YulOutput<U256> {
    unimplemented!()
}

pub fn datacopy<CI>(t: U256, f: U256, s: U256, _context: &Context<CI>) -> YulOutput<()> {
    codecopy(t, f, s, _context)
}

pub fn setimmutable<CI>(
    _offset: U256,
    name: U256,
    value: U256,
    context: &mut Context<CI>,
) -> YulOutput<()> {
    context.immutables.insert(name, value);
    Ok(())
}

pub fn loadimmutable<CI>(name: U256, context: &Context<CI>) -> YulOutput<U256> {
    Ok(context.immutables.get(&name).cloned().unwrap_or(U256::ZERO))
}
