#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CRC Control"]
    pub ctl: crate::Reg<ctl::CTL_SPEC>,
    #[doc = "0x04 - CRC Data Input"]
    pub datain: crate::Reg<datain::DATAIN_SPEC>,
    #[doc = "0x08 - CRC Seed Value"]
    pub seed: crate::Reg<seed::SEED_SPEC>,
    #[doc = "0x0c - CRC Data Output"]
    pub dataout: crate::Reg<dataout::DATAOUT_SPEC>,
}
#[doc = "CTL register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "CRC Control"]
pub mod ctl;
#[doc = "DATAIN register accessor: an alias for `Reg<DATAIN_SPEC>`"]
pub type DATAIN = crate::Reg<datain::DATAIN_SPEC>;
#[doc = "CRC Data Input"]
pub mod datain;
#[doc = "SEED register accessor: an alias for `Reg<SEED_SPEC>`"]
pub type SEED = crate::Reg<seed::SEED_SPEC>;
#[doc = "CRC Seed Value"]
pub mod seed;
#[doc = "DATAOUT register accessor: an alias for `Reg<DATAOUT_SPEC>`"]
pub type DATAOUT = crate::Reg<dataout::DATAOUT_SPEC>;
#[doc = "CRC Data Output"]
pub mod dataout;
