#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - desc CR"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x04 - desc SSN"]
    pub ssn: crate::Reg<ssn::SSN_SPEC>,
    #[doc = "0x08 - desc STAT"]
    pub stat: crate::Reg<stat::STAT_SPEC>,
    #[doc = "0x0c - desc DATA"]
    pub data: crate::Reg<data::DATA_SPEC>,
    #[doc = "0x10 - desc CR2"]
    pub cr2: crate::Reg<cr2::CR2_SPEC>,
    #[doc = "0x14 - desc ICLR"]
    pub iclr: crate::Reg<iclr::ICLR_SPEC>,
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "desc CR"]
pub mod cr;
#[doc = "SSN register accessor: an alias for `Reg<SSN_SPEC>`"]
pub type SSN = crate::Reg<ssn::SSN_SPEC>;
#[doc = "desc SSN"]
pub mod ssn;
#[doc = "STAT register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "desc STAT"]
pub mod stat;
#[doc = "DATA register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "desc DATA"]
pub mod data;
#[doc = "CR2 register accessor: an alias for `Reg<CR2_SPEC>`"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "desc CR2"]
pub mod cr2;
#[doc = "ICLR register accessor: an alias for `Reg<ICLR_SPEC>`"]
pub type ICLR = crate::Reg<iclr::ICLR_SPEC>;
#[doc = "desc ICLR"]
pub mod iclr;
