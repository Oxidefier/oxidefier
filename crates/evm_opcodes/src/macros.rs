/// Converts a `U256` value to a `u64`, saturating to `MAX` if the value is too large.
#[macro_export]
macro_rules! as_u64_saturated {
    ($v:expr) => {
        match $v.as_limbs() {
            x => {
                if (x[1] == 0) & (x[2] == 0) & (x[3] == 0) {
                    x[0]
                } else {
                    u64::MAX
                }
            }
        }
    };
}

/// Converts a `U256` value to a `usize`, saturating to `MAX` if the value is too large.
#[macro_export]
macro_rules! as_usize_saturated {
    ($v:expr) => {
        usize::try_from($crate::as_u64_saturated!($v)).unwrap_or(usize::MAX)
    };
}
