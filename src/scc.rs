#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DESC"]
    pub ccsctl: crate::Reg<ccsctl::CCSCTL_SPEC>,
    #[doc = "0x04 - DESC"]
    pub ccspllctl: crate::Reg<ccspllctl::CCSPLLCTL_SPEC>,
    #[doc = "0x08 - DESC"]
    pub ccsrosctrim: crate::Reg<ccsrosctrim::CCSROSCTRIM_SPEC>,
    #[doc = "0x0c - PA Peripheral MUX Select"]
    pub pamuxsel: crate::Reg<pamuxsel::PAMUXSEL_SPEC>,
    #[doc = "0x10 - PB Peripheral MUX Select"]
    pub pbmuxsel: crate::Reg<pbmuxsel::PBMUXSEL_SPEC>,
    #[doc = "0x14 - PC Peripheral MUX Select"]
    pub pcmuxsel: crate::Reg<pcmuxsel::PCMUXSEL_SPEC>,
    #[doc = "0x18 - PD Peripheral MUX Select"]
    pub pdmuxsel: crate::Reg<pdmuxsel::PDMUXSEL_SPEC>,
    #[doc = "0x1c - PE Peripheral MUX Select"]
    pub pemuxsel: crate::Reg<pemuxsel::PEMUXSEL_SPEC>,
    #[doc = "0x20 - PF Peripheral MUX Select"]
    pub pfmuxsel: crate::Reg<pfmuxsel::PFMUXSEL_SPEC>,
    #[doc = "0x24 - PG Peripheral MUX Select"]
    pub pgmuxsel: crate::Reg<pgmuxsel::PGMUXSEL_SPEC>,
    #[doc = "0x28 - PA Weak Pull-up Enable"]
    pub papuen: crate::Reg<papuen::PAPUEN_SPEC>,
    #[doc = "0x2c - PB Weak Pull-up Enable"]
    pub pbpuen: crate::Reg<pbpuen::PBPUEN_SPEC>,
    #[doc = "0x30 - PC Weak Pull-up Enable"]
    pub pcpuen: crate::Reg<pcpuen::PCPUEN_SPEC>,
    #[doc = "0x34 - PD Weak Pull-up Enable"]
    pub pdpuen: crate::Reg<pdpuen::PDPUEN_SPEC>,
    #[doc = "0x38 - PE Weak Pull-up Enable"]
    pub pepuen: crate::Reg<pepuen::PEPUEN_SPEC>,
    #[doc = "0x3c - PF Weak Pull-up Enable"]
    pub pfpuen: crate::Reg<pfpuen::PFPUEN_SPEC>,
    #[doc = "0x40 - PG Weak Pull-up Enable"]
    pub pgpuen: crate::Reg<pgpuen::PGPUEN_SPEC>,
    #[doc = "0x44 - PA Weak Pull-down Enable"]
    pub papden: crate::Reg<papden::PAPDEN_SPEC>,
    #[doc = "0x48 - PB Weak Pull-down Enable"]
    pub pbpden: crate::Reg<pbpden::PBPDEN_SPEC>,
    #[doc = "0x4c - PC Weak Pull-down Enable"]
    pub pcpden: crate::Reg<pcpden::PCPDEN_SPEC>,
    #[doc = "0x50 - PD Weak Pull-down Enable"]
    pub pdpden: crate::Reg<pdpden::PDPDEN_SPEC>,
    #[doc = "0x54 - PE Weak Pull-down Enable"]
    pub pepden: crate::Reg<pepden::PEPDEN_SPEC>,
    #[doc = "0x58 - PF Weak Pull-down Enable"]
    pub pfpden: crate::Reg<pfpden::PFPDEN_SPEC>,
    #[doc = "0x5c - PG Weak Pull-down Enable"]
    pub pgpden: crate::Reg<pgpden::PGPDEN_SPEC>,
    #[doc = "0x60 - PA Drive Strength/Schmitt Trigger"]
    pub pads: crate::Reg<pads::PADS_SPEC>,
    #[doc = "0x64 - PB Drive Strength/Schmitt Trigger"]
    pub pbds: crate::Reg<pbds::PBDS_SPEC>,
    #[doc = "0x68 - PC Drive Strength/Schmitt Trigger"]
    pub pcds: crate::Reg<pcds::PCDS_SPEC>,
    #[doc = "0x6c - PD Drive Strength/Schmitt Trigger"]
    pub pdds: crate::Reg<pdds::PDDS_SPEC>,
    #[doc = "0x70 - PE Drive Strength/Schmitt Trigger"]
    pub peds: crate::Reg<peds::PEDS_SPEC>,
    #[doc = "0x74 - PF Drive Strength/Schmitt Trigger"]
    pub pfds: crate::Reg<pfds::PFDS_SPEC>,
    #[doc = "0x78 - PG Drive Strength/Schmitt Trigger"]
    pub pgds: crate::Reg<pgds::PGDS_SPEC>,
}
#[doc = "CCSCTL register accessor: an alias for `Reg<CCSCTL_SPEC>`"]
pub type CCSCTL = crate::Reg<ccsctl::CCSCTL_SPEC>;
#[doc = "DESC"]
pub mod ccsctl;
#[doc = "CCSPLLCTL register accessor: an alias for `Reg<CCSPLLCTL_SPEC>`"]
pub type CCSPLLCTL = crate::Reg<ccspllctl::CCSPLLCTL_SPEC>;
#[doc = "DESC"]
pub mod ccspllctl;
#[doc = "CCSROSCTRIM register accessor: an alias for `Reg<CCSROSCTRIM_SPEC>`"]
pub type CCSROSCTRIM = crate::Reg<ccsrosctrim::CCSROSCTRIM_SPEC>;
#[doc = "DESC"]
pub mod ccsrosctrim;
#[doc = "PAMUXSEL register accessor: an alias for `Reg<PAMUXSEL_SPEC>`"]
pub type PAMUXSEL = crate::Reg<pamuxsel::PAMUXSEL_SPEC>;
#[doc = "PA Peripheral MUX Select"]
pub mod pamuxsel;
#[doc = "PBMUXSEL register accessor: an alias for `Reg<PBMUXSEL_SPEC>`"]
pub type PBMUXSEL = crate::Reg<pbmuxsel::PBMUXSEL_SPEC>;
#[doc = "PB Peripheral MUX Select"]
pub mod pbmuxsel;
#[doc = "PCMUXSEL register accessor: an alias for `Reg<PCMUXSEL_SPEC>`"]
pub type PCMUXSEL = crate::Reg<pcmuxsel::PCMUXSEL_SPEC>;
#[doc = "PC Peripheral MUX Select"]
pub mod pcmuxsel;
#[doc = "PDMUXSEL register accessor: an alias for `Reg<PDMUXSEL_SPEC>`"]
pub type PDMUXSEL = crate::Reg<pdmuxsel::PDMUXSEL_SPEC>;
#[doc = "PD Peripheral MUX Select"]
pub mod pdmuxsel;
#[doc = "PEMUXSEL register accessor: an alias for `Reg<PEMUXSEL_SPEC>`"]
pub type PEMUXSEL = crate::Reg<pemuxsel::PEMUXSEL_SPEC>;
#[doc = "PE Peripheral MUX Select"]
pub mod pemuxsel;
#[doc = "PFMUXSEL register accessor: an alias for `Reg<PFMUXSEL_SPEC>`"]
pub type PFMUXSEL = crate::Reg<pfmuxsel::PFMUXSEL_SPEC>;
#[doc = "PF Peripheral MUX Select"]
pub mod pfmuxsel;
#[doc = "PGMUXSEL register accessor: an alias for `Reg<PGMUXSEL_SPEC>`"]
pub type PGMUXSEL = crate::Reg<pgmuxsel::PGMUXSEL_SPEC>;
#[doc = "PG Peripheral MUX Select"]
pub mod pgmuxsel;
#[doc = "PAPUEN register accessor: an alias for `Reg<PAPUEN_SPEC>`"]
pub type PAPUEN = crate::Reg<papuen::PAPUEN_SPEC>;
#[doc = "PA Weak Pull-up Enable"]
pub mod papuen;
#[doc = "PBPUEN register accessor: an alias for `Reg<PBPUEN_SPEC>`"]
pub type PBPUEN = crate::Reg<pbpuen::PBPUEN_SPEC>;
#[doc = "PB Weak Pull-up Enable"]
pub mod pbpuen;
#[doc = "PCPUEN register accessor: an alias for `Reg<PCPUEN_SPEC>`"]
pub type PCPUEN = crate::Reg<pcpuen::PCPUEN_SPEC>;
#[doc = "PC Weak Pull-up Enable"]
pub mod pcpuen;
#[doc = "PDPUEN register accessor: an alias for `Reg<PDPUEN_SPEC>`"]
pub type PDPUEN = crate::Reg<pdpuen::PDPUEN_SPEC>;
#[doc = "PD Weak Pull-up Enable"]
pub mod pdpuen;
#[doc = "PEPUEN register accessor: an alias for `Reg<PEPUEN_SPEC>`"]
pub type PEPUEN = crate::Reg<pepuen::PEPUEN_SPEC>;
#[doc = "PE Weak Pull-up Enable"]
pub mod pepuen;
#[doc = "PFPUEN register accessor: an alias for `Reg<PFPUEN_SPEC>`"]
pub type PFPUEN = crate::Reg<pfpuen::PFPUEN_SPEC>;
#[doc = "PF Weak Pull-up Enable"]
pub mod pfpuen;
#[doc = "PGPUEN register accessor: an alias for `Reg<PGPUEN_SPEC>`"]
pub type PGPUEN = crate::Reg<pgpuen::PGPUEN_SPEC>;
#[doc = "PG Weak Pull-up Enable"]
pub mod pgpuen;
#[doc = "PAPDEN register accessor: an alias for `Reg<PAPDEN_SPEC>`"]
pub type PAPDEN = crate::Reg<papden::PAPDEN_SPEC>;
#[doc = "PA Weak Pull-down Enable"]
pub mod papden;
#[doc = "PBPDEN register accessor: an alias for `Reg<PBPDEN_SPEC>`"]
pub type PBPDEN = crate::Reg<pbpden::PBPDEN_SPEC>;
#[doc = "PB Weak Pull-down Enable"]
pub mod pbpden;
#[doc = "PCPDEN register accessor: an alias for `Reg<PCPDEN_SPEC>`"]
pub type PCPDEN = crate::Reg<pcpden::PCPDEN_SPEC>;
#[doc = "PC Weak Pull-down Enable"]
pub mod pcpden;
#[doc = "PDPDEN register accessor: an alias for `Reg<PDPDEN_SPEC>`"]
pub type PDPDEN = crate::Reg<pdpden::PDPDEN_SPEC>;
#[doc = "PD Weak Pull-down Enable"]
pub mod pdpden;
#[doc = "PEPDEN register accessor: an alias for `Reg<PEPDEN_SPEC>`"]
pub type PEPDEN = crate::Reg<pepden::PEPDEN_SPEC>;
#[doc = "PE Weak Pull-down Enable"]
pub mod pepden;
#[doc = "PFPDEN register accessor: an alias for `Reg<PFPDEN_SPEC>`"]
pub type PFPDEN = crate::Reg<pfpden::PFPDEN_SPEC>;
#[doc = "PF Weak Pull-down Enable"]
pub mod pfpden;
#[doc = "PGPDEN register accessor: an alias for `Reg<PGPDEN_SPEC>`"]
pub type PGPDEN = crate::Reg<pgpden::PGPDEN_SPEC>;
#[doc = "PG Weak Pull-down Enable"]
pub mod pgpden;
#[doc = "PADS register accessor: an alias for `Reg<PADS_SPEC>`"]
pub type PADS = crate::Reg<pads::PADS_SPEC>;
#[doc = "PA Drive Strength/Schmitt Trigger"]
pub mod pads;
#[doc = "PBDS register accessor: an alias for `Reg<PBDS_SPEC>`"]
pub type PBDS = crate::Reg<pbds::PBDS_SPEC>;
#[doc = "PB Drive Strength/Schmitt Trigger"]
pub mod pbds;
#[doc = "PCDS register accessor: an alias for `Reg<PCDS_SPEC>`"]
pub type PCDS = crate::Reg<pcds::PCDS_SPEC>;
#[doc = "PC Drive Strength/Schmitt Trigger"]
pub mod pcds;
#[doc = "PDDS register accessor: an alias for `Reg<PDDS_SPEC>`"]
pub type PDDS = crate::Reg<pdds::PDDS_SPEC>;
#[doc = "PD Drive Strength/Schmitt Trigger"]
pub mod pdds;
#[doc = "PEDS register accessor: an alias for `Reg<PEDS_SPEC>`"]
pub type PEDS = crate::Reg<peds::PEDS_SPEC>;
#[doc = "PE Drive Strength/Schmitt Trigger"]
pub mod peds;
#[doc = "PFDS register accessor: an alias for `Reg<PFDS_SPEC>`"]
pub type PFDS = crate::Reg<pfds::PFDS_SPEC>;
#[doc = "PF Drive Strength/Schmitt Trigger"]
pub mod pfds;
#[doc = "PGDS register accessor: an alias for `Reg<PGDS_SPEC>`"]
pub type PGDS = crate::Reg<pgds::PGDS_SPEC>;
#[doc = "PG Drive Strength/Schmitt Trigger"]
pub mod pgds;
