#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - desc CR0"]
    pub cr0: crate::Reg<cr0::CR0_SPEC>,
    #[doc = "0x08 - desc CR1"]
    pub cr1: crate::Reg<cr1::CR1_SPEC>,
    _reserved2: [u8; 0x34],
    #[doc = "0x40 - desc SQR0"]
    pub sqr0: crate::Reg<sqr0::SQR0_SPEC>,
    #[doc = "0x44 - desc SQR1"]
    pub sqr1: crate::Reg<sqr1::SQR1_SPEC>,
    #[doc = "0x48 - desc SQR2"]
    pub sqr2: crate::Reg<sqr2::SQR2_SPEC>,
    #[doc = "0x4c - desc JQR"]
    pub jqr: crate::Reg<jqr::JQR_SPEC>,
    #[doc = "0x50 - desc SQRRESULT0"]
    pub sqrresult0: crate::Reg<sqrresult0::SQRRESULT0_SPEC>,
    #[doc = "0x54 - desc SQRRESULT1"]
    pub sqrresult1: crate::Reg<sqrresult1::SQRRESULT1_SPEC>,
    #[doc = "0x58 - desc SQRRESULT2"]
    pub sqrresult2: crate::Reg<sqrresult2::SQRRESULT2_SPEC>,
    #[doc = "0x5c - desc SQRRESULT3"]
    pub sqrresult3: crate::Reg<sqrresult3::SQRRESULT3_SPEC>,
    #[doc = "0x60 - desc SQRRESULT4"]
    pub sqrresult4: crate::Reg<sqrresult4::SQRRESULT4_SPEC>,
    #[doc = "0x64 - desc SQRRESULT5"]
    pub sqrresult5: crate::Reg<sqrresult5::SQRRESULT5_SPEC>,
    #[doc = "0x68 - desc SQRRESULT6"]
    pub sqrresult6: crate::Reg<sqrresult6::SQRRESULT6_SPEC>,
    #[doc = "0x6c - desc SQRRESULT7"]
    pub sqrresult7: crate::Reg<sqrresult7::SQRRESULT7_SPEC>,
    #[doc = "0x70 - desc SQRRESULT8"]
    pub sqrresult8: crate::Reg<sqrresult8::SQRRESULT8_SPEC>,
    #[doc = "0x74 - desc SQRRESULT9"]
    pub sqrresult9: crate::Reg<sqrresult9::SQRRESULT9_SPEC>,
    #[doc = "0x78 - desc SQRRESULT10"]
    pub sqrresult10: crate::Reg<sqrresult10::SQRRESULT10_SPEC>,
    #[doc = "0x7c - desc SQRRESULT11"]
    pub sqrresult11: crate::Reg<sqrresult11::SQRRESULT11_SPEC>,
    #[doc = "0x80 - desc SQRRESULT12"]
    pub sqrresult12: crate::Reg<sqrresult12::SQRRESULT12_SPEC>,
    #[doc = "0x84 - desc SQR_RESULT13"]
    pub sqr_result13: crate::Reg<sqr_result13::SQR_RESULT13_SPEC>,
    #[doc = "0x88 - desc SQRRESULT14"]
    pub sqrresult14: crate::Reg<sqrresult14::SQRRESULT14_SPEC>,
    #[doc = "0x8c - desc SQRRESULT15"]
    pub sqrresult15: crate::Reg<sqrresult15::SQRRESULT15_SPEC>,
    #[doc = "0x90 - desc JQRRESULT0"]
    pub jqrresult0: crate::Reg<jqrresult0::JQRRESULT0_SPEC>,
    #[doc = "0x94 - desc JQRRESULT1"]
    pub jqrresult1: crate::Reg<jqrresult1::JQRRESULT1_SPEC>,
    #[doc = "0x98 - desc JQRRESULT2"]
    pub jqrresult2: crate::Reg<jqrresult2::JQRRESULT2_SPEC>,
    #[doc = "0x9c - desc JQRRESULT3"]
    pub jqrresult3: crate::Reg<jqrresult3::JQRRESULT3_SPEC>,
    #[doc = "0xa0 - desc RESULT"]
    pub result: crate::Reg<result::RESULT_SPEC>,
    #[doc = "0xa4 - desc RESULTACC"]
    pub resultacc: crate::Reg<resultacc::RESULTACC_SPEC>,
    #[doc = "0xa8 - desc HT"]
    pub ht: crate::Reg<ht::HT_SPEC>,
    #[doc = "0xac - desc LT"]
    pub lt: crate::Reg<lt::LT_SPEC>,
    #[doc = "0xb0 - desc IFR"]
    pub ifr: crate::Reg<ifr::IFR_SPEC>,
    #[doc = "0xb4 - desc ICR"]
    pub icr: crate::Reg<icr::ICR_SPEC>,
    #[doc = "0xb8 - desc EXTTRIGGER0"]
    pub exttrigger0: crate::Reg<exttrigger0::EXTTRIGGER0_SPEC>,
    #[doc = "0xbc - desc EXTTRIGGER1"]
    pub exttrigger1: crate::Reg<exttrigger1::EXTTRIGGER1_SPEC>,
    #[doc = "0xc0 - desc SGLSTART"]
    pub sglstart: crate::Reg<sglstart::SGLSTART_SPEC>,
    #[doc = "0xc4 - desc SQRSTART"]
    pub sqrstart: crate::Reg<sqrstart::SQRSTART_SPEC>,
    #[doc = "0xc8 - desc JQRSTART"]
    pub jqrstart: crate::Reg<jqrstart::JQRSTART_SPEC>,
}
#[doc = "CR0 register accessor: an alias for `Reg<CR0_SPEC>`"]
pub type CR0 = crate::Reg<cr0::CR0_SPEC>;
#[doc = "desc CR0"]
pub mod cr0;
#[doc = "CR1 register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "desc CR1"]
pub mod cr1;
#[doc = "SQR0 register accessor: an alias for `Reg<SQR0_SPEC>`"]
pub type SQR0 = crate::Reg<sqr0::SQR0_SPEC>;
#[doc = "desc SQR0"]
pub mod sqr0;
#[doc = "SQR1 register accessor: an alias for `Reg<SQR1_SPEC>`"]
pub type SQR1 = crate::Reg<sqr1::SQR1_SPEC>;
#[doc = "desc SQR1"]
pub mod sqr1;
#[doc = "SQR2 register accessor: an alias for `Reg<SQR2_SPEC>`"]
pub type SQR2 = crate::Reg<sqr2::SQR2_SPEC>;
#[doc = "desc SQR2"]
pub mod sqr2;
#[doc = "JQR register accessor: an alias for `Reg<JQR_SPEC>`"]
pub type JQR = crate::Reg<jqr::JQR_SPEC>;
#[doc = "desc JQR"]
pub mod jqr;
#[doc = "SQRRESULT0 register accessor: an alias for `Reg<SQRRESULT0_SPEC>`"]
pub type SQRRESULT0 = crate::Reg<sqrresult0::SQRRESULT0_SPEC>;
#[doc = "desc SQRRESULT0"]
pub mod sqrresult0;
#[doc = "SQRRESULT1 register accessor: an alias for `Reg<SQRRESULT1_SPEC>`"]
pub type SQRRESULT1 = crate::Reg<sqrresult1::SQRRESULT1_SPEC>;
#[doc = "desc SQRRESULT1"]
pub mod sqrresult1;
#[doc = "SQRRESULT2 register accessor: an alias for `Reg<SQRRESULT2_SPEC>`"]
pub type SQRRESULT2 = crate::Reg<sqrresult2::SQRRESULT2_SPEC>;
#[doc = "desc SQRRESULT2"]
pub mod sqrresult2;
#[doc = "SQRRESULT3 register accessor: an alias for `Reg<SQRRESULT3_SPEC>`"]
pub type SQRRESULT3 = crate::Reg<sqrresult3::SQRRESULT3_SPEC>;
#[doc = "desc SQRRESULT3"]
pub mod sqrresult3;
#[doc = "SQRRESULT4 register accessor: an alias for `Reg<SQRRESULT4_SPEC>`"]
pub type SQRRESULT4 = crate::Reg<sqrresult4::SQRRESULT4_SPEC>;
#[doc = "desc SQRRESULT4"]
pub mod sqrresult4;
#[doc = "SQRRESULT5 register accessor: an alias for `Reg<SQRRESULT5_SPEC>`"]
pub type SQRRESULT5 = crate::Reg<sqrresult5::SQRRESULT5_SPEC>;
#[doc = "desc SQRRESULT5"]
pub mod sqrresult5;
#[doc = "SQRRESULT6 register accessor: an alias for `Reg<SQRRESULT6_SPEC>`"]
pub type SQRRESULT6 = crate::Reg<sqrresult6::SQRRESULT6_SPEC>;
#[doc = "desc SQRRESULT6"]
pub mod sqrresult6;
#[doc = "SQRRESULT7 register accessor: an alias for `Reg<SQRRESULT7_SPEC>`"]
pub type SQRRESULT7 = crate::Reg<sqrresult7::SQRRESULT7_SPEC>;
#[doc = "desc SQRRESULT7"]
pub mod sqrresult7;
#[doc = "SQRRESULT8 register accessor: an alias for `Reg<SQRRESULT8_SPEC>`"]
pub type SQRRESULT8 = crate::Reg<sqrresult8::SQRRESULT8_SPEC>;
#[doc = "desc SQRRESULT8"]
pub mod sqrresult8;
#[doc = "SQRRESULT9 register accessor: an alias for `Reg<SQRRESULT9_SPEC>`"]
pub type SQRRESULT9 = crate::Reg<sqrresult9::SQRRESULT9_SPEC>;
#[doc = "desc SQRRESULT9"]
pub mod sqrresult9;
#[doc = "SQRRESULT10 register accessor: an alias for `Reg<SQRRESULT10_SPEC>`"]
pub type SQRRESULT10 = crate::Reg<sqrresult10::SQRRESULT10_SPEC>;
#[doc = "desc SQRRESULT10"]
pub mod sqrresult10;
#[doc = "SQRRESULT11 register accessor: an alias for `Reg<SQRRESULT11_SPEC>`"]
pub type SQRRESULT11 = crate::Reg<sqrresult11::SQRRESULT11_SPEC>;
#[doc = "desc SQRRESULT11"]
pub mod sqrresult11;
#[doc = "SQRRESULT12 register accessor: an alias for `Reg<SQRRESULT12_SPEC>`"]
pub type SQRRESULT12 = crate::Reg<sqrresult12::SQRRESULT12_SPEC>;
#[doc = "desc SQRRESULT12"]
pub mod sqrresult12;
#[doc = "SQR_RESULT13 register accessor: an alias for `Reg<SQR_RESULT13_SPEC>`"]
pub type SQR_RESULT13 = crate::Reg<sqr_result13::SQR_RESULT13_SPEC>;
#[doc = "desc SQR_RESULT13"]
pub mod sqr_result13;
#[doc = "SQRRESULT14 register accessor: an alias for `Reg<SQRRESULT14_SPEC>`"]
pub type SQRRESULT14 = crate::Reg<sqrresult14::SQRRESULT14_SPEC>;
#[doc = "desc SQRRESULT14"]
pub mod sqrresult14;
#[doc = "SQRRESULT15 register accessor: an alias for `Reg<SQRRESULT15_SPEC>`"]
pub type SQRRESULT15 = crate::Reg<sqrresult15::SQRRESULT15_SPEC>;
#[doc = "desc SQRRESULT15"]
pub mod sqrresult15;
#[doc = "JQRRESULT0 register accessor: an alias for `Reg<JQRRESULT0_SPEC>`"]
pub type JQRRESULT0 = crate::Reg<jqrresult0::JQRRESULT0_SPEC>;
#[doc = "desc JQRRESULT0"]
pub mod jqrresult0;
#[doc = "JQRRESULT1 register accessor: an alias for `Reg<JQRRESULT1_SPEC>`"]
pub type JQRRESULT1 = crate::Reg<jqrresult1::JQRRESULT1_SPEC>;
#[doc = "desc JQRRESULT1"]
pub mod jqrresult1;
#[doc = "JQRRESULT2 register accessor: an alias for `Reg<JQRRESULT2_SPEC>`"]
pub type JQRRESULT2 = crate::Reg<jqrresult2::JQRRESULT2_SPEC>;
#[doc = "desc JQRRESULT2"]
pub mod jqrresult2;
#[doc = "JQRRESULT3 register accessor: an alias for `Reg<JQRRESULT3_SPEC>`"]
pub type JQRRESULT3 = crate::Reg<jqrresult3::JQRRESULT3_SPEC>;
#[doc = "desc JQRRESULT3"]
pub mod jqrresult3;
#[doc = "RESULT register accessor: an alias for `Reg<RESULT_SPEC>`"]
pub type RESULT = crate::Reg<result::RESULT_SPEC>;
#[doc = "desc RESULT"]
pub mod result;
#[doc = "RESULTACC register accessor: an alias for `Reg<RESULTACC_SPEC>`"]
pub type RESULTACC = crate::Reg<resultacc::RESULTACC_SPEC>;
#[doc = "desc RESULTACC"]
pub mod resultacc;
#[doc = "HT register accessor: an alias for `Reg<HT_SPEC>`"]
pub type HT = crate::Reg<ht::HT_SPEC>;
#[doc = "desc HT"]
pub mod ht;
#[doc = "LT register accessor: an alias for `Reg<LT_SPEC>`"]
pub type LT = crate::Reg<lt::LT_SPEC>;
#[doc = "desc LT"]
pub mod lt;
#[doc = "IFR register accessor: an alias for `Reg<IFR_SPEC>`"]
pub type IFR = crate::Reg<ifr::IFR_SPEC>;
#[doc = "desc IFR"]
pub mod ifr;
#[doc = "ICR register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "desc ICR"]
pub mod icr;
#[doc = "EXTTRIGGER0 register accessor: an alias for `Reg<EXTTRIGGER0_SPEC>`"]
pub type EXTTRIGGER0 = crate::Reg<exttrigger0::EXTTRIGGER0_SPEC>;
#[doc = "desc EXTTRIGGER0"]
pub mod exttrigger0;
#[doc = "EXTTRIGGER1 register accessor: an alias for `Reg<EXTTRIGGER1_SPEC>`"]
pub type EXTTRIGGER1 = crate::Reg<exttrigger1::EXTTRIGGER1_SPEC>;
#[doc = "desc EXTTRIGGER1"]
pub mod exttrigger1;
#[doc = "SGLSTART register accessor: an alias for `Reg<SGLSTART_SPEC>`"]
pub type SGLSTART = crate::Reg<sglstart::SGLSTART_SPEC>;
#[doc = "desc SGLSTART"]
pub mod sglstart;
#[doc = "SQRSTART register accessor: an alias for `Reg<SQRSTART_SPEC>`"]
pub type SQRSTART = crate::Reg<sqrstart::SQRSTART_SPEC>;
#[doc = "desc SQRSTART"]
pub mod sqrstart;
#[doc = "JQRSTART register accessor: an alias for `Reg<JQRSTART_SPEC>`"]
pub type JQRSTART = crate::Reg<jqrstart::JQRSTART_SPEC>;
#[doc = "desc JQRSTART"]
pub mod jqrstart;
