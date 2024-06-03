pub trait PtrBitsOps {
    fn add_bits_offset(self, offset: u32) -> *const u8;
    fn sub_bits_offset(self, offset: u32) -> *const u8;
    fn bits_align(self, align: u32) -> *const u8;
}

pub trait MutPtrBitsOps {
    fn add_bits_offset(self, offset: u32) -> *mut u8;
    fn sub_bits_offset(self, offset: u32) -> *mut u8;
    fn bits_align(self, align: u32) -> *mut u8;
}

macro_rules! prim_impl {
    ($($t:tt)*) => {
        $(
            impl PtrBitsOps for *const $t {
                fn add_bits_offset(self, offset: u32) -> *const u8 {
                    (self as u32 + offset) as *const u8
                }
            
                fn sub_bits_offset(self, offset: u32) -> *const u8 {
                    (self as u32 - offset) as *const u8
                }

                fn bits_align(self, align: u32) -> *const u8 {
                    ((self as u32 + align) & !(align -1)) as *const u8
                }
            }

            impl MutPtrBitsOps for *mut $t {
                fn add_bits_offset(self, offset: u32) -> *mut u8 {
                    (self as u32 + offset) as *mut u8
                }
            
                fn sub_bits_offset(self, offset: u32) -> *mut u8 {
                    (self as u32 - offset) as *mut u8
                }

                fn bits_align(self, align: u32) -> *mut u8 {
                    ((self as u32 + align) & !(align -1)) as *mut u8
                }
            }
        )*
    };
}

prim_impl!(u8 u16 u32 i8 i16 i32);