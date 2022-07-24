#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x30],
    #[doc = "0x30 - desc CR0"]
    pub cr0: crate::Reg<cr0::CR0_SPEC>,
    #[doc = "0x34 - desc CR1"]
    pub cr1: crate::Reg<cr1::CR1_SPEC>,
    #[doc = "0x38 - desc CR2"]
    pub cr2: crate::Reg<cr2::CR2_SPEC>,
    #[doc = "0x3c - desc CR"]
    pub cr: crate::Reg<cr::CR_SPEC>,
}
#[doc = "CR0 register accessor: an alias for `Reg<CR0_SPEC>`"]
pub type CR0 = crate::Reg<cr0::CR0_SPEC>;
#[doc = "desc CR0"]
pub mod cr0;
#[doc = "CR1 register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "desc CR1"]
pub mod cr1;
#[doc = "CR2 register accessor: an alias for `Reg<CR2_SPEC>`"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "desc CR2"]
pub mod cr2;
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "desc CR"]
pub mod cr;
