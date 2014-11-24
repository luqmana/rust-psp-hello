#[repr(C)]
pub struct SceModuleInfo {
    pub mod_attribute: u16,
    pub mod_version: [u8, ..2],
    pub mod_name: [i8, ..27],
    pub terminal: i8,
    pub gp_value: *const (),
    pub ent_top: *const (),
    pub ent_end: *const (),
    pub stub_top: *const (),
    pub stub_end: *const ()
}

#[repr(u16)]
pub enum Mode {
    USER   = 0,
    KERNEL = 1,
}

// XXX - Find a way to also accept a name
macro_rules! PSP_MODULE_INFO (
    ($mode:expr, $major_version:expr, $minor_version:expr) => (
        extern {
            static _gp: *const i8;
            static __lib_ent_top: *const i8;
            static __lib_ent_bottom: *const ();
            static __lib_stub_top: *const ();
            static __lib_stub_bottom: *const ();
        }

        #[no_mangle]
        #[link_section = ".rodata.sceModuleInfo"]
        #[linkage = "external"]
        #[allow(non_upper_case_globals)]
        pub static module_info: ::raw::SceModuleInfo = ::raw::SceModuleInfo {
            mod_attribute: $mode as u16,
            mod_version: [$major_version, $minor_version],
            mod_name: [0, ..27], // XXX - should be able to specify this...
            terminal: 0,
            gp_value: &_gp as *const _ as *const (),
            ent_top: &__lib_ent_top as *const _ as *const (),
            ent_end: &__lib_ent_bottom as *const _ as *const (),
            stub_top: &__lib_stub_top as *const _ as *const (),
            stub_end: &__lib_stub_bottom as *const _ as *const (),
        };
    )
)
