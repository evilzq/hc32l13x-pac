#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - desc CNTER"]
    pub cnter: crate::Reg<cnter::CNTER_SPEC>,
    #[doc = "0x04 - desc PERAR"]
    pub perar: crate::Reg<perar::PERAR_SPEC>,
    #[doc = "0x08 - desc PERBR"]
    pub perbr: crate::Reg<perbr::PERBR_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - desc GCMAR"]
    pub gcmar: crate::Reg<gcmar::GCMAR_SPEC>,
    #[doc = "0x14 - desc GCMBR"]
    pub gcmbr: crate::Reg<gcmbr::GCMBR_SPEC>,
    #[doc = "0x18 - desc GCMCR"]
    pub gcmcr: crate::Reg<gcmcr::GCMCR_SPEC>,
    #[doc = "0x1c - desc GCMDR"]
    pub gcmdr: crate::Reg<gcmdr::GCMDR_SPEC>,
    _reserved7: [u8; 0x08],
    #[doc = "0x28 - desc SCMAR"]
    pub scmar: crate::Reg<scmar::SCMAR_SPEC>,
    #[doc = "0x2c - desc SCMBR"]
    pub scmbr: crate::Reg<scmbr::SCMBR_SPEC>,
    _reserved9: [u8; 0x10],
    #[doc = "0x40 - desc DTUAR"]
    pub dtuar: crate::Reg<dtuar::DTUAR_SPEC>,
    #[doc = "0x44 - desc DTDAR"]
    pub dtdar: crate::Reg<dtdar::DTDAR_SPEC>,
    _reserved11: [u8; 0x08],
    #[doc = "0x50 - desc GCONR"]
    pub gconr: crate::Reg<gconr::GCONR_SPEC>,
    #[doc = "0x54 - desc ICONR"]
    pub iconr: crate::Reg<iconr::ICONR_SPEC>,
    #[doc = "0x58 - desc PCONR"]
    pub pconr: crate::Reg<pconr::PCONR_SPEC>,
    #[doc = "0x5c - desc BCONR"]
    pub bconr: crate::Reg<bconr::BCONR_SPEC>,
    #[doc = "0x60 - desc DCONR"]
    pub dconr: crate::Reg<dconr::DCONR_SPEC>,
    _reserved16: [u8; 0x04],
    #[doc = "0x68 - desc FCONR"]
    pub fconr: crate::Reg<fconr::FCONR_SPEC>,
    #[doc = "0x6c - desc VPERR"]
    pub vperr: crate::Reg<vperr::VPERR_SPEC>,
    #[doc = "0x70 - desc STFLR"]
    pub stflr: crate::Reg<stflr::STFLR_SPEC>,
    #[doc = "0x74 - desc HSTAR"]
    pub hstar: crate::Reg<hstar::HSTAR_SPEC>,
    #[doc = "0x78 - desc HSTPR"]
    pub hstpr: crate::Reg<hstpr::HSTPR_SPEC>,
    #[doc = "0x7c - desc HCELR"]
    pub hcelr: crate::Reg<hcelr::HCELR_SPEC>,
    #[doc = "0x80 - desc HCPAR"]
    pub hcpar: crate::Reg<hcpar::HCPAR_SPEC>,
    #[doc = "0x84 - desc HCPBR"]
    pub hcpbr: crate::Reg<hcpbr::HCPBR_SPEC>,
    #[doc = "0x88 - desc HCUPR"]
    pub hcupr: crate::Reg<hcupr::HCUPR_SPEC>,
    #[doc = "0x8c - desc HCDOR"]
    pub hcdor: crate::Reg<hcdor::HCDOR_SPEC>,
    _reserved26: [u8; 0x70],
    #[doc = "0x100 - desc IFR"]
    pub ifr: crate::Reg<ifr::IFR_SPEC>,
    #[doc = "0x104 - desc ICLR"]
    pub iclr: crate::Reg<iclr::ICLR_SPEC>,
    #[doc = "0x108 - desc CR"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    _reserved29: [u8; 0x04],
    #[doc = "0x110 - desc AOSSR"]
    pub aossr: crate::Reg<aossr::AOSSR_SPEC>,
    #[doc = "0x114 - desc AOSCL"]
    pub aoscl: crate::Reg<aoscl::AOSCL_SPEC>,
    #[doc = "0x118 - desc PTBKS"]
    pub ptbks: crate::Reg<ptbks::PTBKS_SPEC>,
    #[doc = "0x11c - desc TTRIG"]
    pub ttrig: crate::Reg<ttrig::TTRIG_SPEC>,
    #[doc = "0x120 - desc ITRIG"]
    pub itrig: crate::Reg<itrig::ITRIG_SPEC>,
    #[doc = "0x124 - desc PTBKP"]
    pub ptbkp: crate::Reg<ptbkp::PTBKP_SPEC>,
    _reserved35: [u8; 0x02cc],
    #[doc = "0x3f4 - desc SSTAR"]
    pub sstar: crate::Reg<sstar::SSTAR_SPEC>,
    #[doc = "0x3f8 - desc SSTPR"]
    pub sstpr: crate::Reg<sstpr::SSTPR_SPEC>,
    #[doc = "0x3fc - desc SCLRR"]
    pub sclrr: crate::Reg<sclrr::SCLRR_SPEC>,
}
#[doc = "CNTER register accessor: an alias for `Reg<CNTER_SPEC>`"]
pub type CNTER = crate::Reg<cnter::CNTER_SPEC>;
#[doc = "desc CNTER"]
pub mod cnter;
#[doc = "PERAR register accessor: an alias for `Reg<PERAR_SPEC>`"]
pub type PERAR = crate::Reg<perar::PERAR_SPEC>;
#[doc = "desc PERAR"]
pub mod perar;
#[doc = "PERBR register accessor: an alias for `Reg<PERBR_SPEC>`"]
pub type PERBR = crate::Reg<perbr::PERBR_SPEC>;
#[doc = "desc PERBR"]
pub mod perbr;
#[doc = "GCMAR register accessor: an alias for `Reg<GCMAR_SPEC>`"]
pub type GCMAR = crate::Reg<gcmar::GCMAR_SPEC>;
#[doc = "desc GCMAR"]
pub mod gcmar;
#[doc = "GCMBR register accessor: an alias for `Reg<GCMBR_SPEC>`"]
pub type GCMBR = crate::Reg<gcmbr::GCMBR_SPEC>;
#[doc = "desc GCMBR"]
pub mod gcmbr;
#[doc = "GCMCR register accessor: an alias for `Reg<GCMCR_SPEC>`"]
pub type GCMCR = crate::Reg<gcmcr::GCMCR_SPEC>;
#[doc = "desc GCMCR"]
pub mod gcmcr;
#[doc = "GCMDR register accessor: an alias for `Reg<GCMDR_SPEC>`"]
pub type GCMDR = crate::Reg<gcmdr::GCMDR_SPEC>;
#[doc = "desc GCMDR"]
pub mod gcmdr;
#[doc = "SCMAR register accessor: an alias for `Reg<SCMAR_SPEC>`"]
pub type SCMAR = crate::Reg<scmar::SCMAR_SPEC>;
#[doc = "desc SCMAR"]
pub mod scmar;
#[doc = "SCMBR register accessor: an alias for `Reg<SCMBR_SPEC>`"]
pub type SCMBR = crate::Reg<scmbr::SCMBR_SPEC>;
#[doc = "desc SCMBR"]
pub mod scmbr;
#[doc = "DTUAR register accessor: an alias for `Reg<DTUAR_SPEC>`"]
pub type DTUAR = crate::Reg<dtuar::DTUAR_SPEC>;
#[doc = "desc DTUAR"]
pub mod dtuar;
#[doc = "DTDAR register accessor: an alias for `Reg<DTDAR_SPEC>`"]
pub type DTDAR = crate::Reg<dtdar::DTDAR_SPEC>;
#[doc = "desc DTDAR"]
pub mod dtdar;
#[doc = "GCONR register accessor: an alias for `Reg<GCONR_SPEC>`"]
pub type GCONR = crate::Reg<gconr::GCONR_SPEC>;
#[doc = "desc GCONR"]
pub mod gconr;
#[doc = "ICONR register accessor: an alias for `Reg<ICONR_SPEC>`"]
pub type ICONR = crate::Reg<iconr::ICONR_SPEC>;
#[doc = "desc ICONR"]
pub mod iconr;
#[doc = "PCONR register accessor: an alias for `Reg<PCONR_SPEC>`"]
pub type PCONR = crate::Reg<pconr::PCONR_SPEC>;
#[doc = "desc PCONR"]
pub mod pconr;
#[doc = "BCONR register accessor: an alias for `Reg<BCONR_SPEC>`"]
pub type BCONR = crate::Reg<bconr::BCONR_SPEC>;
#[doc = "desc BCONR"]
pub mod bconr;
#[doc = "DCONR register accessor: an alias for `Reg<DCONR_SPEC>`"]
pub type DCONR = crate::Reg<dconr::DCONR_SPEC>;
#[doc = "desc DCONR"]
pub mod dconr;
#[doc = "FCONR register accessor: an alias for `Reg<FCONR_SPEC>`"]
pub type FCONR = crate::Reg<fconr::FCONR_SPEC>;
#[doc = "desc FCONR"]
pub mod fconr;
#[doc = "VPERR register accessor: an alias for `Reg<VPERR_SPEC>`"]
pub type VPERR = crate::Reg<vperr::VPERR_SPEC>;
#[doc = "desc VPERR"]
pub mod vperr;
#[doc = "STFLR register accessor: an alias for `Reg<STFLR_SPEC>`"]
pub type STFLR = crate::Reg<stflr::STFLR_SPEC>;
#[doc = "desc STFLR"]
pub mod stflr;
#[doc = "HSTAR register accessor: an alias for `Reg<HSTAR_SPEC>`"]
pub type HSTAR = crate::Reg<hstar::HSTAR_SPEC>;
#[doc = "desc HSTAR"]
pub mod hstar;
#[doc = "HSTPR register accessor: an alias for `Reg<HSTPR_SPEC>`"]
pub type HSTPR = crate::Reg<hstpr::HSTPR_SPEC>;
#[doc = "desc HSTPR"]
pub mod hstpr;
#[doc = "HCELR register accessor: an alias for `Reg<HCELR_SPEC>`"]
pub type HCELR = crate::Reg<hcelr::HCELR_SPEC>;
#[doc = "desc HCELR"]
pub mod hcelr;
#[doc = "HCPAR register accessor: an alias for `Reg<HCPAR_SPEC>`"]
pub type HCPAR = crate::Reg<hcpar::HCPAR_SPEC>;
#[doc = "desc HCPAR"]
pub mod hcpar;
#[doc = "HCPBR register accessor: an alias for `Reg<HCPBR_SPEC>`"]
pub type HCPBR = crate::Reg<hcpbr::HCPBR_SPEC>;
#[doc = "desc HCPBR"]
pub mod hcpbr;
#[doc = "HCUPR register accessor: an alias for `Reg<HCUPR_SPEC>`"]
pub type HCUPR = crate::Reg<hcupr::HCUPR_SPEC>;
#[doc = "desc HCUPR"]
pub mod hcupr;
#[doc = "HCDOR register accessor: an alias for `Reg<HCDOR_SPEC>`"]
pub type HCDOR = crate::Reg<hcdor::HCDOR_SPEC>;
#[doc = "desc HCDOR"]
pub mod hcdor;
#[doc = "IFR register accessor: an alias for `Reg<IFR_SPEC>`"]
pub type IFR = crate::Reg<ifr::IFR_SPEC>;
#[doc = "desc IFR"]
pub mod ifr;
#[doc = "ICLR register accessor: an alias for `Reg<ICLR_SPEC>`"]
pub type ICLR = crate::Reg<iclr::ICLR_SPEC>;
#[doc = "desc ICLR"]
pub mod iclr;
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "desc CR"]
pub mod cr;
#[doc = "AOSSR register accessor: an alias for `Reg<AOSSR_SPEC>`"]
pub type AOSSR = crate::Reg<aossr::AOSSR_SPEC>;
#[doc = "desc AOSSR"]
pub mod aossr;
#[doc = "AOSCL register accessor: an alias for `Reg<AOSCL_SPEC>`"]
pub type AOSCL = crate::Reg<aoscl::AOSCL_SPEC>;
#[doc = "desc AOSCL"]
pub mod aoscl;
#[doc = "PTBKS register accessor: an alias for `Reg<PTBKS_SPEC>`"]
pub type PTBKS = crate::Reg<ptbks::PTBKS_SPEC>;
#[doc = "desc PTBKS"]
pub mod ptbks;
#[doc = "TTRIG register accessor: an alias for `Reg<TTRIG_SPEC>`"]
pub type TTRIG = crate::Reg<ttrig::TTRIG_SPEC>;
#[doc = "desc TTRIG"]
pub mod ttrig;
#[doc = "ITRIG register accessor: an alias for `Reg<ITRIG_SPEC>`"]
pub type ITRIG = crate::Reg<itrig::ITRIG_SPEC>;
#[doc = "desc ITRIG"]
pub mod itrig;
#[doc = "PTBKP register accessor: an alias for `Reg<PTBKP_SPEC>`"]
pub type PTBKP = crate::Reg<ptbkp::PTBKP_SPEC>;
#[doc = "desc PTBKP"]
pub mod ptbkp;
#[doc = "SSTAR register accessor: an alias for `Reg<SSTAR_SPEC>`"]
pub type SSTAR = crate::Reg<sstar::SSTAR_SPEC>;
#[doc = "desc SSTAR"]
pub mod sstar;
#[doc = "SSTPR register accessor: an alias for `Reg<SSTPR_SPEC>`"]
pub type SSTPR = crate::Reg<sstpr::SSTPR_SPEC>;
#[doc = "desc SSTPR"]
pub mod sstpr;
#[doc = "SCLRR register accessor: an alias for `Reg<SCLRR_SPEC>`"]
pub type SCLRR = crate::Reg<sclrr::SCLRR_SPEC>;
#[doc = "desc SCLRR"]
pub mod sclrr;
