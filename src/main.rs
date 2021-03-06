#![feature(lang_items)]
#![feature(start)]
#![no_std]
#![no_main]

// Pull in the system libc library for what crt0.o likely requires
// extern crate libc;

// Entry point for this program
#[no_mangle] // ensure that this symbol is called `main` in the output
pub extern fn main() {
    0;
}
// pub extern fn main(_argc: i32, _argv: *const *const u8) -> i32 {
//     0
// }

// These functions are used by the compiler, but not
// for a bare-bones hello world. These are normally
// provided by libstd.
#[lang = "eh_personality"]
#[no_mangle]
pub extern fn rust_eh_personality() {
}

// This function may be needed based on the compilation target.
#[lang = "eh_unwind_resume"]
#[no_mangle]
pub extern fn rust_eh_unwind_resume() {
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(_msg: core::fmt::Arguments,
                               _file: &'static str,
                               _line: u32) -> ! {
    loop {}
}