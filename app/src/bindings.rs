// Generated by `wit-bindgen` 0.24.0. DO NOT EDIT!
// Options used:
#[allow(dead_code)]
pub mod dkwr {
    #[allow(dead_code)]
    pub mod stringlength {
        #[allow(dead_code, clippy::all)]
        pub mod len {
            #[used]
            #[doc(hidden)]
            #[cfg(target_arch = "wasm32")]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            #[allow(unused_unsafe, clippy::all)]
            pub fn len(s: &str) -> u32 {
                unsafe {
                    let vec0 = s;
                    let ptr0 = vec0.as_ptr().cast::<u8>();
                    let len0 = vec0.len();

                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "dkwr:stringlength/len@0.1.0")]
                    extern "C" {
                        #[link_name = "len"]
                        fn wit_import(_: *mut u8, _: usize) -> i32;
                    }

                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8, _: usize) -> i32 {
                        unreachable!()
                    }
                    let ret = wit_import(ptr0.cast_mut(), len0);
                    ret as u32
                }
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.24.0:app:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 201] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07P\x01A\x02\x01A\x02\x01\
B\x02\x01@\x01\x01ss\0y\x04\0\x03len\x01\0\x03\x01\x1bdkwr:stringlength/len@0.1.\
0\x05\0\x04\x01\x12dkwr:app/app@0.1.0\x04\0\x0b\x09\x01\0\x03app\x03\0\0\0G\x09p\
roducers\x01\x0cprocessed-by\x02\x0dwit-component\x070.202.0\x10wit-bindgen-rust\
\x060.24.0";

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
