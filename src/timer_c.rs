#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer C Control"]
    pub ctl: crate::Reg<ctl::CTL_SPEC>,
    #[doc = "0x04 - Timer C Interrupt Control"]
    pub int: crate::Reg<int::INT_SPEC>,
    #[doc = "0x08 - Timer C Period"]
    pub prd: crate::Reg<prd::PRD_SPEC>,
    #[doc = "0x0c - Timer C Counter"]
    pub ctr: crate::Reg<ctr::CTR_SPEC>,
    #[doc = "0x10 - Timer C QEP Control"]
    pub qepctl: crate::Reg<qepctl::QEPCTL_SPEC>,
    _reserved5: [u8; 0xec],
    #[doc = "0x100 - Timer C CC Control 0"]
    pub cctl0: crate::Reg<cctl0::CCTL0_SPEC>,
    #[doc = "0x104 - Timer C CC Counter 0"]
    pub cctr0: crate::Reg<cctr0::CCTR0_SPEC>,
    #[doc = "0x108 - Timer C CC Control 1"]
    pub cctl1: crate::Reg<cctl1::CCTL1_SPEC>,
    #[doc = "0x10c - Timer C CC Counter 1"]
    pub cctr1: crate::Reg<cctr1::CCTR1_SPEC>,
    #[doc = "0x110 - Timer C CC Control 2"]
    pub cctl2: crate::Reg<cctl2::CCTL2_SPEC>,
    #[doc = "0x114 - Timer C CC Counter 2"]
    pub cctr2: crate::Reg<cctr2::CCTR2_SPEC>,
    #[doc = "0x118 - Timer C CC Control 3"]
    pub cctl3: crate::Reg<cctl3::CCTL3_SPEC>,
    #[doc = "0x11c - Timer C CC Counter 3"]
    pub cctr3: crate::Reg<cctr3::CCTR3_SPEC>,
    #[doc = "0x120 - Timer C CC Control 4"]
    pub cctl4: crate::Reg<cctl4::CCTL4_SPEC>,
    #[doc = "0x124 - Timer C CC Counter 4"]
    pub cctr4: crate::Reg<cctr4::CCTR4_SPEC>,
    #[doc = "0x128 - Timer C CC Control 5"]
    pub cctl5: crate::Reg<cctl5::CCTL5_SPEC>,
    #[doc = "0x12c - Timer C CC Counter 5"]
    pub cctr5: crate::Reg<cctr5::CCTR5_SPEC>,
    #[doc = "0x130 - Timer C CC Control 6"]
    pub cctl6: crate::Reg<cctl6::CCTL6_SPEC>,
    #[doc = "0x134 - Timer C CC Counter 6"]
    pub cctr6: crate::Reg<cctr6::CCTR6_SPEC>,
    #[doc = "0x138 - Timer C CC Control 7"]
    pub cctl7: crate::Reg<cctl7::CCTL7_SPEC>,
    #[doc = "0x13c - Timer C CC Counter 7"]
    pub cctr7: crate::Reg<cctr7::CCTR7_SPEC>,
    _reserved21: [u8; 0xc0],
    #[doc = "0x200 - Timer C DTG Control 0"]
    pub dtgctl0: crate::Reg<dtgctl0::DTGCTL0_SPEC>,
    #[doc = "0x204 - Timer C DTG Control 1"]
    pub dtgctl1: crate::Reg<dtgctl1::DTGCTL1_SPEC>,
    #[doc = "0x208 - Timer C DTG Control 2"]
    pub dtgctl2: crate::Reg<dtgctl2::DTGCTL2_SPEC>,
    #[doc = "0x20c - Timer C DTG Control 3"]
    pub dtgctl3: crate::Reg<dtgctl3::DTGCTL3_SPEC>,
}
#[doc = "CTL register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Timer C Control"]
pub mod ctl;
#[doc = "INT register accessor: an alias for `Reg<INT_SPEC>`"]
pub type INT = crate::Reg<int::INT_SPEC>;
#[doc = "Timer C Interrupt Control"]
pub mod int;
#[doc = "PRD register accessor: an alias for `Reg<PRD_SPEC>`"]
pub type PRD = crate::Reg<prd::PRD_SPEC>;
#[doc = "Timer C Period"]
pub mod prd;
#[doc = "CTR register accessor: an alias for `Reg<CTR_SPEC>`"]
pub type CTR = crate::Reg<ctr::CTR_SPEC>;
#[doc = "Timer C Counter"]
pub mod ctr;
#[doc = "QEPCTL register accessor: an alias for `Reg<QEPCTL_SPEC>`"]
pub type QEPCTL = crate::Reg<qepctl::QEPCTL_SPEC>;
#[doc = "Timer C QEP Control"]
pub mod qepctl;
#[doc = "CCTL0 register accessor: an alias for `Reg<CCTL0_SPEC>`"]
pub type CCTL0 = crate::Reg<cctl0::CCTL0_SPEC>;
#[doc = "Timer C CC Control 0"]
pub mod cctl0;
#[doc = "CCTR0 register accessor: an alias for `Reg<CCTR0_SPEC>`"]
pub type CCTR0 = crate::Reg<cctr0::CCTR0_SPEC>;
#[doc = "Timer C CC Counter 0"]
pub mod cctr0;
#[doc = "CCTL1 register accessor: an alias for `Reg<CCTL1_SPEC>`"]
pub type CCTL1 = crate::Reg<cctl1::CCTL1_SPEC>;
#[doc = "Timer C CC Control 1"]
pub mod cctl1;
#[doc = "CCTR1 register accessor: an alias for `Reg<CCTR1_SPEC>`"]
pub type CCTR1 = crate::Reg<cctr1::CCTR1_SPEC>;
#[doc = "Timer C CC Counter 1"]
pub mod cctr1;
#[doc = "CCTL2 register accessor: an alias for `Reg<CCTL2_SPEC>`"]
pub type CCTL2 = crate::Reg<cctl2::CCTL2_SPEC>;
#[doc = "Timer C CC Control 2"]
pub mod cctl2;
#[doc = "CCTR2 register accessor: an alias for `Reg<CCTR2_SPEC>`"]
pub type CCTR2 = crate::Reg<cctr2::CCTR2_SPEC>;
#[doc = "Timer C CC Counter 2"]
pub mod cctr2;
#[doc = "CCTL3 register accessor: an alias for `Reg<CCTL3_SPEC>`"]
pub type CCTL3 = crate::Reg<cctl3::CCTL3_SPEC>;
#[doc = "Timer C CC Control 3"]
pub mod cctl3;
#[doc = "CCTR3 register accessor: an alias for `Reg<CCTR3_SPEC>`"]
pub type CCTR3 = crate::Reg<cctr3::CCTR3_SPEC>;
#[doc = "Timer C CC Counter 3"]
pub mod cctr3;
#[doc = "CCTL4 register accessor: an alias for `Reg<CCTL4_SPEC>`"]
pub type CCTL4 = crate::Reg<cctl4::CCTL4_SPEC>;
#[doc = "Timer C CC Control 4"]
pub mod cctl4;
#[doc = "CCTR4 register accessor: an alias for `Reg<CCTR4_SPEC>`"]
pub type CCTR4 = crate::Reg<cctr4::CCTR4_SPEC>;
#[doc = "Timer C CC Counter 4"]
pub mod cctr4;
#[doc = "CCTL5 register accessor: an alias for `Reg<CCTL5_SPEC>`"]
pub type CCTL5 = crate::Reg<cctl5::CCTL5_SPEC>;
#[doc = "Timer C CC Control 5"]
pub mod cctl5;
#[doc = "CCTR5 register accessor: an alias for `Reg<CCTR5_SPEC>`"]
pub type CCTR5 = crate::Reg<cctr5::CCTR5_SPEC>;
#[doc = "Timer C CC Counter 5"]
pub mod cctr5;
#[doc = "CCTL6 register accessor: an alias for `Reg<CCTL6_SPEC>`"]
pub type CCTL6 = crate::Reg<cctl6::CCTL6_SPEC>;
#[doc = "Timer C CC Control 6"]
pub mod cctl6;
#[doc = "CCTR6 register accessor: an alias for `Reg<CCTR6_SPEC>`"]
pub type CCTR6 = crate::Reg<cctr6::CCTR6_SPEC>;
#[doc = "Timer C CC Counter 6"]
pub mod cctr6;
#[doc = "CCTL7 register accessor: an alias for `Reg<CCTL7_SPEC>`"]
pub type CCTL7 = crate::Reg<cctl7::CCTL7_SPEC>;
#[doc = "Timer C CC Control 7"]
pub mod cctl7;
#[doc = "CCTR7 register accessor: an alias for `Reg<CCTR7_SPEC>`"]
pub type CCTR7 = crate::Reg<cctr7::CCTR7_SPEC>;
#[doc = "Timer C CC Counter 7"]
pub mod cctr7;
#[doc = "DTGCTL0 register accessor: an alias for `Reg<DTGCTL0_SPEC>`"]
pub type DTGCTL0 = crate::Reg<dtgctl0::DTGCTL0_SPEC>;
#[doc = "Timer C DTG Control 0"]
pub mod dtgctl0;
#[doc = "DTGCTL1 register accessor: an alias for `Reg<DTGCTL1_SPEC>`"]
pub type DTGCTL1 = crate::Reg<dtgctl1::DTGCTL1_SPEC>;
#[doc = "Timer C DTG Control 1"]
pub mod dtgctl1;
#[doc = "DTGCTL2 register accessor: an alias for `Reg<DTGCTL2_SPEC>`"]
pub type DTGCTL2 = crate::Reg<dtgctl2::DTGCTL2_SPEC>;
#[doc = "Timer C DTG Control 2"]
pub mod dtgctl2;
#[doc = "DTGCTL3 register accessor: an alias for `Reg<DTGCTL3_SPEC>`"]
pub type DTGCTL3 = crate::Reg<dtgctl3::DTGCTL3_SPEC>;
#[doc = "Timer C DTG Control 3"]
pub mod dtgctl3;
