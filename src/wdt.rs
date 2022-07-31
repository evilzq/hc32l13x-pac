#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x80],
    #[doc = "0x80 - desc RST"]
    pub rst: crate::Reg<rst::RST_SPEC>,
    #[doc = "0x84 - desc CON"]
    pub con: crate::Reg<con_s::CON_SPEC>,
}
#[doc = "RST register accessor: an alias for `Reg<RST_SPEC>`"]
pub type RST = crate::Reg<rst::RST_SPEC>;
#[doc = "desc RST"]
pub mod rst;
#[doc = "CON register accessor: an alias for `Reg<CON_SPEC>`"]
pub type CON = crate::Reg<con_s::CON_SPEC>;
#[doc = "desc CON"]
pub mod con_s;
