#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - desc CNT"]
    pub cnt: crate::Reg<cnt::CNT_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x0c - desc M1CR"]
    pub m1cr: crate::Reg<m1cr::M1CR_SPEC>,
    #[doc = "0x10 - desc IFR"]
    pub ifr: crate::Reg<ifr::IFR_SPEC>,
    #[doc = "0x14 - desc ICLR"]
    pub iclr: crate::Reg<iclr::ICLR_SPEC>,
    #[doc = "0x18 - desc MSCR"]
    pub mscr: crate::Reg<mscr::MSCR_SPEC>,
    #[doc = "0x1c - desc FLTR"]
    pub fltr: crate::Reg<fltr::FLTR_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x24 - desc CR0"]
    pub cr0: crate::Reg<cr0::CR0_SPEC>,
    _reserved7: [u8; 0x14],
    #[doc = "0x3c - desc CCR0A"]
    pub ccr0a: crate::Reg<ccr0a::CCR0A_SPEC>,
}
#[doc = "CNT register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "desc CNT"]
pub mod cnt;
#[doc = "M1CR register accessor: an alias for `Reg<M1CR_SPEC>`"]
pub type M1CR = crate::Reg<m1cr::M1CR_SPEC>;
#[doc = "desc M1CR"]
pub mod m1cr;
#[doc = "IFR register accessor: an alias for `Reg<IFR_SPEC>`"]
pub type IFR = crate::Reg<ifr::IFR_SPEC>;
#[doc = "desc IFR"]
pub mod ifr;
#[doc = "ICLR register accessor: an alias for `Reg<ICLR_SPEC>`"]
pub type ICLR = crate::Reg<iclr::ICLR_SPEC>;
#[doc = "desc ICLR"]
pub mod iclr;
#[doc = "MSCR register accessor: an alias for `Reg<MSCR_SPEC>`"]
pub type MSCR = crate::Reg<mscr::MSCR_SPEC>;
#[doc = "desc MSCR"]
pub mod mscr;
#[doc = "FLTR register accessor: an alias for `Reg<FLTR_SPEC>`"]
pub type FLTR = crate::Reg<fltr::FLTR_SPEC>;
#[doc = "desc FLTR"]
pub mod fltr;
#[doc = "CR0 register accessor: an alias for `Reg<CR0_SPEC>`"]
pub type CR0 = crate::Reg<cr0::CR0_SPEC>;
#[doc = "desc CR0"]
pub mod cr0;
#[doc = "CCR0A register accessor: an alias for `Reg<CCR0A_SPEC>`"]
pub type CCR0A = crate::Reg<ccr0a::CCR0A_SPEC>;
#[doc = "desc CCR0A"]
pub mod ccr0a;
