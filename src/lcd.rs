#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - desc CR0"]
    pub cr0: crate::Reg<cr0::CR0_SPEC>,
    #[doc = "0x04 - desc CR1"]
    pub cr1: crate::Reg<cr1::CR1_SPEC>,
    #[doc = "0x08 - desc INTCLR"]
    pub intclr: crate::Reg<intclr::INTCLR_SPEC>,
    #[doc = "0x0c - desc POEN0"]
    pub poen0: crate::Reg<poen0::POEN0_SPEC>,
    #[doc = "0x10 - desc POEN1"]
    pub poen1: crate::Reg<poen1::POEN1_SPEC>,
    _reserved5: [u8; 0x2c],
    #[doc = "0x40 - desc RAM0"]
    pub ram0: crate::Reg<ram0::RAM0_SPEC>,
    #[doc = "0x44 - desc RAM1"]
    pub ram1: crate::Reg<ram1::RAM1_SPEC>,
    #[doc = "0x48 - desc RAM2"]
    pub ram2: crate::Reg<ram2::RAM2_SPEC>,
    #[doc = "0x4c - desc RAM3"]
    pub ram3: crate::Reg<ram3::RAM3_SPEC>,
    #[doc = "0x50 - desc RAM4"]
    pub ram4: crate::Reg<ram4::RAM4_SPEC>,
    #[doc = "0x54 - desc RAM5"]
    pub ram5: crate::Reg<ram5::RAM5_SPEC>,
    #[doc = "0x58 - desc RAM6"]
    pub ram6: crate::Reg<ram6::RAM6_SPEC>,
    #[doc = "0x5c - desc RAM7"]
    pub ram7: crate::Reg<ram7::RAM7_SPEC>,
    #[doc = "0x60 - desc RAM8"]
    pub ram8: crate::Reg<ram8::RAM8_SPEC>,
    #[doc = "0x64 - desc RAM9"]
    pub ram9: crate::Reg<ram9::RAM9_SPEC>,
    #[doc = "0x68 - desc RAMA"]
    pub rama: crate::Reg<rama::RAMA_SPEC>,
    #[doc = "0x6c - desc RAMB"]
    pub ramb: crate::Reg<ramb::RAMB_SPEC>,
    #[doc = "0x70 - desc RAMC"]
    pub ramc: crate::Reg<ramc::RAMC_SPEC>,
    #[doc = "0x74 - desc RAMD"]
    pub ramd: crate::Reg<ramd::RAMD_SPEC>,
    #[doc = "0x78 - desc RAME"]
    pub rame: crate::Reg<rame::RAME_SPEC>,
    #[doc = "0x7c - desc RAMF"]
    pub ramf: crate::Reg<ramf::RAMF_SPEC>,
}
#[doc = "CR0 register accessor: an alias for `Reg<CR0_SPEC>`"]
pub type CR0 = crate::Reg<cr0::CR0_SPEC>;
#[doc = "desc CR0"]
pub mod cr0;
#[doc = "CR1 register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "desc CR1"]
pub mod cr1;
#[doc = "INTCLR register accessor: an alias for `Reg<INTCLR_SPEC>`"]
pub type INTCLR = crate::Reg<intclr::INTCLR_SPEC>;
#[doc = "desc INTCLR"]
pub mod intclr;
#[doc = "POEN0 register accessor: an alias for `Reg<POEN0_SPEC>`"]
pub type POEN0 = crate::Reg<poen0::POEN0_SPEC>;
#[doc = "desc POEN0"]
pub mod poen0;
#[doc = "POEN1 register accessor: an alias for `Reg<POEN1_SPEC>`"]
pub type POEN1 = crate::Reg<poen1::POEN1_SPEC>;
#[doc = "desc POEN1"]
pub mod poen1;
#[doc = "RAM0 register accessor: an alias for `Reg<RAM0_SPEC>`"]
pub type RAM0 = crate::Reg<ram0::RAM0_SPEC>;
#[doc = "desc RAM0"]
pub mod ram0;
#[doc = "RAM1 register accessor: an alias for `Reg<RAM1_SPEC>`"]
pub type RAM1 = crate::Reg<ram1::RAM1_SPEC>;
#[doc = "desc RAM1"]
pub mod ram1;
#[doc = "RAM2 register accessor: an alias for `Reg<RAM2_SPEC>`"]
pub type RAM2 = crate::Reg<ram2::RAM2_SPEC>;
#[doc = "desc RAM2"]
pub mod ram2;
#[doc = "RAM3 register accessor: an alias for `Reg<RAM3_SPEC>`"]
pub type RAM3 = crate::Reg<ram3::RAM3_SPEC>;
#[doc = "desc RAM3"]
pub mod ram3;
#[doc = "RAM4 register accessor: an alias for `Reg<RAM4_SPEC>`"]
pub type RAM4 = crate::Reg<ram4::RAM4_SPEC>;
#[doc = "desc RAM4"]
pub mod ram4;
#[doc = "RAM5 register accessor: an alias for `Reg<RAM5_SPEC>`"]
pub type RAM5 = crate::Reg<ram5::RAM5_SPEC>;
#[doc = "desc RAM5"]
pub mod ram5;
#[doc = "RAM6 register accessor: an alias for `Reg<RAM6_SPEC>`"]
pub type RAM6 = crate::Reg<ram6::RAM6_SPEC>;
#[doc = "desc RAM6"]
pub mod ram6;
#[doc = "RAM7 register accessor: an alias for `Reg<RAM7_SPEC>`"]
pub type RAM7 = crate::Reg<ram7::RAM7_SPEC>;
#[doc = "desc RAM7"]
pub mod ram7;
#[doc = "RAM8 register accessor: an alias for `Reg<RAM8_SPEC>`"]
pub type RAM8 = crate::Reg<ram8::RAM8_SPEC>;
#[doc = "desc RAM8"]
pub mod ram8;
#[doc = "RAM9 register accessor: an alias for `Reg<RAM9_SPEC>`"]
pub type RAM9 = crate::Reg<ram9::RAM9_SPEC>;
#[doc = "desc RAM9"]
pub mod ram9;
#[doc = "RAMA register accessor: an alias for `Reg<RAMA_SPEC>`"]
pub type RAMA = crate::Reg<rama::RAMA_SPEC>;
#[doc = "desc RAMA"]
pub mod rama;
#[doc = "RAMB register accessor: an alias for `Reg<RAMB_SPEC>`"]
pub type RAMB = crate::Reg<ramb::RAMB_SPEC>;
#[doc = "desc RAMB"]
pub mod ramb;
#[doc = "RAMC register accessor: an alias for `Reg<RAMC_SPEC>`"]
pub type RAMC = crate::Reg<ramc::RAMC_SPEC>;
#[doc = "desc RAMC"]
pub mod ramc;
#[doc = "RAMD register accessor: an alias for `Reg<RAMD_SPEC>`"]
pub type RAMD = crate::Reg<ramd::RAMD_SPEC>;
#[doc = "desc RAMD"]
pub mod ramd;
#[doc = "RAME register accessor: an alias for `Reg<RAME_SPEC>`"]
pub type RAME = crate::Reg<rame::RAME_SPEC>;
#[doc = "desc RAME"]
pub mod rame;
#[doc = "RAMF register accessor: an alias for `Reg<RAMF_SPEC>`"]
pub type RAMF = crate::Reg<ramf::RAMF_SPEC>;
#[doc = "desc RAMF"]
pub mod ramf;
