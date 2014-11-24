#![no_std]
#![no_main]
#![feature(intrinsics, lang_items, linkage, macro_rules)]
#![allow(dead_code)]

mod lang;
#[macro_escape]
mod raw;
mod utils;

const VERSION_MAJOR: u8 = 0;
const VERSION_MINOR: u8 = 1;

PSP_MODULE_INFO!(raw::Mode::USER, VERSION_MAJOR, VERSION_MINOR)


#[no_mangle]
pub extern "C" fn main() {
    utils::debug_init();
    utils::debug_print("Hello World!");
}
