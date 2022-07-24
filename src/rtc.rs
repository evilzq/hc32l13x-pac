#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - desc CR0"]
    pub cr0: crate::Reg<cr0::CR0_SPEC>,
    #[doc = "0x04 - desc CR1"]
    pub cr1: crate::Reg<cr1::CR1_SPEC>,
    #[doc = "0x08 - desc SEC"]
    pub sec: crate::Reg<sec::SEC_SPEC>,
    #[doc = "0x0c - desc MIN"]
    pub min: crate::Reg<min::MIN_SPEC>,
    #[doc = "0x10 - desc HOUR"]
    pub hour: crate::Reg<hour::HOUR_SPEC>,
    #[doc = "0x14 - desc WEEK"]
    pub week: crate::Reg<week::WEEK_SPEC>,
    #[doc = "0x18 - desc DAY"]
    pub day: crate::Reg<day::DAY_SPEC>,
    #[doc = "0x1c - desc MON"]
    pub mon: crate::Reg<mon::MON_SPEC>,
    #[doc = "0x20 - desc YEAR"]
    pub year: crate::Reg<year::YEAR_SPEC>,
    #[doc = "0x24 - desc ALMMIN"]
    pub almmin: crate::Reg<almmin::ALMMIN_SPEC>,
    #[doc = "0x28 - desc ALMHOUR"]
    pub almhour: crate::Reg<almhour::ALMHOUR_SPEC>,
    #[doc = "0x2c - desc ALMWEEK"]
    pub almweek: crate::Reg<almweek::ALMWEEK_SPEC>,
    #[doc = "0x30 - desc COMPEN"]
    pub compen: crate::Reg<compen::COMPEN_SPEC>,
}
#[doc = "CR0 register accessor: an alias for `Reg<CR0_SPEC>`"]
pub type CR0 = crate::Reg<cr0::CR0_SPEC>;
#[doc = "desc CR0"]
pub mod cr0;
#[doc = "CR1 register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "desc CR1"]
pub mod cr1;
#[doc = "SEC register accessor: an alias for `Reg<SEC_SPEC>`"]
pub type SEC = crate::Reg<sec::SEC_SPEC>;
#[doc = "desc SEC"]
pub mod sec;
#[doc = "MIN register accessor: an alias for `Reg<MIN_SPEC>`"]
pub type MIN = crate::Reg<min::MIN_SPEC>;
#[doc = "desc MIN"]
pub mod min;
#[doc = "HOUR register accessor: an alias for `Reg<HOUR_SPEC>`"]
pub type HOUR = crate::Reg<hour::HOUR_SPEC>;
#[doc = "desc HOUR"]
pub mod hour;
#[doc = "WEEK register accessor: an alias for `Reg<WEEK_SPEC>`"]
pub type WEEK = crate::Reg<week::WEEK_SPEC>;
#[doc = "desc WEEK"]
pub mod week;
#[doc = "DAY register accessor: an alias for `Reg<DAY_SPEC>`"]
pub type DAY = crate::Reg<day::DAY_SPEC>;
#[doc = "desc DAY"]
pub mod day;
#[doc = "MON register accessor: an alias for `Reg<MON_SPEC>`"]
pub type MON = crate::Reg<mon::MON_SPEC>;
#[doc = "desc MON"]
pub mod mon;
#[doc = "YEAR register accessor: an alias for `Reg<YEAR_SPEC>`"]
pub type YEAR = crate::Reg<year::YEAR_SPEC>;
#[doc = "desc YEAR"]
pub mod year;
#[doc = "ALMMIN register accessor: an alias for `Reg<ALMMIN_SPEC>`"]
pub type ALMMIN = crate::Reg<almmin::ALMMIN_SPEC>;
#[doc = "desc ALMMIN"]
pub mod almmin;
#[doc = "ALMHOUR register accessor: an alias for `Reg<ALMHOUR_SPEC>`"]
pub type ALMHOUR = crate::Reg<almhour::ALMHOUR_SPEC>;
#[doc = "desc ALMHOUR"]
pub mod almhour;
#[doc = "ALMWEEK register accessor: an alias for `Reg<ALMWEEK_SPEC>`"]
pub type ALMWEEK = crate::Reg<almweek::ALMWEEK_SPEC>;
#[doc = "desc ALMWEEK"]
pub mod almweek;
#[doc = "COMPEN register accessor: an alias for `Reg<COMPEN_SPEC>`"]
pub type COMPEN = crate::Reg<compen::COMPEN_SPEC>;
#[doc = "desc COMPEN"]
pub mod compen;
