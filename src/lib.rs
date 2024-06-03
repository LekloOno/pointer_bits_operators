pub trait PtrBitsOps {
    fn add_bit_offset(self, offset: u32) -> *const u8;
    fn sub_bit_offset(self, offset: u32) -> *const u8;
    fn mut_add_bit_offset(self, offset: u32) -> *mut u8;
    fn mut_sub_bit_offset(self, offset: u32) -> *mut u8;
}

macro_rules! prim_impl {
    ($($t:tt)*) => {
        $(
            impl PtrBitsOps for *const $t {
                fn add_bit_offset(self, offset: u32) -> *const u8 {
                    (self as u32 + offset) as *const u8
                }
            
                fn sub_bit_offset(self, offset: u32) -> *const u8 {
                    (self as u32 - offset) as *const u8
                }

                fn mut_add_bit_offset(self, offset: u32) -> *mut u8 {
                    (self as u32 + offset) as *mut u8
                }
            
                fn mut_sub_bit_offset(self, offset: u32) -> *mut u8 {
                    (self as u32 - offset) as *mut u8
                }
            }

            impl PtrBitsOps for *mut $t {
                fn add_bit_offset(self, offset: u32) -> *const u8 {
                    (self as u32 + offset) as *const u8
                }
            
                fn sub_bit_offset(self, offset: u32) -> *const u8 {
                    (self as u32 - offset) as *const u8
                }

                fn mut_add_bit_offset(self, offset: u32) -> *mut u8 {
                    (self as u32 + offset) as *mut u8
                }
            
                fn mut_sub_bit_offset(self, offset: u32) -> *mut u8 {
                    (self as u32 - offset) as *mut u8
                }
            }
        )*
    };
}

prim_impl!(u8 u16 u32 i8 i16 i32);