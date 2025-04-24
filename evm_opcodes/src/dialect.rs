use crate::Memory;
use crate::YulOutput;
use crate::U256;

// FIXME: A lot of those are stubs

pub fn memoryguard(mem: &mut Memory, a: U256) -> YulOutput<U256> {
  Ok(a)
}

pub fn calldatasize(mem: &mut Memory) -> YulOutput<U256> {
 Ok(U256::ZERO)
}

pub fn calldataload(mem: &mut Memory, _: U256) -> YulOutput<U256> {
 Ok(U256::ZERO)
}

pub fn revert(mem: &mut Memory, _: U256, _: U256) -> YulOutput<()> {
  Ok(()) 
}

pub fn callvalue(mem: &mut Memory) -> YulOutput<U256> {
  Ok(U256::ZERO)
}

pub fn return_evm(mem: &mut Memory, _: U256, _: U256) -> YulOutput<()> {
  Ok(())
}
