#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - desc RESET_FLAG"]
    pub reset_flag: crate::Reg<reset_flag::RESET_FLAG_SPEC>,
    _reserved1: [u8; 0x08],
    #[doc = "0x0c - desc PERI_RESET"]
    pub peri_reset: crate::Reg<peri_reset::PERI_RESET_SPEC>,
}
#[doc = "RESET_FLAG register accessor: an alias for `Reg<RESET_FLAG_SPEC>`"]
pub type RESET_FLAG = crate::Reg<reset_flag::RESET_FLAG_SPEC>;
#[doc = "desc RESET_FLAG"]
pub mod reset_flag;
#[doc = "PERI_RESET register accessor: an alias for `Reg<PERI_RESET_SPEC>`"]
pub type PERI_RESET = crate::Reg<peri_reset::PERI_RESET_SPEC>;
#[doc = "desc PERI_RESET"]
pub mod peri_reset;
