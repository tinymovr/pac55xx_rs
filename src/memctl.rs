#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Memory Controller Configuration"]
    pub memctl: crate::Reg<memctl::MEMCTL_SPEC>,
    #[doc = "0x04 - Memory Controller Status"]
    pub memstatus: crate::Reg<memstatus::MEMSTATUS_SPEC>,
    #[doc = "0x08 - FLASH Lock Access"]
    pub flashlock: crate::Reg<flashlock::FLASHLOCK_SPEC>,
    #[doc = "0x0c - FLASH Page"]
    pub flashpage: crate::Reg<flashpage::FLASHPAGE_SPEC>,
    #[doc = "0x10 - SWD Unlock"]
    pub swdunlock: crate::Reg<swdunlock::SWDUNLOCK_SPEC>,
    _reserved5: [u8; 0x0c],
    #[doc = "0x20 - FLASH Erase"]
    pub flasherase: crate::Reg<flasherase::FLASHERASE_SPEC>,
}
#[doc = "MEMCTL register accessor: an alias for `Reg<MEMCTL_SPEC>`"]
pub type MEMCTL = crate::Reg<memctl::MEMCTL_SPEC>;
#[doc = "Memory Controller Configuration"]
pub mod memctl;
#[doc = "MEMSTATUS register accessor: an alias for `Reg<MEMSTATUS_SPEC>`"]
pub type MEMSTATUS = crate::Reg<memstatus::MEMSTATUS_SPEC>;
#[doc = "Memory Controller Status"]
pub mod memstatus;
#[doc = "FLASHLOCK register accessor: an alias for `Reg<FLASHLOCK_SPEC>`"]
pub type FLASHLOCK = crate::Reg<flashlock::FLASHLOCK_SPEC>;
#[doc = "FLASH Lock Access"]
pub mod flashlock;
#[doc = "FLASHPAGE register accessor: an alias for `Reg<FLASHPAGE_SPEC>`"]
pub type FLASHPAGE = crate::Reg<flashpage::FLASHPAGE_SPEC>;
#[doc = "FLASH Page"]
pub mod flashpage;
#[doc = "SWDUNLOCK register accessor: an alias for `Reg<SWDUNLOCK_SPEC>`"]
pub type SWDUNLOCK = crate::Reg<swdunlock::SWDUNLOCK_SPEC>;
#[doc = "SWD Unlock"]
pub mod swdunlock;
#[doc = "FLASHERASE register accessor: an alias for `Reg<FLASHERASE_SPEC>`"]
pub type FLASHERASE = crate::Reg<flasherase::FLASHERASE_SPEC>;
#[doc = "FLASH Erase"]
pub mod flasherase;
