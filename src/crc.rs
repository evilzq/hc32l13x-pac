#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - desc CR"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x04 - desc RESULT"]
    pub result: crate::Reg<result::RESULT_SPEC>,
    _reserved2: [u8; 0x78],
    #[doc = "0x80 - desc DATA"]
    pub data: crate::Reg<data::DATA_SPEC>,
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "desc CR"]
pub mod cr;
#[doc = "RESULT register accessor: an alias for `Reg<RESULT_SPEC>`"]
pub type RESULT = crate::Reg<result::RESULT_SPEC>;
#[doc = "desc RESULT"]
pub mod result;
#[doc = "DATA register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "desc DATA"]
pub mod data;
