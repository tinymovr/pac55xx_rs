#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Combined CAN MR, CMR, SR, and ISR registers"]
    pub isr_sr_cmr_mr: crate::Reg<isr_sr_cmr_mr::ISR_SR_CMR_MR_SPEC>,
    #[doc = "0x04 - Combined CAN IMR, RMC, BTR0, and BTR1 registers"]
    pub btr1_btr0_rmc_imr: crate::Reg<btr1_btr0_rmc_imr::BTR1_BTR0_RMC_IMR_SPEC>,
    #[doc = "0x08 - Combined CAN Transmit Buffer registers"]
    pub txbuf: crate::Reg<txbuf::TXBUF_SPEC>,
    #[doc = "0x0c - Combined CAN Receive Buffer registers"]
    pub rxbuf: crate::Reg<rxbuf::RXBUF_SPEC>,
    #[doc = "0x10 - Combined CAN Acceptance Code registers"]
    pub acr: crate::Reg<acr::ACR_SPEC>,
    #[doc = "0x14 - Combined CAN Acceptance Mask registers"]
    pub amr: crate::Reg<amr::AMR_SPEC>,
    #[doc = "0x18 - Combined CAN ECC, RXERR, TXERR, and ALC registers"]
    pub alc_txerr_rxerr_ecc: crate::Reg<alc_txerr_rxerr_ecc::ALC_TXERR_RXERR_ECC_SPEC>,
}
#[doc = "ISR_SR_CMR_MR register accessor: an alias for `Reg<ISR_SR_CMR_MR_SPEC>`"]
pub type ISR_SR_CMR_MR = crate::Reg<isr_sr_cmr_mr::ISR_SR_CMR_MR_SPEC>;
#[doc = "Combined CAN MR, CMR, SR, and ISR registers"]
pub mod isr_sr_cmr_mr;
#[doc = "BTR1_BTR0_RMC_IMR register accessor: an alias for `Reg<BTR1_BTR0_RMC_IMR_SPEC>`"]
pub type BTR1_BTR0_RMC_IMR = crate::Reg<btr1_btr0_rmc_imr::BTR1_BTR0_RMC_IMR_SPEC>;
#[doc = "Combined CAN IMR, RMC, BTR0, and BTR1 registers"]
pub mod btr1_btr0_rmc_imr;
#[doc = "TXBUF register accessor: an alias for `Reg<TXBUF_SPEC>`"]
pub type TXBUF = crate::Reg<txbuf::TXBUF_SPEC>;
#[doc = "Combined CAN Transmit Buffer registers"]
pub mod txbuf;
#[doc = "RXBUF register accessor: an alias for `Reg<RXBUF_SPEC>`"]
pub type RXBUF = crate::Reg<rxbuf::RXBUF_SPEC>;
#[doc = "Combined CAN Receive Buffer registers"]
pub mod rxbuf;
#[doc = "ACR register accessor: an alias for `Reg<ACR_SPEC>`"]
pub type ACR = crate::Reg<acr::ACR_SPEC>;
#[doc = "Combined CAN Acceptance Code registers"]
pub mod acr;
#[doc = "AMR register accessor: an alias for `Reg<AMR_SPEC>`"]
pub type AMR = crate::Reg<amr::AMR_SPEC>;
#[doc = "Combined CAN Acceptance Mask registers"]
pub mod amr;
#[doc = "ALC_TXERR_RXERR_ECC register accessor: an alias for `Reg<ALC_TXERR_RXERR_ECC_SPEC>`"]
pub type ALC_TXERR_RXERR_ECC = crate::Reg<alc_txerr_rxerr_ecc::ALC_TXERR_RXERR_ECC_SPEC>;
#[doc = "Combined CAN ECC, RXERR, TXERR, and ALC registers"]
pub mod alc_txerr_rxerr_ecc;
