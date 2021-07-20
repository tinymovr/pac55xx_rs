#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I2C Control Set"]
    pub conset: crate::Reg<conset::CONSET_SPEC>,
    #[doc = "0x04 - I2C Control Clear"]
    pub conclr: crate::Reg<conclr::CONCLR_SPEC>,
    #[doc = "0x08 - I2C Status"]
    pub stat: crate::Reg<stat::STAT_SPEC>,
    #[doc = "0x0c - I2C Data"]
    pub dat: crate::Reg<dat::DAT_SPEC>,
    #[doc = "0x10 - I2C Clock Control"]
    pub clk: crate::Reg<clk::CLK_SPEC>,
    #[doc = "0x14 - I2C Slave Address 0"]
    pub adr0: crate::Reg<adr0::ADR0_SPEC>,
    #[doc = "0x18 - I2C Slave Address Mask 0"]
    pub adrm0: crate::Reg<adrm0::ADRM0_SPEC>,
    #[doc = "0x1c - I2C Extended Slave Address 0"]
    pub xadr0: crate::Reg<xadr0::XADR0_SPEC>,
    #[doc = "0x20 - I2C Extended Slave Address Mask 0"]
    pub xadrm0: crate::Reg<xadrm0::XADRM0_SPEC>,
    #[doc = "0x24 - I2C Software Reset"]
    pub rst: crate::Reg<rst::RST_SPEC>,
    #[doc = "0x28 - I2C Slave Address 1"]
    pub adr1: crate::Reg<adr1::ADR1_SPEC>,
    #[doc = "0x2c - I2C Slave Address Mask 1"]
    pub adrm1: crate::Reg<adrm1::ADRM1_SPEC>,
    #[doc = "0x30 - I2C Slave Address 2"]
    pub adr2: crate::Reg<adr2::ADR2_SPEC>,
    #[doc = "0x34 - I2C Slave Address Mask 2"]
    pub adrm2: crate::Reg<adrm2::ADRM2_SPEC>,
    #[doc = "0x38 - I2C Slave Address 3"]
    pub adr3: crate::Reg<adr3::ADR3_SPEC>,
    #[doc = "0x3c - I2C Slave Address Mask 3"]
    pub adrm3: crate::Reg<adrm3::ADRM3_SPEC>,
}
#[doc = "CONSET register accessor: an alias for `Reg<CONSET_SPEC>`"]
pub type CONSET = crate::Reg<conset::CONSET_SPEC>;
#[doc = "I2C Control Set"]
pub mod conset;
#[doc = "CONCLR register accessor: an alias for `Reg<CONCLR_SPEC>`"]
pub type CONCLR = crate::Reg<conclr::CONCLR_SPEC>;
#[doc = "I2C Control Clear"]
pub mod conclr;
#[doc = "STAT register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "I2C Status"]
pub mod stat;
#[doc = "DAT register accessor: an alias for `Reg<DAT_SPEC>`"]
pub type DAT = crate::Reg<dat::DAT_SPEC>;
#[doc = "I2C Data"]
pub mod dat;
#[doc = "CLK register accessor: an alias for `Reg<CLK_SPEC>`"]
pub type CLK = crate::Reg<clk::CLK_SPEC>;
#[doc = "I2C Clock Control"]
pub mod clk;
#[doc = "ADR0 register accessor: an alias for `Reg<ADR0_SPEC>`"]
pub type ADR0 = crate::Reg<adr0::ADR0_SPEC>;
#[doc = "I2C Slave Address 0"]
pub mod adr0;
#[doc = "ADRM0 register accessor: an alias for `Reg<ADRM0_SPEC>`"]
pub type ADRM0 = crate::Reg<adrm0::ADRM0_SPEC>;
#[doc = "I2C Slave Address Mask 0"]
pub mod adrm0;
#[doc = "XADR0 register accessor: an alias for `Reg<XADR0_SPEC>`"]
pub type XADR0 = crate::Reg<xadr0::XADR0_SPEC>;
#[doc = "I2C Extended Slave Address 0"]
pub mod xadr0;
#[doc = "XADRM0 register accessor: an alias for `Reg<XADRM0_SPEC>`"]
pub type XADRM0 = crate::Reg<xadrm0::XADRM0_SPEC>;
#[doc = "I2C Extended Slave Address Mask 0"]
pub mod xadrm0;
#[doc = "RST register accessor: an alias for `Reg<RST_SPEC>`"]
pub type RST = crate::Reg<rst::RST_SPEC>;
#[doc = "I2C Software Reset"]
pub mod rst;
#[doc = "ADR1 register accessor: an alias for `Reg<ADR1_SPEC>`"]
pub type ADR1 = crate::Reg<adr1::ADR1_SPEC>;
#[doc = "I2C Slave Address 1"]
pub mod adr1;
#[doc = "ADRM1 register accessor: an alias for `Reg<ADRM1_SPEC>`"]
pub type ADRM1 = crate::Reg<adrm1::ADRM1_SPEC>;
#[doc = "I2C Slave Address Mask 1"]
pub mod adrm1;
#[doc = "ADR2 register accessor: an alias for `Reg<ADR2_SPEC>`"]
pub type ADR2 = crate::Reg<adr2::ADR2_SPEC>;
#[doc = "I2C Slave Address 2"]
pub mod adr2;
#[doc = "ADRM2 register accessor: an alias for `Reg<ADRM2_SPEC>`"]
pub type ADRM2 = crate::Reg<adrm2::ADRM2_SPEC>;
#[doc = "I2C Slave Address Mask 2"]
pub mod adrm2;
#[doc = "ADR3 register accessor: an alias for `Reg<ADR3_SPEC>`"]
pub type ADR3 = crate::Reg<adr3::ADR3_SPEC>;
#[doc = "I2C Slave Address 3"]
pub mod adr3;
#[doc = "ADRM3 register accessor: an alias for `Reg<ADRM3_SPEC>`"]
pub type ADRM3 = crate::Reg<adrm3::ADRM3_SPEC>;
#[doc = "I2C Slave Address Mask 3"]
pub mod adrm3;
