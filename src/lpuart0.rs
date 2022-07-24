#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - desc SBUF"]
    pub sbuf: crate::Reg<sbuf::SBUF_SPEC>,
    #[doc = "0x04 - desc SCON"]
    pub scon: crate::Reg<scon::SCON_SPEC>,
    #[doc = "0x08 - desc SADDR"]
    pub saddr: crate::Reg<saddr::SADDR_SPEC>,
    #[doc = "0x0c - desc SADEN"]
    pub saden: crate::Reg<saden::SADEN_SPEC>,
    #[doc = "0x10 - desc ISR"]
    pub isr: crate::Reg<isr::ISR_SPEC>,
    #[doc = "0x14 - desc ICR"]
    pub icr: crate::Reg<icr::ICR_SPEC>,
    #[doc = "0x18 - desc SCNT"]
    pub scnt: crate::Reg<scnt::SCNT_SPEC>,
}
#[doc = "SBUF register accessor: an alias for `Reg<SBUF_SPEC>`"]
pub type SBUF = crate::Reg<sbuf::SBUF_SPEC>;
#[doc = "desc SBUF"]
pub mod sbuf;
#[doc = "SCON register accessor: an alias for `Reg<SCON_SPEC>`"]
pub type SCON = crate::Reg<scon::SCON_SPEC>;
#[doc = "desc SCON"]
pub mod scon;
#[doc = "SADDR register accessor: an alias for `Reg<SADDR_SPEC>`"]
pub type SADDR = crate::Reg<saddr::SADDR_SPEC>;
#[doc = "desc SADDR"]
pub mod saddr;
#[doc = "SADEN register accessor: an alias for `Reg<SADEN_SPEC>`"]
pub type SADEN = crate::Reg<saden::SADEN_SPEC>;
#[doc = "desc SADEN"]
pub mod saden;
#[doc = "ISR register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "desc ISR"]
pub mod isr;
#[doc = "ICR register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "desc ICR"]
pub mod icr;
#[doc = "SCNT register accessor: an alias for `Reg<SCNT_SPEC>`"]
pub type SCNT = crate::Reg<scnt::SCNT_SPEC>;
#[doc = "desc SCNT"]
pub mod scnt;
