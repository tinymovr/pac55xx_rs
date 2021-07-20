#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC Control"]
    pub ctl: crate::Reg<ctl::CTL_SPEC>,
    #[doc = "0x04 - RTC Time"]
    pub time: crate::Reg<time::TIME_SPEC>,
    #[doc = "0x08 - RTC Date"]
    pub date: crate::Reg<date::DATE_SPEC>,
    #[doc = "0x0c - RTC Time Setting"]
    pub timeset: crate::Reg<timeset::TIMESET_SPEC>,
    #[doc = "0x10 - RTC Date Setting"]
    pub dateset: crate::Reg<dateset::DATESET_SPEC>,
    #[doc = "0x14 - RTC Alarm Setting"]
    pub alarmset: crate::Reg<alarmset::ALARMSET_SPEC>,
}
#[doc = "CTL register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "RTC Control"]
pub mod ctl;
#[doc = "TIME register accessor: an alias for `Reg<TIME_SPEC>`"]
pub type TIME = crate::Reg<time::TIME_SPEC>;
#[doc = "RTC Time"]
pub mod time;
#[doc = "DATE register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "RTC Date"]
pub mod date;
#[doc = "TIMESET register accessor: an alias for `Reg<TIMESET_SPEC>`"]
pub type TIMESET = crate::Reg<timeset::TIMESET_SPEC>;
#[doc = "RTC Time Setting"]
pub mod timeset;
#[doc = "DATESET register accessor: an alias for `Reg<DATESET_SPEC>`"]
pub type DATESET = crate::Reg<dateset::DATESET_SPEC>;
#[doc = "RTC Date Setting"]
pub mod dateset;
#[doc = "ALARMSET register accessor: an alias for `Reg<ALARMSET_SPEC>`"]
pub type ALARMSET = crate::Reg<alarmset::ALARMSET_SPEC>;
#[doc = "RTC Alarm Setting"]
pub mod alarmset;
