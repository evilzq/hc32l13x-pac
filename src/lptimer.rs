#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - desc CNT"]
    pub cnt: crate::Reg<cnt::CNT_SPEC>,
    #[doc = "0x04 - desc ARR"]
    pub arr: crate::Reg<arr::ARR_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - desc CR"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x10 - desc IFR"]
    pub ifr: crate::Reg<ifr::IFR_SPEC>,
    #[doc = "0x14 - desc ICLR"]
    pub iclr: crate::Reg<iclr::ICLR_SPEC>,
}
#[doc = "CNT register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "desc CNT"]
pub mod cnt;
#[doc = "ARR register accessor: an alias for `Reg<ARR_SPEC>`"]
pub type ARR = crate::Reg<arr::ARR_SPEC>;
#[doc = "desc ARR"]
pub mod arr;
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "desc CR"]
pub mod cr;
#[doc = "IFR register accessor: an alias for `Reg<IFR_SPEC>`"]
pub type IFR = crate::Reg<ifr::IFR_SPEC>;
#[doc = "desc IFR"]
pub mod ifr;
#[doc = "ICLR register accessor: an alias for `Reg<ICLR_SPEC>`"]
pub type ICLR = crate::Reg<iclr::ICLR_SPEC>;
#[doc = "desc ICLR"]
pub mod iclr;
