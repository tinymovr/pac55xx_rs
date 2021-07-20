#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - UART C Receive Buffer / SSP C Control"]
    pub rbr_con: crate::Reg<rbr_con::RBR_CON_SPEC>,
    #[doc = "0x04 - UART C Transmit Holding / SSP C Status"]
    pub thr_stat: crate::Reg<thr_stat::THR_STAT_SPEC>,
    #[doc = "0x08 - UART C Divisor Latch / SSP C Data"]
    pub drl_dat: crate::Reg<drl_dat::DRL_DAT_SPEC>,
    #[doc = "0x0c - UART C Interrupt Enable / SSP C Clock Control"]
    pub ier_clk: crate::Reg<ier_clk::IER_CLK_SPEC>,
    #[doc = "0x10 - UART C Interrupt Identification / SSP C Interrupt Mask Set and Clear"]
    pub iir_imsc: crate::Reg<iir_imsc::IIR_IMSC_SPEC>,
    #[doc = "0x14 - UART C FIFO Control / SSP C Raw Interrupt Status"]
    pub fcr_ris: crate::Reg<fcr_ris::FCR_RIS_SPEC>,
    #[doc = "0x18 - UART C Line Control / SSP C Masked Interrupt Status"]
    pub lcr_mis: crate::Reg<lcr_mis::LCR_MIS_SPEC>,
    #[doc = "0x1c - SSP C Interrupt Clear"]
    pub iclr: crate::Reg<iclr::ICLR_SPEC>,
    #[doc = "0x20 - UART C Line Status"]
    pub lsr: crate::Reg<lsr::LSR_SPEC>,
    _reserved9: [u8; 0x04],
    #[doc = "0x28 - UART C Scratch Pad / SSP C Slave Select Configuration"]
    pub scr_sscr: crate::Reg<scr_sscr::SCR_SSCR_SPEC>,
    #[doc = "0x2c - Enhanced Feature"]
    pub efr: crate::Reg<efr::EFR_SPEC>,
}
#[doc = "RBR_CON register accessor: an alias for `Reg<RBR_CON_SPEC>`"]
pub type RBR_CON = crate::Reg<rbr_con::RBR_CON_SPEC>;
#[doc = "UART C Receive Buffer / SSP C Control"]
pub mod rbr_con;
#[doc = "THR_STAT register accessor: an alias for `Reg<THR_STAT_SPEC>`"]
pub type THR_STAT = crate::Reg<thr_stat::THR_STAT_SPEC>;
#[doc = "UART C Transmit Holding / SSP C Status"]
pub mod thr_stat;
#[doc = "DRL_DAT register accessor: an alias for `Reg<DRL_DAT_SPEC>`"]
pub type DRL_DAT = crate::Reg<drl_dat::DRL_DAT_SPEC>;
#[doc = "UART C Divisor Latch / SSP C Data"]
pub mod drl_dat;
#[doc = "IER_CLK register accessor: an alias for `Reg<IER_CLK_SPEC>`"]
pub type IER_CLK = crate::Reg<ier_clk::IER_CLK_SPEC>;
#[doc = "UART C Interrupt Enable / SSP C Clock Control"]
pub mod ier_clk;
#[doc = "IIR_IMSC register accessor: an alias for `Reg<IIR_IMSC_SPEC>`"]
pub type IIR_IMSC = crate::Reg<iir_imsc::IIR_IMSC_SPEC>;
#[doc = "UART C Interrupt Identification / SSP C Interrupt Mask Set and Clear"]
pub mod iir_imsc;
#[doc = "FCR_RIS register accessor: an alias for `Reg<FCR_RIS_SPEC>`"]
pub type FCR_RIS = crate::Reg<fcr_ris::FCR_RIS_SPEC>;
#[doc = "UART C FIFO Control / SSP C Raw Interrupt Status"]
pub mod fcr_ris;
#[doc = "LCR_MIS register accessor: an alias for `Reg<LCR_MIS_SPEC>`"]
pub type LCR_MIS = crate::Reg<lcr_mis::LCR_MIS_SPEC>;
#[doc = "UART C Line Control / SSP C Masked Interrupt Status"]
pub mod lcr_mis;
#[doc = "ICLR register accessor: an alias for `Reg<ICLR_SPEC>`"]
pub type ICLR = crate::Reg<iclr::ICLR_SPEC>;
#[doc = "SSP C Interrupt Clear"]
pub mod iclr;
#[doc = "LSR register accessor: an alias for `Reg<LSR_SPEC>`"]
pub type LSR = crate::Reg<lsr::LSR_SPEC>;
#[doc = "UART C Line Status"]
pub mod lsr;
#[doc = "SCR_SSCR register accessor: an alias for `Reg<SCR_SSCR_SPEC>`"]
pub type SCR_SSCR = crate::Reg<scr_sscr::SCR_SSCR_SPEC>;
#[doc = "UART C Scratch Pad / SSP C Slave Select Configuration"]
pub mod scr_sscr;
#[doc = "EFR register accessor: an alias for `Reg<EFR_SPEC>`"]
pub type EFR = crate::Reg<efr::EFR_SPEC>;
#[doc = "Enhanced Feature"]
pub mod efr;
