#![allow(warnings)]
use alloy_primitives::U256;
mod opcodes;
use opcodes::*;
mod dialect;
use dialect::*;

fn to_bool(a: U256) -> Result<bool, ReturnOrRevert> {
    Ok(U256::is_zero(&a))
}


fn entrypoint(mem: &mut Memory) -> Result<(), ReturnOrRevert> {
  let _ = mstore(mem, "64".parse().unwrap(), memoryguard(mem, "128".parse().unwrap())?)?;
  if to_bool(iszero(mem, lt(mem, calldatasize(mem)?, "4".parse().unwrap())?)?)? { let mut selector: U256 = shift_right_224_unsigned(mem, calldataload(mem, "0".parse().unwrap())?)?;
    match selector {"0xdde38a34".parse().unwrap() => {external_fun_add_one_13(mem)?;},}; };
  revert_error_42b3090547df1d2001c96683413b8cf91c1b902ef5e3cb8d9f6f304cf7446f74(mem)?;
  return Ok(());
}
fn shift_right_224_unsigned(mem: &mut Memory, mut value: U256) -> Result<U256, ReturnOrRevert> {
  let mut newValue: U256;
  newValue = shr(mem, "224".parse().unwrap(), value)?;
  return Ok(newValue);
}
fn allocate_unbounded(mem: &mut Memory) -> Result<U256, ReturnOrRevert> {
  let mut memPtr: U256;
  memPtr = mload(mem, "64".parse().unwrap())?;
  return Ok(memPtr);
}
fn revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb(mem: &mut Memory) -> Result<(), ReturnOrRevert> {
  revert(mem, "0".parse().unwrap(), "0".parse().unwrap())?;
  return Ok(());
}
fn revert_error_dbdddcbe895c83990c08b3492a0e83918d802a52331272ac6fdb6a7c4aea3b1b(mem: &mut Memory) -> Result<(), ReturnOrRevert> {
  revert(mem, "0".parse().unwrap(), "0".parse().unwrap())?;
  return Ok(());
}
fn revert_error_c1322bf8034eace5e0b5c7295db60986aa89aae5e0ea0873e4689e076861a5db(mem: &mut Memory) -> Result<(), ReturnOrRevert> {
  revert(mem, "0".parse().unwrap(), "0".parse().unwrap())?;
  return Ok(());
}
fn cleanup_t_uint256(mem: &mut Memory, mut value: U256) -> Result<U256, ReturnOrRevert> {
  let mut cleaned: U256;
  cleaned = value;
  return Ok(cleaned);
}
fn validator_revert_t_uint256(mem: &mut Memory, mut value: U256) -> Result<(), ReturnOrRevert> {
  if to_bool(iszero(mem, eq(mem, value, cleanup_t_uint256(mem, value)?)?)?)? { revert(mem, "0".parse().unwrap(), "0".parse().unwrap())?; };
  return Ok(());
}
fn abi_decode_t_uint256(mem: &mut Memory, mut offset: U256, mut end: U256) -> Result<U256, ReturnOrRevert> {
  let mut value: U256;
  value = calldataload(mem, offset)?;
  validator_revert_t_uint256(mem, value)?;
  return Ok(value);
}
fn abi_decode_tuple_t_uint256(mem: &mut Memory, mut headStart: U256, mut dataEnd: U256) -> Result<U256, ReturnOrRevert> {
  let mut value0: U256;
  if to_bool(slt(mem, sub(mem, dataEnd, headStart)?, "32".parse().unwrap())?)? { revert_error_dbdddcbe895c83990c08b3492a0e83918d802a52331272ac6fdb6a7c4aea3b1b(mem)?; };
  { let mut offset: U256 = "0".parse().unwrap();
    value0 = abi_decode_t_uint256(mem, add(mem, headStart, offset)?, dataEnd)?; };
  return Ok(value0);
}
fn abi_encode_t_uint256_to_t_uint256_fromStack(mem: &mut Memory, mut value: U256, mut pos: U256) -> Result<(), ReturnOrRevert> {
  mstore(mem, pos, cleanup_t_uint256(mem, value)?)?;
  return Ok(());
}
fn abi_encode_tuple_t_uint256__to_t_uint256__fromStack(mem: &mut Memory, mut headStart: U256, mut value0: U256) -> Result<U256, ReturnOrRevert> {
  let mut tail: U256;
  tail = add(mem, headStart, "32".parse().unwrap())?;
  abi_encode_t_uint256_to_t_uint256_fromStack(mem, value0, add(mem, headStart, "0".parse().unwrap())?)?;
  return Ok(tail);
}
fn external_fun_add_one_13(mem: &mut Memory) -> Result<(), ReturnOrRevert> {
  if to_bool(callvalue(mem)?)? { revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb(mem)?; };
  let mut param_0: U256 = abi_decode_tuple_t_uint256(mem, "4".parse().unwrap(), calldatasize(mem)?)?;
  let mut ret_0: U256 = fun_add_one_13(mem, param_0)?;
  let mut memPos: U256 = allocate_unbounded(mem)?;
  let mut memEnd: U256 = abi_encode_tuple_t_uint256__to_t_uint256__fromStack(mem, memPos, ret_0)?;
  return_evm(mem, memPos, sub(mem, memEnd, memPos)?)?;
  return Ok(());
}
fn revert_error_42b3090547df1d2001c96683413b8cf91c1b902ef5e3cb8d9f6f304cf7446f74(mem: &mut Memory) -> Result<(), ReturnOrRevert> {
  revert(mem, "0".parse().unwrap(), "0".parse().unwrap())?;
  return Ok(());
}
fn zero_value_for_split_t_uint256(mem: &mut Memory) -> Result<U256, ReturnOrRevert> {
  let mut ret: U256;
  ret = "0".parse().unwrap();
  return Ok(ret);
}
fn cleanup_t_rational_1_by_1(mem: &mut Memory, mut value: U256) -> Result<U256, ReturnOrRevert> {
  let mut cleaned: U256;
  cleaned = value;
  return Ok(cleaned);
}
fn identity(mem: &mut Memory, mut value: U256) -> Result<U256, ReturnOrRevert> {
  let mut ret: U256;
  ret = value;
  return Ok(ret);
}
fn convert_t_rational_1_by_1_to_t_uint256(mem: &mut Memory, mut value: U256) -> Result<U256, ReturnOrRevert> {
  let mut converted: U256;
  converted = cleanup_t_uint256(mem, identity(mem, cleanup_t_rational_1_by_1(mem, value)?)?)?;
  return Ok(converted);
}
fn panic_error_0x11(mem: &mut Memory) -> Result<(), ReturnOrRevert> {
  mstore(mem, "0".parse().unwrap(), "35408467139433450592217433187231851964531694900788300625387963629091585785856".parse().unwrap())?;
  mstore(mem, "4".parse().unwrap(), "0x11".parse().unwrap())?;
  revert(mem, "0".parse().unwrap(), "0x24".parse().unwrap())?;
  return Ok(());
}
fn checked_add_t_uint256(mem: &mut Memory, mut x: U256, mut y: U256) -> Result<U256, ReturnOrRevert> {
  let mut sum: U256;
  x = cleanup_t_uint256(mem, x)?;
  y = cleanup_t_uint256(mem, y)?;
  sum = add(mem, x, y)?;
  if to_bool(gt(mem, x, sum)?)? { panic_error_0x11(mem)?; };
  return Ok(sum);
}
fn fun_add_one_13(mem: &mut Memory, mut var_x_3: U256) -> Result<U256, ReturnOrRevert> {
  let mut var__6: U256;
  let mut zero_t_uint256_1: U256 = zero_value_for_split_t_uint256(mem)?;
  var__6 = zero_t_uint256_1;
  let mut _2: U256 = var_x_3;
  let mut expr_8: U256 = _2;
  let mut expr_9: U256 = "0x01".parse().unwrap();
  let mut expr_10: U256 = checked_add_t_uint256(mem, expr_8, convert_t_rational_1_by_1_to_t_uint256(mem, expr_9)?)?;
  var__6 = expr_10;
  return Ok(var__6);
}
