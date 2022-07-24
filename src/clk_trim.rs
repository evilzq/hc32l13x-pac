#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - desc CR"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x04 - desc REFCON"]
    pub refcon: crate::Reg<refcon::REFCON_SPEC>,
    #[doc = "0x08 - desc REFCNT"]
    pub refcnt: crate::Reg<refcnt::REFCNT_SPEC>,
    #[doc = "0x0c - desc CALCNT"]
    pub calcnt: crate::Reg<calcnt::CALCNT_SPEC>,
    #[doc = "0x10 - desc IFR"]
    pub ifr: crate::Reg<ifr::IFR_SPEC>,
    #[doc = "0x14 - desc ICLR"]
    pub iclr: crate::Reg<iclr::ICLR_SPEC>,
    #[doc = "0x18 - desc CALCON"]
    pub calcon: crate::Reg<calcon::CALCON_SPEC>,
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "desc CR"]
pub mod cr;
#[doc = "REFCON register accessor: an alias for `Reg<REFCON_SPEC>`"]
pub type REFCON = crate::Reg<refcon::REFCON_SPEC>;
#[doc = "desc REFCON"]
pub mod refcon;
#[doc = "REFCNT register accessor: an alias for `Reg<REFCNT_SPEC>`"]
pub type REFCNT = crate::Reg<refcnt::REFCNT_SPEC>;
#[doc = "desc REFCNT"]
pub mod refcnt;
#[doc = "CALCNT register accessor: an alias for `Reg<CALCNT_SPEC>`"]
pub type CALCNT = crate::Reg<calcnt::CALCNT_SPEC>;
#[doc = "desc CALCNT"]
pub mod calcnt;
#[doc = "IFR register accessor: an alias for `Reg<IFR_SPEC>`"]
pub type IFR = crate::Reg<ifr::IFR_SPEC>;
#[doc = "desc IFR"]
pub mod ifr;
#[doc = "ICLR register accessor: an alias for `Reg<ICLR_SPEC>`"]
pub type ICLR = crate::Reg<iclr::ICLR_SPEC>;
#[doc = "desc ICLR"]
pub mod iclr;
#[doc = "CALCON register accessor: an alias for `Reg<CALCON_SPEC>`"]
pub type CALCON = crate::Reg<calcon::CALCON_SPEC>;
#[doc = "desc CALCON"]
pub mod calcon;
