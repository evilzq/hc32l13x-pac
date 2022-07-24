#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - desc SYSCTRL0"]
    pub sysctrl0: crate::Reg<sysctrl0::SYSCTRL0_SPEC>,
    #[doc = "0x04 - desc SYSCTRL1"]
    pub sysctrl1: crate::Reg<sysctrl1::SYSCTRL1_SPEC>,
    #[doc = "0x08 - desc SYSCTRL2"]
    pub sysctrl2: crate::Reg<sysctrl2::SYSCTRL2_SPEC>,
    #[doc = "0x0c - desc RCH_CR"]
    pub rch_cr: crate::Reg<rch_cr::RCH_CR_SPEC>,
    #[doc = "0x10 - desc XTH_CR"]
    pub xth_cr: crate::Reg<xth_cr::XTH_CR_SPEC>,
    #[doc = "0x14 - desc RCL_CR"]
    pub rcl_cr: crate::Reg<rcl_cr::RCL_CR_SPEC>,
    #[doc = "0x18 - desc XTL_CR"]
    pub xtl_cr: crate::Reg<xtl_cr::XTL_CR_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - desc PERI_CLKEN"]
    pub peri_clken: crate::Reg<peri_clken::PERI_CLKEN_SPEC>,
    _reserved8: [u8; 0x18],
    #[doc = "0x3c - desc PLL_CR"]
    pub pll_cr: crate::Reg<pll_cr::PLL_CR_SPEC>,
}
#[doc = "SYSCTRL0 register accessor: an alias for `Reg<SYSCTRL0_SPEC>`"]
pub type SYSCTRL0 = crate::Reg<sysctrl0::SYSCTRL0_SPEC>;
#[doc = "desc SYSCTRL0"]
pub mod sysctrl0;
#[doc = "SYSCTRL1 register accessor: an alias for `Reg<SYSCTRL1_SPEC>`"]
pub type SYSCTRL1 = crate::Reg<sysctrl1::SYSCTRL1_SPEC>;
#[doc = "desc SYSCTRL1"]
pub mod sysctrl1;
#[doc = "SYSCTRL2 register accessor: an alias for `Reg<SYSCTRL2_SPEC>`"]
pub type SYSCTRL2 = crate::Reg<sysctrl2::SYSCTRL2_SPEC>;
#[doc = "desc SYSCTRL2"]
pub mod sysctrl2;
#[doc = "RCH_CR register accessor: an alias for `Reg<RCH_CR_SPEC>`"]
pub type RCH_CR = crate::Reg<rch_cr::RCH_CR_SPEC>;
#[doc = "desc RCH_CR"]
pub mod rch_cr;
#[doc = "XTH_CR register accessor: an alias for `Reg<XTH_CR_SPEC>`"]
pub type XTH_CR = crate::Reg<xth_cr::XTH_CR_SPEC>;
#[doc = "desc XTH_CR"]
pub mod xth_cr;
#[doc = "RCL_CR register accessor: an alias for `Reg<RCL_CR_SPEC>`"]
pub type RCL_CR = crate::Reg<rcl_cr::RCL_CR_SPEC>;
#[doc = "desc RCL_CR"]
pub mod rcl_cr;
#[doc = "XTL_CR register accessor: an alias for `Reg<XTL_CR_SPEC>`"]
pub type XTL_CR = crate::Reg<xtl_cr::XTL_CR_SPEC>;
#[doc = "desc XTL_CR"]
pub mod xtl_cr;
#[doc = "PERI_CLKEN register accessor: an alias for `Reg<PERI_CLKEN_SPEC>`"]
pub type PERI_CLKEN = crate::Reg<peri_clken::PERI_CLKEN_SPEC>;
#[doc = "desc PERI_CLKEN"]
pub mod peri_clken;
#[doc = "PLL_CR register accessor: an alias for `Reg<PLL_CR_SPEC>`"]
pub type PLL_CR = crate::Reg<pll_cr::PLL_CR_SPEC>;
#[doc = "desc PLL_CR"]
pub mod pll_cr;
