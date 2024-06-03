# pointer_bits_operators.rs

This lib offers quick hand tools to manipulate raw pointer offsets in bits, instead of bytes.
It currently always returns `u8` pointers as it is the type used within pip-mpu, which is the context in which this crate has been built.
