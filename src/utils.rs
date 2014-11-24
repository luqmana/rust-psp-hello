use lang::transmute;

#[link(name = "pspdebug")]
#[link(name = "pspdisplay")]
#[link(name = "pspge")]
#[link(name = "c")]
#[link(name = "pspuser")]
#[link(name = "gcc")]
extern {
    fn pspDebugScreenInit();
    fn pspDebugScreenPrintf(format: *const u8, ...);
}

pub fn debug_init() {
    unsafe {
        pspDebugScreenInit();
    }
}

pub fn debug_print(s: &str) {
    unsafe {
        // Get the pointer & size for the passed in string
        let (ptr, sz): (*const u8, i32) = transmute(s);

        // The format string for printf
        let (fptr, _): (*const u8, i32) = transmute("%.*s\0");

        pspDebugScreenPrintf(fptr, sz, ptr);
    }
}
