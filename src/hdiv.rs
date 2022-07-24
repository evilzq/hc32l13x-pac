#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - desc DIVIDEND"]
    pub dividend: crate::Reg<dividend::DIVIDEND_SPEC>,
    #[doc = "0x04 - desc DIVISOR"]
    pub divisor: crate::Reg<divisor::DIVISOR_SPEC>,
    #[doc = "0x08 - desc QUOTIENT"]
    pub quotient: crate::Reg<quotient::QUOTIENT_SPEC>,
    #[doc = "0x0c - desc REMAINDER"]
    pub remainder: crate::Reg<remainder::REMAINDER_SPEC>,
    #[doc = "0x10 - desc SIGN"]
    pub sign: crate::Reg<sign::SIGN_SPEC>,
    #[doc = "0x14 - desc STAT"]
    pub stat: crate::Reg<stat::STAT_SPEC>,
}
#[doc = "DIVIDEND register accessor: an alias for `Reg<DIVIDEND_SPEC>`"]
pub type DIVIDEND = crate::Reg<dividend::DIVIDEND_SPEC>;
#[doc = "desc DIVIDEND"]
pub mod dividend;
#[doc = "DIVISOR register accessor: an alias for `Reg<DIVISOR_SPEC>`"]
pub type DIVISOR = crate::Reg<divisor::DIVISOR_SPEC>;
#[doc = "desc DIVISOR"]
pub mod divisor;
#[doc = "QUOTIENT register accessor: an alias for `Reg<QUOTIENT_SPEC>`"]
pub type QUOTIENT = crate::Reg<quotient::QUOTIENT_SPEC>;
#[doc = "desc QUOTIENT"]
pub mod quotient;
#[doc = "REMAINDER register accessor: an alias for `Reg<REMAINDER_SPEC>`"]
pub type REMAINDER = crate::Reg<remainder::REMAINDER_SPEC>;
#[doc = "desc REMAINDER"]
pub mod remainder;
#[doc = "SIGN register accessor: an alias for `Reg<SIGN_SPEC>`"]
pub type SIGN = crate::Reg<sign::SIGN_SPEC>;
#[doc = "desc SIGN"]
pub mod sign;
#[doc = "STAT register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "desc STAT"]
pub mod stat;
