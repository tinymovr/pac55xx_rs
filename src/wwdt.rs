#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - WWDT Control"]
    pub ctl: crate::Reg<ctl::CTL_SPEC>,
    #[doc = "0x04 - WWDT Load Counter Value"]
    pub load: crate::Reg<load::LOAD_SPEC>,
    #[doc = "0x08 - WWDT Counter"]
    pub ctr: crate::Reg<ctr::CTR_SPEC>,
    #[doc = "0x0c - WWDT Interrupt Flag"]
    pub intf: crate::Reg<intf::INTF_SPEC>,
    #[doc = "0x10 - WWDT Interrupt Clear"]
    pub intclr: crate::Reg<intclr::INTCLR_SPEC>,
    #[doc = "0x14 - WWDT Lock"]
    pub lock: crate::Reg<lock::LOCK_SPEC>,
}
#[doc = "CTL register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "WWDT Control"]
pub mod ctl;
#[doc = "LOAD register accessor: an alias for `Reg<LOAD_SPEC>`"]
pub type LOAD = crate::Reg<load::LOAD_SPEC>;
#[doc = "WWDT Load Counter Value"]
pub mod load;
#[doc = "CTR register accessor: an alias for `Reg<CTR_SPEC>`"]
pub type CTR = crate::Reg<ctr::CTR_SPEC>;
#[doc = "WWDT Counter"]
pub mod ctr;
#[doc = "INTF register accessor: an alias for `Reg<INTF_SPEC>`"]
pub type INTF = crate::Reg<intf::INTF_SPEC>;
#[doc = "WWDT Interrupt Flag"]
pub mod intf;
#[doc = "INTCLR register accessor: an alias for `Reg<INTCLR_SPEC>`"]
pub type INTCLR = crate::Reg<intclr::INTCLR_SPEC>;
#[doc = "WWDT Interrupt Clear"]
pub mod intclr;
#[doc = "LOCK register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "WWDT Lock"]
pub mod lock;
