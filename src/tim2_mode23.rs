#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - desc ARR"]
    pub arr: crate::Reg<arr::ARR_SPEC>,
    #[doc = "0x04 - desc CNT"]
    pub cnt: crate::Reg<cnt::CNT_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - desc M23CR"]
    pub m23cr: crate::Reg<m23cr::M23CR_SPEC>,
    #[doc = "0x10 - desc IFR"]
    pub ifr: crate::Reg<ifr::IFR_SPEC>,
    #[doc = "0x14 - desc ICLR"]
    pub iclr: crate::Reg<iclr::ICLR_SPEC>,
    #[doc = "0x18 - desc MSCR"]
    pub mscr: crate::Reg<mscr::MSCR_SPEC>,
    #[doc = "0x1c - desc FLTR"]
    pub fltr: crate::Reg<fltr::FLTR_SPEC>,
    #[doc = "0x20 - desc ADTR"]
    pub adtr: crate::Reg<adtr::ADTR_SPEC>,
    #[doc = "0x24 - desc CRCH0"]
    pub crch0: crate::Reg<crch0::CRCH0_SPEC>,
    _reserved9: [u8; 0x08],
    #[doc = "0x30 - desc DTR"]
    pub dtr: crate::Reg<dtr::DTR_SPEC>,
    #[doc = "0x34 - desc RCR"]
    pub rcr: crate::Reg<rcr::RCR_SPEC>,
    #[doc = "0x38 - desc ARRDM"]
    pub arrdm: crate::Reg<arrdm::ARRDM_SPEC>,
    #[doc = "0x3c - desc CCR0A"]
    pub ccr0a: crate::Reg<ccr0a::CCR0A_SPEC>,
    #[doc = "0x40 - desc CCR0B"]
    pub ccr0b: crate::Reg<ccr0b::CCR0B_SPEC>,
}
#[doc = "ARR register accessor: an alias for `Reg<ARR_SPEC>`"]
pub type ARR = crate::Reg<arr::ARR_SPEC>;
#[doc = "desc ARR"]
pub mod arr;
#[doc = "CNT register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "desc CNT"]
pub mod cnt;
#[doc = "M23CR register accessor: an alias for `Reg<M23CR_SPEC>`"]
pub type M23CR = crate::Reg<m23cr::M23CR_SPEC>;
#[doc = "desc M23CR"]
pub mod m23cr;
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
#[doc = "ADTR register accessor: an alias for `Reg<ADTR_SPEC>`"]
pub type ADTR = crate::Reg<adtr::ADTR_SPEC>;
#[doc = "desc ADTR"]
pub mod adtr;
#[doc = "CRCH0 register accessor: an alias for `Reg<CRCH0_SPEC>`"]
pub type CRCH0 = crate::Reg<crch0::CRCH0_SPEC>;
#[doc = "desc CRCH0"]
pub mod crch0;
#[doc = "DTR register accessor: an alias for `Reg<DTR_SPEC>`"]
pub type DTR = crate::Reg<dtr::DTR_SPEC>;
#[doc = "desc DTR"]
pub mod dtr;
#[doc = "RCR register accessor: an alias for `Reg<RCR_SPEC>`"]
pub type RCR = crate::Reg<rcr::RCR_SPEC>;
#[doc = "desc RCR"]
pub mod rcr;
#[doc = "ARRDM register accessor: an alias for `Reg<ARRDM_SPEC>`"]
pub type ARRDM = crate::Reg<arrdm::ARRDM_SPEC>;
#[doc = "desc ARRDM"]
pub mod arrdm;
#[doc = "CCR0A register accessor: an alias for `Reg<CCR0A_SPEC>`"]
pub type CCR0A = crate::Reg<ccr0a::CCR0A_SPEC>;
#[doc = "desc CCR0A"]
pub mod ccr0a;
#[doc = "CCR0B register accessor: an alias for `Reg<CCR0B_SPEC>`"]
pub type CCR0B = crate::Reg<ccr0b::CCR0B_SPEC>;
#[doc = "desc CCR0B"]
pub mod ccr0b;
