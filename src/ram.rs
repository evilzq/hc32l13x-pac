#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - desc CR"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x04 - desc ERRADDR"]
    pub erraddr: crate::Reg<erraddr::ERRADDR_SPEC>,
    #[doc = "0x08 - desc IFR"]
    pub ifr: crate::Reg<ifr::IFR_SPEC>,
    #[doc = "0x0c - desc ICLR"]
    pub iclr: crate::Reg<iclr::ICLR_SPEC>,
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "desc CR"]
pub mod cr;
#[doc = "ERRADDR register accessor: an alias for `Reg<ERRADDR_SPEC>`"]
pub type ERRADDR = crate::Reg<erraddr::ERRADDR_SPEC>;
#[doc = "desc ERRADDR"]
pub mod erraddr;
#[doc = "IFR register accessor: an alias for `Reg<IFR_SPEC>`"]
pub type IFR = crate::Reg<ifr::IFR_SPEC>;
#[doc = "desc IFR"]
pub mod ifr;
#[doc = "ICLR register accessor: an alias for `Reg<ICLR_SPEC>`"]
pub type ICLR = crate::Reg<iclr::ICLR_SPEC>;
#[doc = "desc ICLR"]
pub mod iclr;
