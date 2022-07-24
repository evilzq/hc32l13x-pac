#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - desc DEBUG_ACTIVE"]
    pub debug_active: crate::Reg<debug_active::DEBUG_ACTIVE_SPEC>,
}
#[doc = "DEBUG_ACTIVE register accessor: an alias for `Reg<DEBUG_ACTIVE_SPEC>`"]
pub type DEBUG_ACTIVE = crate::Reg<debug_active::DEBUG_ACTIVE_SPEC>;
#[doc = "desc DEBUG_ACTIVE"]
pub mod debug_active;
