#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - desc TMRUN"]
    pub tmrun: crate::Reg<tmrun::TMRUN_SPEC>,
    #[doc = "0x04 - desc TM"]
    pub tm: crate::Reg<tm::TM_SPEC>,
    #[doc = "0x08 - desc CR"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x0c - desc DATA"]
    pub data: crate::Reg<data::DATA_SPEC>,
    #[doc = "0x10 - desc ADDR"]
    pub addr: crate::Reg<addr::ADDR_SPEC>,
    #[doc = "0x14 - desc STAT"]
    pub stat: crate::Reg<stat::STAT_SPEC>,
}
#[doc = "TMRUN register accessor: an alias for `Reg<TMRUN_SPEC>`"]
pub type TMRUN = crate::Reg<tmrun::TMRUN_SPEC>;
#[doc = "desc TMRUN"]
pub mod tmrun;
#[doc = "TM register accessor: an alias for `Reg<TM_SPEC>`"]
pub type TM = crate::Reg<tm::TM_SPEC>;
#[doc = "desc TM"]
pub mod tm;
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "desc CR"]
pub mod cr;
#[doc = "DATA register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "desc DATA"]
pub mod data;
#[doc = "ADDR register accessor: an alias for `Reg<ADDR_SPEC>`"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "desc ADDR"]
pub mod addr;
#[doc = "STAT register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "desc STAT"]
pub mod stat;
