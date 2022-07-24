#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - desc CR"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x04 - desc MODE"]
    pub mode: crate::Reg<mode::MODE_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - desc DATA0"]
    pub data0: crate::Reg<data0::DATA0_SPEC>,
    #[doc = "0x10 - desc DATA1"]
    pub data1: crate::Reg<data1::DATA1_SPEC>,
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "desc CR"]
pub mod cr;
#[doc = "MODE register accessor: an alias for `Reg<MODE_SPEC>`"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "desc MODE"]
pub mod mode;
#[doc = "DATA0 register accessor: an alias for `Reg<DATA0_SPEC>`"]
pub type DATA0 = crate::Reg<data0::DATA0_SPEC>;
#[doc = "desc DATA0"]
pub mod data0;
#[doc = "DATA1 register accessor: an alias for `Reg<DATA1_SPEC>`"]
pub type DATA1 = crate::Reg<data1::DATA1_SPEC>;
#[doc = "desc DATA1"]
pub mod data1;
