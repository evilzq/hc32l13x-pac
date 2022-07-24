#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - desc CCON"]
    pub ccon: crate::Reg<ccon::CCON_SPEC>,
    #[doc = "0x04 - desc CMOD"]
    pub cmod: crate::Reg<cmod::CMOD_SPEC>,
    #[doc = "0x08 - desc CNT"]
    pub cnt: crate::Reg<cnt::CNT_SPEC>,
    #[doc = "0x0c - desc ICLR"]
    pub iclr: crate::Reg<iclr::ICLR_SPEC>,
    #[doc = "0x10 - desc CCAPM0"]
    pub ccapm0: crate::Reg<ccapm0::CCAPM0_SPEC>,
    #[doc = "0x14 - desc CCAPM1"]
    pub ccapm1: crate::Reg<ccapm1::CCAPM1_SPEC>,
    #[doc = "0x18 - desc CCAPM2"]
    pub ccapm2: crate::Reg<ccapm2::CCAPM2_SPEC>,
    #[doc = "0x1c - desc CCAPM3"]
    pub ccapm3: crate::Reg<ccapm3::CCAPM3_SPEC>,
    #[doc = "0x20 - desc CCAPM4"]
    pub ccapm4: crate::Reg<ccapm4::CCAPM4_SPEC>,
    #[doc = "0x24 - desc CCAP0H"]
    pub ccap0h: crate::Reg<ccap0h::CCAP0H_SPEC>,
    #[doc = "0x28 - desc CCAP0L"]
    pub ccap0l: crate::Reg<ccap0l::CCAP0L_SPEC>,
    #[doc = "0x2c - desc CCAP1H"]
    pub ccap1h: crate::Reg<ccap1h::CCAP1H_SPEC>,
    #[doc = "0x30 - desc CCAP1L"]
    pub ccap1l: crate::Reg<ccap1l::CCAP1L_SPEC>,
    #[doc = "0x34 - desc CCAP2H"]
    pub ccap2h: crate::Reg<ccap2h::CCAP2H_SPEC>,
    #[doc = "0x38 - desc CCAP2L"]
    pub ccap2l: crate::Reg<ccap2l::CCAP2L_SPEC>,
    #[doc = "0x3c - desc CCAP3H"]
    pub ccap3h: crate::Reg<ccap3h::CCAP3H_SPEC>,
    #[doc = "0x40 - desc CCAP3L"]
    pub ccap3l: crate::Reg<ccap3l::CCAP3L_SPEC>,
    #[doc = "0x44 - desc CCAP4H"]
    pub ccap4h: crate::Reg<ccap4h::CCAP4H_SPEC>,
    #[doc = "0x48 - desc CCAP4L"]
    pub ccap4l: crate::Reg<ccap4l::CCAP4L_SPEC>,
    #[doc = "0x4c - desc CCAPO"]
    pub ccapo: crate::Reg<ccapo::CCAPO_SPEC>,
    #[doc = "0x50 - desc CCAP0"]
    pub ccap0: crate::Reg<ccap0::CCAP0_SPEC>,
    #[doc = "0x54 - desc CCAP1"]
    pub ccap1: crate::Reg<ccap1::CCAP1_SPEC>,
    #[doc = "0x58 - desc CCAP2"]
    pub ccap2: crate::Reg<ccap2::CCAP2_SPEC>,
    #[doc = "0x5c - desc CCAP3"]
    pub ccap3: crate::Reg<ccap3::CCAP3_SPEC>,
    #[doc = "0x60 - desc CCAP4"]
    pub ccap4: crate::Reg<ccap4::CCAP4_SPEC>,
    #[doc = "0x64 - desc CARR"]
    pub carr: crate::Reg<carr::CARR_SPEC>,
    #[doc = "0x68 - desc EPWM"]
    pub epwm: crate::Reg<epwm::EPWM_SPEC>,
}
#[doc = "CCON register accessor: an alias for `Reg<CCON_SPEC>`"]
pub type CCON = crate::Reg<ccon::CCON_SPEC>;
#[doc = "desc CCON"]
pub mod ccon;
#[doc = "CMOD register accessor: an alias for `Reg<CMOD_SPEC>`"]
pub type CMOD = crate::Reg<cmod::CMOD_SPEC>;
#[doc = "desc CMOD"]
pub mod cmod;
#[doc = "CNT register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "desc CNT"]
pub mod cnt;
#[doc = "ICLR register accessor: an alias for `Reg<ICLR_SPEC>`"]
pub type ICLR = crate::Reg<iclr::ICLR_SPEC>;
#[doc = "desc ICLR"]
pub mod iclr;
#[doc = "CCAPM0 register accessor: an alias for `Reg<CCAPM0_SPEC>`"]
pub type CCAPM0 = crate::Reg<ccapm0::CCAPM0_SPEC>;
#[doc = "desc CCAPM0"]
pub mod ccapm0;
#[doc = "CCAPM1 register accessor: an alias for `Reg<CCAPM1_SPEC>`"]
pub type CCAPM1 = crate::Reg<ccapm1::CCAPM1_SPEC>;
#[doc = "desc CCAPM1"]
pub mod ccapm1;
#[doc = "CCAPM2 register accessor: an alias for `Reg<CCAPM2_SPEC>`"]
pub type CCAPM2 = crate::Reg<ccapm2::CCAPM2_SPEC>;
#[doc = "desc CCAPM2"]
pub mod ccapm2;
#[doc = "CCAPM3 register accessor: an alias for `Reg<CCAPM3_SPEC>`"]
pub type CCAPM3 = crate::Reg<ccapm3::CCAPM3_SPEC>;
#[doc = "desc CCAPM3"]
pub mod ccapm3;
#[doc = "CCAPM4 register accessor: an alias for `Reg<CCAPM4_SPEC>`"]
pub type CCAPM4 = crate::Reg<ccapm4::CCAPM4_SPEC>;
#[doc = "desc CCAPM4"]
pub mod ccapm4;
#[doc = "CCAP0H register accessor: an alias for `Reg<CCAP0H_SPEC>`"]
pub type CCAP0H = crate::Reg<ccap0h::CCAP0H_SPEC>;
#[doc = "desc CCAP0H"]
pub mod ccap0h;
#[doc = "CCAP0L register accessor: an alias for `Reg<CCAP0L_SPEC>`"]
pub type CCAP0L = crate::Reg<ccap0l::CCAP0L_SPEC>;
#[doc = "desc CCAP0L"]
pub mod ccap0l;
#[doc = "CCAP1H register accessor: an alias for `Reg<CCAP1H_SPEC>`"]
pub type CCAP1H = crate::Reg<ccap1h::CCAP1H_SPEC>;
#[doc = "desc CCAP1H"]
pub mod ccap1h;
#[doc = "CCAP1L register accessor: an alias for `Reg<CCAP1L_SPEC>`"]
pub type CCAP1L = crate::Reg<ccap1l::CCAP1L_SPEC>;
#[doc = "desc CCAP1L"]
pub mod ccap1l;
#[doc = "CCAP2H register accessor: an alias for `Reg<CCAP2H_SPEC>`"]
pub type CCAP2H = crate::Reg<ccap2h::CCAP2H_SPEC>;
#[doc = "desc CCAP2H"]
pub mod ccap2h;
#[doc = "CCAP2L register accessor: an alias for `Reg<CCAP2L_SPEC>`"]
pub type CCAP2L = crate::Reg<ccap2l::CCAP2L_SPEC>;
#[doc = "desc CCAP2L"]
pub mod ccap2l;
#[doc = "CCAP3H register accessor: an alias for `Reg<CCAP3H_SPEC>`"]
pub type CCAP3H = crate::Reg<ccap3h::CCAP3H_SPEC>;
#[doc = "desc CCAP3H"]
pub mod ccap3h;
#[doc = "CCAP3L register accessor: an alias for `Reg<CCAP3L_SPEC>`"]
pub type CCAP3L = crate::Reg<ccap3l::CCAP3L_SPEC>;
#[doc = "desc CCAP3L"]
pub mod ccap3l;
#[doc = "CCAP4H register accessor: an alias for `Reg<CCAP4H_SPEC>`"]
pub type CCAP4H = crate::Reg<ccap4h::CCAP4H_SPEC>;
#[doc = "desc CCAP4H"]
pub mod ccap4h;
#[doc = "CCAP4L register accessor: an alias for `Reg<CCAP4L_SPEC>`"]
pub type CCAP4L = crate::Reg<ccap4l::CCAP4L_SPEC>;
#[doc = "desc CCAP4L"]
pub mod ccap4l;
#[doc = "CCAPO register accessor: an alias for `Reg<CCAPO_SPEC>`"]
pub type CCAPO = crate::Reg<ccapo::CCAPO_SPEC>;
#[doc = "desc CCAPO"]
pub mod ccapo;
#[doc = "CCAP0 register accessor: an alias for `Reg<CCAP0_SPEC>`"]
pub type CCAP0 = crate::Reg<ccap0::CCAP0_SPEC>;
#[doc = "desc CCAP0"]
pub mod ccap0;
#[doc = "CCAP1 register accessor: an alias for `Reg<CCAP1_SPEC>`"]
pub type CCAP1 = crate::Reg<ccap1::CCAP1_SPEC>;
#[doc = "desc CCAP1"]
pub mod ccap1;
#[doc = "CCAP2 register accessor: an alias for `Reg<CCAP2_SPEC>`"]
pub type CCAP2 = crate::Reg<ccap2::CCAP2_SPEC>;
#[doc = "desc CCAP2"]
pub mod ccap2;
#[doc = "CCAP3 register accessor: an alias for `Reg<CCAP3_SPEC>`"]
pub type CCAP3 = crate::Reg<ccap3::CCAP3_SPEC>;
#[doc = "desc CCAP3"]
pub mod ccap3;
#[doc = "CCAP4 register accessor: an alias for `Reg<CCAP4_SPEC>`"]
pub type CCAP4 = crate::Reg<ccap4::CCAP4_SPEC>;
#[doc = "desc CCAP4"]
pub mod ccap4;
#[doc = "CARR register accessor: an alias for `Reg<CARR_SPEC>`"]
pub type CARR = crate::Reg<carr::CARR_SPEC>;
#[doc = "desc CARR"]
pub mod carr;
#[doc = "EPWM register accessor: an alias for `Reg<EPWM_SPEC>`"]
pub type EPWM = crate::Reg<epwm::EPWM_SPEC>;
#[doc = "desc EPWM"]
pub mod epwm;
