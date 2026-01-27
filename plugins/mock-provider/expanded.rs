#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use hydrust_sdk::register_plugin;
#[doc(hidden)]
use ::hydrust_sdk::wit_bindgen as wit_bindgen;
pub type StreamInfo = hydrust::protocol::types::StreamInfo;
pub type ErrorCode = hydrust::protocol::types::ErrorCode;
#[doc(hidden)]
#[allow(non_snake_case, unused_unsafe)]
pub unsafe fn _export_can_handle_cabi<T: Guest>(arg0: *mut u8, arg1: usize) -> i32 {
    unsafe {
        let result1 = {
            let len0 = arg1;
            let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
            T::can_handle(_rt::string_lift(bytes0))
        };
        match result1 {
            true => 1,
            false => 0,
        }
    }
}
#[doc(hidden)]
#[allow(non_snake_case, unused_unsafe)]
pub unsafe fn _export_get_stream_cabi<T: Guest>(arg0: *mut u8, arg1: usize) -> *mut u8 {
    unsafe {
        let result1 = {
            let len0 = arg1;
            let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
            T::get_stream(_rt::string_lift(bytes0))
        };
        let ptr2 = (&raw mut _RET_AREA.0).cast::<u8>();
        match result1 {
            Ok(e) => {
                *ptr2.add(0).cast::<u8>() = (0i32) as u8;
                let hydrust::protocol::types::StreamInfo { title: title3, url: url3 } = e;
                let vec4 = (title3.into_bytes()).into_boxed_slice();
                let ptr4 = vec4.as_ptr().cast::<u8>();
                let len4 = vec4.len();
                ::core::mem::forget(vec4);
                *ptr2.add(2 * ::core::mem::size_of::<*const u8>()).cast::<usize>() = len4;
                *ptr2.add(::core::mem::size_of::<*const u8>()).cast::<*mut u8>() = ptr4
                    .cast_mut();
                let vec5 = (url3.into_bytes()).into_boxed_slice();
                let ptr5 = vec5.as_ptr().cast::<u8>();
                let len5 = vec5.len();
                ::core::mem::forget(vec5);
                *ptr2.add(4 * ::core::mem::size_of::<*const u8>()).cast::<usize>() = len5;
                *ptr2.add(3 * ::core::mem::size_of::<*const u8>()).cast::<*mut u8>() = ptr5
                    .cast_mut();
            }
            Err(e) => {
                *ptr2.add(0).cast::<u8>() = (1i32) as u8;
                *ptr2.add(::core::mem::size_of::<*const u8>()).cast::<u8>() = (e.clone()
                    as i32) as u8;
            }
        };
        ptr2
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn __post_return_get_stream<T: Guest>(arg0: *mut u8) {
    unsafe {
        let l0 = i32::from(*arg0.add(0).cast::<u8>());
        match l0 {
            0 => {
                let l1 = *arg0
                    .add(::core::mem::size_of::<*const u8>())
                    .cast::<*mut u8>();
                let l2 = *arg0
                    .add(2 * ::core::mem::size_of::<*const u8>())
                    .cast::<usize>();
                _rt::cabi_dealloc(l1, l2, 1);
                let l3 = *arg0
                    .add(3 * ::core::mem::size_of::<*const u8>())
                    .cast::<*mut u8>();
                let l4 = *arg0
                    .add(4 * ::core::mem::size_of::<*const u8>())
                    .cast::<usize>();
                _rt::cabi_dealloc(l3, l4, 1);
            }
            _ => {}
        }
    }
}
pub trait Guest {
    #[allow(async_fn_in_trait)]
    fn can_handle(url: _rt::String) -> bool;
    #[allow(async_fn_in_trait)]
    fn get_stream(url: _rt::String) -> Result<StreamInfo, ErrorCode>;
}
#[doc(hidden)]
pub(crate) use __export_world_site_provider_cabi;
#[repr(align(8))]
struct _RetArea([::core::mem::MaybeUninit<u8>; 5 * ::core::mem::size_of::<*const u8>()]);
static mut _RET_AREA: _RetArea = _RetArea(
    [::core::mem::MaybeUninit::uninit(); 5 * ::core::mem::size_of::<*const u8>()],
);
#[allow(dead_code, clippy::all)]
pub mod hydrust {
    pub mod protocol {
        #[allow(dead_code, async_fn_in_trait, unused_imports, clippy::all)]
        pub mod types {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub struct StreamInfo {
                pub title: _rt::String,
                pub url: _rt::String,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for StreamInfo {
                #[inline]
                fn clone(&self) -> StreamInfo {
                    StreamInfo {
                        title: ::core::clone::Clone::clone(&self.title),
                        url: ::core::clone::Clone::clone(&self.url),
                    }
                }
            }
            impl ::core::fmt::Debug for StreamInfo {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("StreamInfo")
                        .field("title", &self.title)
                        .field("url", &self.url)
                        .finish()
                }
            }
            #[repr(u8)]
            pub enum ErrorCode {
                NetworkError,
                InvalidUrl,
                Other,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ErrorCode {
                #[inline]
                fn clone(&self) -> ErrorCode {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for ErrorCode {}
            #[automatically_derived]
            impl ::core::cmp::Eq for ErrorCode {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {}
            }
            #[automatically_derived]
            impl ::core::cmp::Ord for ErrorCode {
                #[inline]
                fn cmp(&self, other: &ErrorCode) -> ::core::cmp::Ordering {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ErrorCode {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ErrorCode {
                #[inline]
                fn eq(&self, other: &ErrorCode) -> bool {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    __self_discr == __arg1_discr
                }
            }
            #[automatically_derived]
            impl ::core::cmp::PartialOrd for ErrorCode {
                #[inline]
                fn partial_cmp(
                    &self,
                    other: &ErrorCode,
                ) -> ::core::option::Option<::core::cmp::Ordering> {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
                }
            }
            impl ErrorCode {
                pub fn name(&self) -> &'static str {
                    match self {
                        ErrorCode::NetworkError => "network-error",
                        ErrorCode::InvalidUrl => "invalid-url",
                        ErrorCode::Other => "other",
                    }
                }
                pub fn message(&self) -> &'static str {
                    match self {
                        ErrorCode::NetworkError => "",
                        ErrorCode::InvalidUrl => "",
                        ErrorCode::Other => "",
                    }
                }
            }
            impl ::core::fmt::Debug for ErrorCode {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("ErrorCode")
                        .field("code", &(*self as i32))
                        .field("name", &self.name())
                        .field("message", &self.message())
                        .finish()
                }
            }
            impl ::core::fmt::Display for ErrorCode {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.write_fmt(
                        format_args!("{0} (error {1})", self.name(), *self as i32),
                    )
                }
            }
            impl ::core::error::Error for ErrorCode {}
            impl ErrorCode {
                #[doc(hidden)]
                pub unsafe fn _lift(val: u8) -> ErrorCode {
                    if !true {
                        return unsafe { ::core::mem::transmute(val) };
                    }
                    match val {
                        0 => ErrorCode::NetworkError,
                        1 => ErrorCode::InvalidUrl,
                        2 => ErrorCode::Other,
                        _ => {
                            ::core::panicking::panic_fmt(
                                format_args!("invalid enum discriminant"),
                            );
                        }
                    }
                }
            }
        }
    }
}
mod _rt {
    #![allow(dead_code, unused_imports, clippy::all)]
    pub use alloc_crate::string::String;
    pub use alloc_crate::vec::Vec;
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if true {
            String::from_utf8(bytes).unwrap()
        } else {
            unsafe { String::from_utf8_unchecked(bytes) }
        }
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        unsafe {
            let layout = alloc::Layout::from_size_align_unchecked(size, align);
            alloc::dealloc(ptr, layout);
        }
    }
    extern crate alloc as alloc_crate;
    pub use alloc_crate::alloc;
}
#[doc(inline)]
pub(crate) use __export_site_provider_impl as export;
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen::maybe_link_cabi_realloc();
}
const _: () = {
    #[unsafe(export_name = "can-handle")]
    unsafe extern "C" fn export_can_handle(arg0: *mut u8, arg1: usize) -> i32 {
        unsafe { self::_export_can_handle_cabi::<MockPlugin>(arg0, arg1) }
    }
    #[unsafe(export_name = "get-stream")]
    unsafe extern "C" fn export_get_stream(arg0: *mut u8, arg1: usize) -> *mut u8 {
        unsafe { self::_export_get_stream_cabi::<MockPlugin>(arg0, arg1) }
    }
    #[unsafe(export_name = "cabi_post_get-stream")]
    unsafe extern "C" fn _post_return_get_stream(arg0: *mut u8) {
        unsafe { self::__post_return_get_stream::<MockPlugin>(arg0) }
    }
};
struct MockPlugin;
impl Guest for MockPlugin {
    fn can_handle(url: String) -> bool {
        url.contains("example.com")
    }
    fn get_stream(_url: String) -> Result<StreamInfo, ErrorCode> {
        Ok(StreamInfo {
            title: "Hydrust Mock Stream".to_string(),
            url: "https://test-streams.mux.dev/x36xhzz/x36xhzz.m3u8".to_string(),
        })
    }
}
