use alloy_primitives::U256;
use core::cmp::Ordering;

mod i256;
mod macros;

fn add(x: U256, y: U256) -> U256 {
    x.wrapping_add(y)
}

fn sub(x: U256, y: U256) -> U256 {
    x.wrapping_sub(y)
}

fn mul(x: U256, y: U256) -> U256 {
    x.wrapping_mul(y)
}

fn div(x: U256, y: U256) -> U256 {
    if y == U256::ZERO {
        U256::ZERO
    } else {
        x.wrapping_div(y)
    }
}

fn sdiv(x: U256, y: U256) -> U256 {
    i256::i256_div(x, y)
}

fn mod_(x: U256, y: U256) -> U256 {
    if y == U256::ZERO {
        U256::ZERO
    } else {
        x.wrapping_rem(y)
    }
}

fn smod(x: U256, y: U256) -> U256 {
    i256::i256_mod(x, y)
}

fn exp(x: U256, y: U256) -> U256 {
    x.pow(y)
}

fn not(x: U256) -> U256 {
    !x
}

fn lt(x: U256, y: U256) -> U256 {
    U256::from(x < y)
}

fn gt(x: U256, y: U256) -> U256 {
    U256::from(x > y)
}

fn slt(x: U256, y: U256) -> U256 {
    U256::from(i256::i256_cmp(&x, &y) == Ordering::Less)
}

fn sgt(x: U256, y: U256) -> U256 {
    U256::from(i256::i256_cmp(&x, &y) == Ordering::Greater)
}

fn eq(x: U256, y: U256) -> U256 {
    U256::from(x == y)
}

fn iszero(x: U256) -> U256 {
    U256::from(x.is_zero())
}

fn and(x: U256, y: U256) -> U256 {
    x & y
}

fn or(x: U256, y: U256) -> U256 {
    x | y
}

fn xor(x: U256, y: U256) -> U256 {
    x ^ y
}

fn byte(op1: U256, op2: U256) -> U256 {
    let o1 = as_usize_saturated!(op1);
    if o1 < 32 {
        // `31 - o1` because `byte` returns LE, while we want BE
        U256::from(op2.byte(31 - o1))
    } else {
        U256::ZERO
    }
}

fn shl(op1: U256, op2: U256) -> U256 {
    let shift = as_usize_saturated!(op1);
    if shift < 256 {
        op2 << shift
    } else {
        U256::ZERO
    }
}

fn shr(op1: U256, op2: U256) -> U256 {
    let shift = as_usize_saturated!(op1);
    if shift < 256 {
        op2 >> shift
    } else {
        U256::ZERO
    }
}

fn sar(op1: U256, op2: U256) -> U256 {
    let shift = as_usize_saturated!(op1);
    if shift < 256 {
        op2.arithmetic_shr(shift)
    } else if op2.bit(255) {
        U256::MAX
    } else {
        U256::ZERO
    }
}

fn addmod(op1: U256, op2: U256, op3: U256) -> U256 {
    op1.add_mod(op2, op3)
}

fn mulmod(op1: U256, op2: U256, op3: U256) -> U256 {
    op1.mul_mod(op2, op3)
}

fn signextend(ext: U256, x: U256) -> U256 {
    // For 31 we also don't need to do anything.
    if ext < U256::from(31) {
        let ext = ext.as_limbs()[0];
        let bit_index = (8 * ext + 7) as usize;
        let bit = x.bit(bit_index);
        let mask = (U256::from(1) << bit_index) - U256::from(1);
        if bit {
            x | !mask
        } else {
            x & mask
        }
    } else {
        x
    }
}

fn main() {
    println!("Hello, world!");
}
