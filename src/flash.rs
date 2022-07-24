#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - desc TNVS"]
    pub tnvs: crate::Reg<tnvs::TNVS_SPEC>,
    #[doc = "0x04 - desc TPGS"]
    pub tpgs: crate::Reg<tpgs::TPGS_SPEC>,
    #[doc = "0x08 - desc TPROG"]
    pub tprog: crate::Reg<tprog::TPROG_SPEC>,
    #[doc = "0x0c - desc TSERASE"]
    pub tserase: crate::Reg<tserase::TSERASE_SPEC>,
    #[doc = "0x10 - desc TMERASE"]
    pub tmerase: crate::Reg<tmerase::TMERASE_SPEC>,
    #[doc = "0x14 - desc TPRCV"]
    pub tprcv: crate::Reg<tprcv::TPRCV_SPEC>,
    #[doc = "0x18 - desc TSRCV"]
    pub tsrcv: crate::Reg<tsrcv::TSRCV_SPEC>,
    #[doc = "0x1c - desc TMRCV"]
    pub tmrcv: crate::Reg<tmrcv::TMRCV_SPEC>,
    #[doc = "0x20 - desc CR"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x24 - desc IFR"]
    pub ifr: crate::Reg<ifr::IFR_SPEC>,
    #[doc = "0x28 - desc ICLR"]
    pub iclr: crate::Reg<iclr::ICLR_SPEC>,
    #[doc = "0x2c - desc BYPASS"]
    pub bypass: crate::Reg<bypass::BYPASS_SPEC>,
    #[doc = "0x30 - desc SLOCK"]
    pub slock: crate::Reg<slock::SLOCK_SPEC>,
}
#[doc = "TNVS register accessor: an alias for `Reg<TNVS_SPEC>`"]
pub type TNVS = crate::Reg<tnvs::TNVS_SPEC>;
#[doc = "desc TNVS"]
pub mod tnvs;
#[doc = "TPGS register accessor: an alias for `Reg<TPGS_SPEC>`"]
pub type TPGS = crate::Reg<tpgs::TPGS_SPEC>;
#[doc = "desc TPGS"]
pub mod tpgs;
#[doc = "TPROG register accessor: an alias for `Reg<TPROG_SPEC>`"]
pub type TPROG = crate::Reg<tprog::TPROG_SPEC>;
#[doc = "desc TPROG"]
pub mod tprog;
#[doc = "TSERASE register accessor: an alias for `Reg<TSERASE_SPEC>`"]
pub type TSERASE = crate::Reg<tserase::TSERASE_SPEC>;
#[doc = "desc TSERASE"]
pub mod tserase;
#[doc = "TMERASE register accessor: an alias for `Reg<TMERASE_SPEC>`"]
pub type TMERASE = crate::Reg<tmerase::TMERASE_SPEC>;
#[doc = "desc TMERASE"]
pub mod tmerase;
#[doc = "TPRCV register accessor: an alias for `Reg<TPRCV_SPEC>`"]
pub type TPRCV = crate::Reg<tprcv::TPRCV_SPEC>;
#[doc = "desc TPRCV"]
pub mod tprcv;
#[doc = "TSRCV register accessor: an alias for `Reg<TSRCV_SPEC>`"]
pub type TSRCV = crate::Reg<tsrcv::TSRCV_SPEC>;
#[doc = "desc TSRCV"]
pub mod tsrcv;
#[doc = "TMRCV register accessor: an alias for `Reg<TMRCV_SPEC>`"]
pub type TMRCV = crate::Reg<tmrcv::TMRCV_SPEC>;
#[doc = "desc TMRCV"]
pub mod tmrcv;
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
#[doc = "BYPASS register accessor: an alias for `Reg<BYPASS_SPEC>`"]
pub type BYPASS = crate::Reg<bypass::BYPASS_SPEC>;
#[doc = "desc BYPASS"]
pub mod bypass;
#[doc = "SLOCK register accessor: an alias for `Reg<SLOCK_SPEC>`"]
pub type SLOCK = crate::Reg<slock::SLOCK_SPEC>;
#[doc = "desc SLOCK"]
pub mod slock;
