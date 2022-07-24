#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - desc RUN"]
    pub run: crate::Reg<run::RUN_SPEC>,
    #[doc = "0x04 - desc CTRL"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x08 - desc FLT"]
    pub flt: crate::Reg<flt::FLT_SPEC>,
    #[doc = "0x0c - desc TOCR"]
    pub tocr: crate::Reg<tocr::TOCR_SPEC>,
    #[doc = "0x10 - desc CMD"]
    pub cmd: crate::Reg<cmd::CMD_SPEC>,
    #[doc = "0x14 - desc SR1"]
    pub sr1: crate::Reg<sr1::SR1_SPEC>,
    #[doc = "0x18 - desc CNT"]
    pub cnt: crate::Reg<cnt::CNT_SPEC>,
    #[doc = "0x1c - desc TOP"]
    pub top: crate::Reg<top::TOP_SPEC>,
    #[doc = "0x20 - desc BUF"]
    pub buf: crate::Reg<buf::BUF_SPEC>,
    #[doc = "0x24 - desc IFR"]
    pub ifr: crate::Reg<ifr::IFR_SPEC>,
    #[doc = "0x28 - desc ICR"]
    pub icr: crate::Reg<icr::ICR_SPEC>,
    #[doc = "0x2c - desc IEN"]
    pub ien: crate::Reg<ien::IEN_SPEC>,
    #[doc = "0x30 - desc SR2"]
    pub sr2: crate::Reg<sr2::SR2_SPEC>,
}
#[doc = "RUN register accessor: an alias for `Reg<RUN_SPEC>`"]
pub type RUN = crate::Reg<run::RUN_SPEC>;
#[doc = "desc RUN"]
pub mod run;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "desc CTRL"]
pub mod ctrl;
#[doc = "FLT register accessor: an alias for `Reg<FLT_SPEC>`"]
pub type FLT = crate::Reg<flt::FLT_SPEC>;
#[doc = "desc FLT"]
pub mod flt;
#[doc = "TOCR register accessor: an alias for `Reg<TOCR_SPEC>`"]
pub type TOCR = crate::Reg<tocr::TOCR_SPEC>;
#[doc = "desc TOCR"]
pub mod tocr;
#[doc = "CMD register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "desc CMD"]
pub mod cmd;
#[doc = "SR1 register accessor: an alias for `Reg<SR1_SPEC>`"]
pub type SR1 = crate::Reg<sr1::SR1_SPEC>;
#[doc = "desc SR1"]
pub mod sr1;
#[doc = "CNT register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "desc CNT"]
pub mod cnt;
#[doc = "TOP register accessor: an alias for `Reg<TOP_SPEC>`"]
pub type TOP = crate::Reg<top::TOP_SPEC>;
#[doc = "desc TOP"]
pub mod top;
#[doc = "BUF register accessor: an alias for `Reg<BUF_SPEC>`"]
pub type BUF = crate::Reg<buf::BUF_SPEC>;
#[doc = "desc BUF"]
pub mod buf;
#[doc = "IFR register accessor: an alias for `Reg<IFR_SPEC>`"]
pub type IFR = crate::Reg<ifr::IFR_SPEC>;
#[doc = "desc IFR"]
pub mod ifr;
#[doc = "ICR register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "desc ICR"]
pub mod icr;
#[doc = "IEN register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "desc IEN"]
pub mod ien;
#[doc = "SR2 register accessor: an alias for `Reg<SR2_SPEC>`"]
pub type SR2 = crate::Reg<sr2::SR2_SPEC>;
#[doc = "desc SR2"]
pub mod sr2;
