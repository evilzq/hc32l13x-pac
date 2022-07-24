#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - desc ARR"]
    pub arr: crate::Reg<arr::ARR_SPEC>,
    #[doc = "0x04 - desc CNT"]
    pub cnt: crate::Reg<cnt::CNT_SPEC>,
    #[doc = "0x08 - desc CNT32"]
    pub cnt32: crate::Reg<cnt32::CNT32_SPEC>,
    #[doc = "0x0c - desc M0CR"]
    pub m0cr: crate::Reg<m0cr::M0CR_SPEC>,
    #[doc = "0x10 - desc IFR"]
    pub ifr: crate::Reg<ifr::IFR_SPEC>,
    #[doc = "0x14 - desc ICLR"]
    pub iclr: crate::Reg<iclr::ICLR_SPEC>,
    _reserved6: [u8; 0x18],
    #[doc = "0x30 - desc DTR"]
    pub dtr: crate::Reg<dtr::DTR_SPEC>,
}
#[doc = "ARR register accessor: an alias for `Reg<ARR_SPEC>`"]
pub type ARR = crate::Reg<arr::ARR_SPEC>;
#[doc = "desc ARR"]
pub mod arr;
#[doc = "CNT register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "desc CNT"]
pub mod cnt;
#[doc = "CNT32 register accessor: an alias for `Reg<CNT32_SPEC>`"]
pub type CNT32 = crate::Reg<cnt32::CNT32_SPEC>;
#[doc = "desc CNT32"]
pub mod cnt32;
#[doc = "M0CR register accessor: an alias for `Reg<M0CR_SPEC>`"]
pub type M0CR = crate::Reg<m0cr::M0CR_SPEC>;
#[doc = "desc M0CR"]
pub mod m0cr;
#[doc = "IFR register accessor: an alias for `Reg<IFR_SPEC>`"]
pub type IFR = crate::Reg<ifr::IFR_SPEC>;
#[doc = "desc IFR"]
pub mod ifr;
#[doc = "ICLR register accessor: an alias for `Reg<ICLR_SPEC>`"]
pub type ICLR = crate::Reg<iclr::ICLR_SPEC>;
#[doc = "desc ICLR"]
pub mod iclr;
#[doc = "DTR register accessor: an alias for `Reg<DTR_SPEC>`"]
pub type DTR = crate::Reg<dtr::DTR_SPEC>;
#[doc = "desc DTR"]
pub mod dtr;
