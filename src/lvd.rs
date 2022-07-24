#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x28],
    #[doc = "0x28 - desc CR"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x2c - desc IFR"]
    pub ifr: crate::Reg<ifr::IFR_SPEC>,
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "desc CR"]
pub mod cr;
#[doc = "IFR register accessor: an alias for `Reg<IFR_SPEC>`"]
pub type IFR = crate::Reg<ifr::IFR_SPEC>;
#[doc = "desc IFR"]
pub mod ifr;
