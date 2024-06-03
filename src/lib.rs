#![no_std]

pub trait PtrBitsOps {
    fn add_bits_offset(self, offset: usize) -> *const u8;
    fn sub_bits_offset(self, offset: usize) -> *const u8;
    fn bits_align(self, align: usize) -> *const u8;
}

pub trait MutPtrBitsOps {
    fn add_bits_offset(self, offset: usize) -> *mut u8;
    fn sub_bits_offset(self, offset: usize) -> *mut u8;
    fn bits_align(self, align: usize) -> *mut u8;
}

macro_rules! prim_impl {
    ($($t:tt)*) => {
        $(
            impl PtrBitsOps for *const $t {
                fn add_bits_offset(self, offset: usize) -> *const u8 {
                    (self as usize + offset) as *const u8
                }
            
                fn sub_bits_offset(self, offset: usize) -> *const u8 {
                    (self as usize - offset) as *const u8
                }

                fn bits_align(self, align: usize) -> *const u8 {
                    ((self as usize + align) & !(align -1)) as *const u8
                }
            }

            impl MutPtrBitsOps for *mut $t {
                fn add_bits_offset(self, offset: usize) -> *mut u8 {
                    (self as usize + offset) as *mut u8
                }
            
                fn sub_bits_offset(self, offset: usize) -> *mut u8 {
                    (self as usize - offset) as *mut u8
                }

                fn bits_align(self, align: usize) -> *mut u8 {
                    ((self as usize + align) & !(align -1)) as *mut u8
                }
            }
        )*
    };
}

prim_impl!(usize u8 u16 u32 i8 i16 i32);