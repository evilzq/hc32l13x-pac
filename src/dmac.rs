#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - desc CONF"]
    pub conf: crate::Reg<conf::CONF_SPEC>,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - desc CONFA0"]
    pub confa0: crate::Reg<confa0::CONFA0_SPEC>,
    #[doc = "0x14 - desc CONFB0"]
    pub confb0: crate::Reg<confb0::CONFB0_SPEC>,
    #[doc = "0x18 - desc SRCADR0"]
    pub srcadr0: crate::Reg<srcadr0::SRCADR0_SPEC>,
    #[doc = "0x1c - desc DSTADR0"]
    pub dstadr0: crate::Reg<dstadr0::DSTADR0_SPEC>,
    #[doc = "0x20 - desc CONFA1"]
    pub confa1: crate::Reg<confa1::CONFA1_SPEC>,
    #[doc = "0x24 - desc CONFB1"]
    pub confb1: crate::Reg<confb1::CONFB1_SPEC>,
    #[doc = "0x28 - desc SRCADR1"]
    pub srcadr1: crate::Reg<srcadr1::SRCADR1_SPEC>,
    #[doc = "0x2c - desc DSTADR1"]
    pub dstadr1: crate::Reg<dstadr1::DSTADR1_SPEC>,
}
#[doc = "CONF register accessor: an alias for `Reg<CONF_SPEC>`"]
pub type CONF = crate::Reg<conf::CONF_SPEC>;
#[doc = "desc CONF"]
pub mod conf;
#[doc = "CONFA0 register accessor: an alias for `Reg<CONFA0_SPEC>`"]
pub type CONFA0 = crate::Reg<confa0::CONFA0_SPEC>;
#[doc = "desc CONFA0"]
pub mod confa0;
#[doc = "CONFB0 register accessor: an alias for `Reg<CONFB0_SPEC>`"]
pub type CONFB0 = crate::Reg<confb0::CONFB0_SPEC>;
#[doc = "desc CONFB0"]
pub mod confb0;
#[doc = "SRCADR0 register accessor: an alias for `Reg<SRCADR0_SPEC>`"]
pub type SRCADR0 = crate::Reg<srcadr0::SRCADR0_SPEC>;
#[doc = "desc SRCADR0"]
pub mod srcadr0;
#[doc = "DSTADR0 register accessor: an alias for `Reg<DSTADR0_SPEC>`"]
pub type DSTADR0 = crate::Reg<dstadr0::DSTADR0_SPEC>;
#[doc = "desc DSTADR0"]
pub mod dstadr0;
#[doc = "CONFA1 register accessor: an alias for `Reg<CONFA1_SPEC>`"]
pub type CONFA1 = crate::Reg<confa1::CONFA1_SPEC>;
#[doc = "desc CONFA1"]
pub mod confa1;
#[doc = "CONFB1 register accessor: an alias for `Reg<CONFB1_SPEC>`"]
pub type CONFB1 = crate::Reg<confb1::CONFB1_SPEC>;
#[doc = "desc CONFB1"]
pub mod confb1;
#[doc = "SRCADR1 register accessor: an alias for `Reg<SRCADR1_SPEC>`"]
pub type SRCADR1 = crate::Reg<srcadr1::SRCADR1_SPEC>;
#[doc = "desc SRCADR1"]
pub mod srcadr1;
#[doc = "DSTADR1 register accessor: an alias for `Reg<DSTADR1_SPEC>`"]
pub type DSTADR1 = crate::Reg<dstadr1::DSTADR1_SPEC>;
#[doc = "desc DSTADR1"]
pub mod dstadr1;
