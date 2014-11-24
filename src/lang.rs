// Lang items and intrinsics


#[lang = "copy"]
trait Copy {}
#[lang = "sized"]
trait Sized {}
#[lang = "sync"]
trait Sync {}

extern "rust-intrinsic" {
    pub fn transmute<F, T>(from: F) -> T;
}
