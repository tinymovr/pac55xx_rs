#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPTimer A Control"]
    pub ctl: crate::Reg<ctl::CTL_SPEC>,
    #[doc = "0x04 - GPTimer A Counter"]
    pub ctr: crate::Reg<ctr::CTR_SPEC>,
}
#[doc = "CTL register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "GPTimer A Control"]
pub mod ctl;
#[doc = "CTR register accessor: an alias for `Reg<CTR_SPEC>`"]
pub type CTR = crate::Reg<ctr::CTR_SPEC>;
#[doc = "GPTimer A Counter"]
pub mod ctr;
