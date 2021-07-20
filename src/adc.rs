#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EMUX Control"]
    pub emuxctl: crate::Reg<emuxctl::EMUXCTL_SPEC>,
    #[doc = "0x04 - EMUX Data"]
    pub emuxdata: crate::Reg<emuxdata::EMUXDATA_SPEC>,
    #[doc = "0x08 - ADC Control"]
    pub adcctl: crate::Reg<adcctl::ADCCTL_SPEC>,
    #[doc = "0x0c - ADC Result"]
    pub adcres: crate::Reg<adcres::ADCRES_SPEC>,
    #[doc = "0x10 - ADC Interrupt Control"]
    pub adcint: crate::Reg<adcint::ADCINT_SPEC>,
    _reserved5: [u8; 0x2c],
    #[doc = "0x40 - DTSE Trigger Entry 0 to 3"]
    pub dtsetrigent0to3: crate::Reg<dtsetrigent0to3::DTSETRIGENT0TO3_SPEC>,
    #[doc = "0x44 - DTSE Trigger Entry 4 to 7"]
    pub dtsetrigent4to7: crate::Reg<dtsetrigent4to7::DTSETRIGENT4TO7_SPEC>,
    #[doc = "0x48 - DTSE Trigger Entry 8 to 11"]
    pub dtsetrigent8to11: crate::Reg<dtsetrigent8to11::DTSETRIGENT8TO11_SPEC>,
    #[doc = "0x4c - DTSE Trigger Entry 12 to 15"]
    pub dtsetrigent12to15: crate::Reg<dtsetrigent12to15::DTSETRIGENT12TO15_SPEC>,
    #[doc = "0x50 - DTSE Trigger Entry 16 to 19"]
    pub dtsetrigent16to19: crate::Reg<dtsetrigent16to19::DTSETRIGENT16TO19_SPEC>,
    #[doc = "0x54 - DTSE Trigger Entry 20 to 23"]
    pub dtsetrigent20to23: crate::Reg<dtsetrigent20to23::DTSETRIGENT20TO23_SPEC>,
    #[doc = "0x58 - DTSE Trigger Entry 24 to 27"]
    pub dtsetrigent24to27: crate::Reg<dtsetrigent24to27::DTSETRIGENT24TO27_SPEC>,
    #[doc = "0x5c - DTSE Trigger Entry 28 to 31"]
    pub dtsetrigent28to31: crate::Reg<dtsetrigent28to31::DTSETRIGENT28TO31_SPEC>,
    _reserved13: [u8; 0x20],
    #[doc = "0x80 - DTSE Sequence Config 0"]
    pub dtseseqcfg0: crate::Reg<dtseseqcfg0::DTSESEQCFG0_SPEC>,
    #[doc = "0x84 - DTSE Sequence Config 1"]
    pub dtseseqcfg1: crate::Reg<dtseseqcfg1::DTSESEQCFG1_SPEC>,
    #[doc = "0x88 - DTSE Sequence Config 2"]
    pub dtseseqcfg2: crate::Reg<dtseseqcfg2::DTSESEQCFG2_SPEC>,
    #[doc = "0x8c - DTSE Sequence Config 3"]
    pub dtseseqcfg3: crate::Reg<dtseseqcfg3::DTSESEQCFG3_SPEC>,
    #[doc = "0x90 - DTSE Sequence Config 4"]
    pub dtseseqcfg4: crate::Reg<dtseseqcfg4::DTSESEQCFG4_SPEC>,
    #[doc = "0x94 - DTSE Sequence Config 5"]
    pub dtseseqcfg5: crate::Reg<dtseseqcfg5::DTSESEQCFG5_SPEC>,
    #[doc = "0x98 - DTSE Sequence Config 6"]
    pub dtseseqcfg6: crate::Reg<dtseseqcfg6::DTSESEQCFG6_SPEC>,
    #[doc = "0x9c - DTSE Sequence Config 7"]
    pub dtseseqcfg7: crate::Reg<dtseseqcfg7::DTSESEQCFG7_SPEC>,
    _reserved21: [u8; 0x60],
    #[doc = "0x100 - DTSE Sequence Config 8"]
    pub dtseseqcfg8: crate::Reg<dtseseqcfg8::DTSESEQCFG8_SPEC>,
    #[doc = "0x104 - DTSE Sequence Config 9"]
    pub dtseseqcfg9: crate::Reg<dtseseqcfg9::DTSESEQCFG9_SPEC>,
    #[doc = "0x108 - DTSE Sequence Config 10"]
    pub dtseseqcfg10: crate::Reg<dtseseqcfg10::DTSESEQCFG10_SPEC>,
    #[doc = "0x10c - DTSE Sequence Config 11"]
    pub dtseseqcfg11: crate::Reg<dtseseqcfg11::DTSESEQCFG11_SPEC>,
    #[doc = "0x110 - DTSE Sequence Config 12"]
    pub dtseseqcfg12: crate::Reg<dtseseqcfg12::DTSESEQCFG12_SPEC>,
    #[doc = "0x114 - DTSE Sequence Config 13"]
    pub dtseseqcfg13: crate::Reg<dtseseqcfg13::DTSESEQCFG13_SPEC>,
    #[doc = "0x118 - DTSE Sequence Config 14"]
    pub dtseseqcfg14: crate::Reg<dtseseqcfg14::DTSESEQCFG14_SPEC>,
    #[doc = "0x11c - DTSE Sequence Config 15"]
    pub dtseseqcfg15: crate::Reg<dtseseqcfg15::DTSESEQCFG15_SPEC>,
    #[doc = "0x120 - DTSE Sequence Config 16"]
    pub dtseseqcfg16: crate::Reg<dtseseqcfg16::DTSESEQCFG16_SPEC>,
    #[doc = "0x124 - DTSE Sequence Config 17"]
    pub dtseseqcfg17: crate::Reg<dtseseqcfg17::DTSESEQCFG17_SPEC>,
    #[doc = "0x128 - DTSE Sequence Config 18"]
    pub dtseseqcfg18: crate::Reg<dtseseqcfg18::DTSESEQCFG18_SPEC>,
    #[doc = "0x12c - DTSE Sequence Config 19"]
    pub dtseseqcfg19: crate::Reg<dtseseqcfg19::DTSESEQCFG19_SPEC>,
    #[doc = "0x130 - DTSE Sequence Config 20"]
    pub dtseseqcfg20: crate::Reg<dtseseqcfg20::DTSESEQCFG20_SPEC>,
    #[doc = "0x134 - DTSE Sequence Config 21"]
    pub dtseseqcfg21: crate::Reg<dtseseqcfg21::DTSESEQCFG21_SPEC>,
    #[doc = "0x138 - DTSE Sequence Config 22"]
    pub dtseseqcfg22: crate::Reg<dtseseqcfg22::DTSESEQCFG22_SPEC>,
    #[doc = "0x13c - DTSE Sequence Config 23"]
    pub dtseseqcfg23: crate::Reg<dtseseqcfg23::DTSESEQCFG23_SPEC>,
    _reserved37: [u8; 0xc0],
    #[doc = "0x200 - DTSE Result 0"]
    pub dtseres0: crate::Reg<dtseres0::DTSERES0_SPEC>,
    #[doc = "0x204 - DTSE Result 1"]
    pub dtseres1: crate::Reg<dtseres1::DTSERES1_SPEC>,
    #[doc = "0x208 - DTSE Result 2"]
    pub dtseres2: crate::Reg<dtseres2::DTSERES2_SPEC>,
    #[doc = "0x20c - DTSE Result 3"]
    pub dtseres3: crate::Reg<dtseres3::DTSERES3_SPEC>,
    #[doc = "0x210 - DTSE Result 4"]
    pub dtseres4: crate::Reg<dtseres4::DTSERES4_SPEC>,
    #[doc = "0x214 - DTSE Result 5"]
    pub dtseres5: crate::Reg<dtseres5::DTSERES5_SPEC>,
    #[doc = "0x218 - DTSE Result 6"]
    pub dtseres6: crate::Reg<dtseres6::DTSERES6_SPEC>,
    #[doc = "0x21c - DTSE Result 7"]
    pub dtseres7: crate::Reg<dtseres7::DTSERES7_SPEC>,
    #[doc = "0x220 - DTSE Result 8"]
    pub dtseres8: crate::Reg<dtseres8::DTSERES8_SPEC>,
    #[doc = "0x224 - DTSE Result 9"]
    pub dtseres9: crate::Reg<dtseres9::DTSERES9_SPEC>,
    #[doc = "0x228 - DTSE Result 10"]
    pub dtseres10: crate::Reg<dtseres10::DTSERES10_SPEC>,
    #[doc = "0x22c - DTSE Result 11"]
    pub dtseres11: crate::Reg<dtseres11::DTSERES11_SPEC>,
    #[doc = "0x230 - DTSE Result 12"]
    pub dtseres12: crate::Reg<dtseres12::DTSERES12_SPEC>,
    #[doc = "0x234 - DTSE Result 13"]
    pub dtseres13: crate::Reg<dtseres13::DTSERES13_SPEC>,
    #[doc = "0x238 - DTSE Result 14"]
    pub dtseres14: crate::Reg<dtseres14::DTSERES14_SPEC>,
    #[doc = "0x23c - DTSE Result 15"]
    pub dtseres15: crate::Reg<dtseres15::DTSERES15_SPEC>,
    #[doc = "0x240 - DTSE Result 16"]
    pub dtseres16: crate::Reg<dtseres16::DTSERES16_SPEC>,
    #[doc = "0x244 - DTSE Result 17"]
    pub dtseres17: crate::Reg<dtseres17::DTSERES17_SPEC>,
    #[doc = "0x248 - DTSE Result 18"]
    pub dtseres18: crate::Reg<dtseres18::DTSERES18_SPEC>,
    #[doc = "0x24c - DTSE Result 19"]
    pub dtseres19: crate::Reg<dtseres19::DTSERES19_SPEC>,
    #[doc = "0x250 - DTSE Result 20"]
    pub dtseres20: crate::Reg<dtseres20::DTSERES20_SPEC>,
    #[doc = "0x254 - DTSE Result 21"]
    pub dtseres21: crate::Reg<dtseres21::DTSERES21_SPEC>,
    #[doc = "0x258 - DTSE Result 22"]
    pub dtseres22: crate::Reg<dtseres22::DTSERES22_SPEC>,
    #[doc = "0x25c - DTSE Result 23"]
    pub dtseres23: crate::Reg<dtseres23::DTSERES23_SPEC>,
}
#[doc = "EMUXCTL register accessor: an alias for `Reg<EMUXCTL_SPEC>`"]
pub type EMUXCTL = crate::Reg<emuxctl::EMUXCTL_SPEC>;
#[doc = "EMUX Control"]
pub mod emuxctl;
#[doc = "EMUXDATA register accessor: an alias for `Reg<EMUXDATA_SPEC>`"]
pub type EMUXDATA = crate::Reg<emuxdata::EMUXDATA_SPEC>;
#[doc = "EMUX Data"]
pub mod emuxdata;
#[doc = "ADCCTL register accessor: an alias for `Reg<ADCCTL_SPEC>`"]
pub type ADCCTL = crate::Reg<adcctl::ADCCTL_SPEC>;
#[doc = "ADC Control"]
pub mod adcctl;
#[doc = "ADCRES register accessor: an alias for `Reg<ADCRES_SPEC>`"]
pub type ADCRES = crate::Reg<adcres::ADCRES_SPEC>;
#[doc = "ADC Result"]
pub mod adcres;
#[doc = "ADCINT register accessor: an alias for `Reg<ADCINT_SPEC>`"]
pub type ADCINT = crate::Reg<adcint::ADCINT_SPEC>;
#[doc = "ADC Interrupt Control"]
pub mod adcint;
#[doc = "DTSETRIGENT0TO3 register accessor: an alias for `Reg<DTSETRIGENT0TO3_SPEC>`"]
pub type DTSETRIGENT0TO3 = crate::Reg<dtsetrigent0to3::DTSETRIGENT0TO3_SPEC>;
#[doc = "DTSE Trigger Entry 0 to 3"]
pub mod dtsetrigent0to3;
#[doc = "DTSETRIGENT4TO7 register accessor: an alias for `Reg<DTSETRIGENT4TO7_SPEC>`"]
pub type DTSETRIGENT4TO7 = crate::Reg<dtsetrigent4to7::DTSETRIGENT4TO7_SPEC>;
#[doc = "DTSE Trigger Entry 4 to 7"]
pub mod dtsetrigent4to7;
#[doc = "DTSETRIGENT8TO11 register accessor: an alias for `Reg<DTSETRIGENT8TO11_SPEC>`"]
pub type DTSETRIGENT8TO11 = crate::Reg<dtsetrigent8to11::DTSETRIGENT8TO11_SPEC>;
#[doc = "DTSE Trigger Entry 8 to 11"]
pub mod dtsetrigent8to11;
#[doc = "DTSETRIGENT12TO15 register accessor: an alias for `Reg<DTSETRIGENT12TO15_SPEC>`"]
pub type DTSETRIGENT12TO15 = crate::Reg<dtsetrigent12to15::DTSETRIGENT12TO15_SPEC>;
#[doc = "DTSE Trigger Entry 12 to 15"]
pub mod dtsetrigent12to15;
#[doc = "DTSETRIGENT16TO19 register accessor: an alias for `Reg<DTSETRIGENT16TO19_SPEC>`"]
pub type DTSETRIGENT16TO19 = crate::Reg<dtsetrigent16to19::DTSETRIGENT16TO19_SPEC>;
#[doc = "DTSE Trigger Entry 16 to 19"]
pub mod dtsetrigent16to19;
#[doc = "DTSETRIGENT20TO23 register accessor: an alias for `Reg<DTSETRIGENT20TO23_SPEC>`"]
pub type DTSETRIGENT20TO23 = crate::Reg<dtsetrigent20to23::DTSETRIGENT20TO23_SPEC>;
#[doc = "DTSE Trigger Entry 20 to 23"]
pub mod dtsetrigent20to23;
#[doc = "DTSETRIGENT24TO27 register accessor: an alias for `Reg<DTSETRIGENT24TO27_SPEC>`"]
pub type DTSETRIGENT24TO27 = crate::Reg<dtsetrigent24to27::DTSETRIGENT24TO27_SPEC>;
#[doc = "DTSE Trigger Entry 24 to 27"]
pub mod dtsetrigent24to27;
#[doc = "DTSETRIGENT28TO31 register accessor: an alias for `Reg<DTSETRIGENT28TO31_SPEC>`"]
pub type DTSETRIGENT28TO31 = crate::Reg<dtsetrigent28to31::DTSETRIGENT28TO31_SPEC>;
#[doc = "DTSE Trigger Entry 28 to 31"]
pub mod dtsetrigent28to31;
#[doc = "DTSESEQCFG0 register accessor: an alias for `Reg<DTSESEQCFG0_SPEC>`"]
pub type DTSESEQCFG0 = crate::Reg<dtseseqcfg0::DTSESEQCFG0_SPEC>;
#[doc = "DTSE Sequence Config 0"]
pub mod dtseseqcfg0;
#[doc = "DTSESEQCFG1 register accessor: an alias for `Reg<DTSESEQCFG1_SPEC>`"]
pub type DTSESEQCFG1 = crate::Reg<dtseseqcfg1::DTSESEQCFG1_SPEC>;
#[doc = "DTSE Sequence Config 1"]
pub mod dtseseqcfg1;
#[doc = "DTSESEQCFG2 register accessor: an alias for `Reg<DTSESEQCFG2_SPEC>`"]
pub type DTSESEQCFG2 = crate::Reg<dtseseqcfg2::DTSESEQCFG2_SPEC>;
#[doc = "DTSE Sequence Config 2"]
pub mod dtseseqcfg2;
#[doc = "DTSESEQCFG3 register accessor: an alias for `Reg<DTSESEQCFG3_SPEC>`"]
pub type DTSESEQCFG3 = crate::Reg<dtseseqcfg3::DTSESEQCFG3_SPEC>;
#[doc = "DTSE Sequence Config 3"]
pub mod dtseseqcfg3;
#[doc = "DTSESEQCFG4 register accessor: an alias for `Reg<DTSESEQCFG4_SPEC>`"]
pub type DTSESEQCFG4 = crate::Reg<dtseseqcfg4::DTSESEQCFG4_SPEC>;
#[doc = "DTSE Sequence Config 4"]
pub mod dtseseqcfg4;
#[doc = "DTSESEQCFG5 register accessor: an alias for `Reg<DTSESEQCFG5_SPEC>`"]
pub type DTSESEQCFG5 = crate::Reg<dtseseqcfg5::DTSESEQCFG5_SPEC>;
#[doc = "DTSE Sequence Config 5"]
pub mod dtseseqcfg5;
#[doc = "DTSESEQCFG6 register accessor: an alias for `Reg<DTSESEQCFG6_SPEC>`"]
pub type DTSESEQCFG6 = crate::Reg<dtseseqcfg6::DTSESEQCFG6_SPEC>;
#[doc = "DTSE Sequence Config 6"]
pub mod dtseseqcfg6;
#[doc = "DTSESEQCFG7 register accessor: an alias for `Reg<DTSESEQCFG7_SPEC>`"]
pub type DTSESEQCFG7 = crate::Reg<dtseseqcfg7::DTSESEQCFG7_SPEC>;
#[doc = "DTSE Sequence Config 7"]
pub mod dtseseqcfg7;
#[doc = "DTSESEQCFG8 register accessor: an alias for `Reg<DTSESEQCFG8_SPEC>`"]
pub type DTSESEQCFG8 = crate::Reg<dtseseqcfg8::DTSESEQCFG8_SPEC>;
#[doc = "DTSE Sequence Config 8"]
pub mod dtseseqcfg8;
#[doc = "DTSESEQCFG9 register accessor: an alias for `Reg<DTSESEQCFG9_SPEC>`"]
pub type DTSESEQCFG9 = crate::Reg<dtseseqcfg9::DTSESEQCFG9_SPEC>;
#[doc = "DTSE Sequence Config 9"]
pub mod dtseseqcfg9;
#[doc = "DTSESEQCFG10 register accessor: an alias for `Reg<DTSESEQCFG10_SPEC>`"]
pub type DTSESEQCFG10 = crate::Reg<dtseseqcfg10::DTSESEQCFG10_SPEC>;
#[doc = "DTSE Sequence Config 10"]
pub mod dtseseqcfg10;
#[doc = "DTSESEQCFG11 register accessor: an alias for `Reg<DTSESEQCFG11_SPEC>`"]
pub type DTSESEQCFG11 = crate::Reg<dtseseqcfg11::DTSESEQCFG11_SPEC>;
#[doc = "DTSE Sequence Config 11"]
pub mod dtseseqcfg11;
#[doc = "DTSESEQCFG12 register accessor: an alias for `Reg<DTSESEQCFG12_SPEC>`"]
pub type DTSESEQCFG12 = crate::Reg<dtseseqcfg12::DTSESEQCFG12_SPEC>;
#[doc = "DTSE Sequence Config 12"]
pub mod dtseseqcfg12;
#[doc = "DTSESEQCFG13 register accessor: an alias for `Reg<DTSESEQCFG13_SPEC>`"]
pub type DTSESEQCFG13 = crate::Reg<dtseseqcfg13::DTSESEQCFG13_SPEC>;
#[doc = "DTSE Sequence Config 13"]
pub mod dtseseqcfg13;
#[doc = "DTSESEQCFG14 register accessor: an alias for `Reg<DTSESEQCFG14_SPEC>`"]
pub type DTSESEQCFG14 = crate::Reg<dtseseqcfg14::DTSESEQCFG14_SPEC>;
#[doc = "DTSE Sequence Config 14"]
pub mod dtseseqcfg14;
#[doc = "DTSESEQCFG15 register accessor: an alias for `Reg<DTSESEQCFG15_SPEC>`"]
pub type DTSESEQCFG15 = crate::Reg<dtseseqcfg15::DTSESEQCFG15_SPEC>;
#[doc = "DTSE Sequence Config 15"]
pub mod dtseseqcfg15;
#[doc = "DTSESEQCFG16 register accessor: an alias for `Reg<DTSESEQCFG16_SPEC>`"]
pub type DTSESEQCFG16 = crate::Reg<dtseseqcfg16::DTSESEQCFG16_SPEC>;
#[doc = "DTSE Sequence Config 16"]
pub mod dtseseqcfg16;
#[doc = "DTSESEQCFG17 register accessor: an alias for `Reg<DTSESEQCFG17_SPEC>`"]
pub type DTSESEQCFG17 = crate::Reg<dtseseqcfg17::DTSESEQCFG17_SPEC>;
#[doc = "DTSE Sequence Config 17"]
pub mod dtseseqcfg17;
#[doc = "DTSESEQCFG18 register accessor: an alias for `Reg<DTSESEQCFG18_SPEC>`"]
pub type DTSESEQCFG18 = crate::Reg<dtseseqcfg18::DTSESEQCFG18_SPEC>;
#[doc = "DTSE Sequence Config 18"]
pub mod dtseseqcfg18;
#[doc = "DTSESEQCFG19 register accessor: an alias for `Reg<DTSESEQCFG19_SPEC>`"]
pub type DTSESEQCFG19 = crate::Reg<dtseseqcfg19::DTSESEQCFG19_SPEC>;
#[doc = "DTSE Sequence Config 19"]
pub mod dtseseqcfg19;
#[doc = "DTSESEQCFG20 register accessor: an alias for `Reg<DTSESEQCFG20_SPEC>`"]
pub type DTSESEQCFG20 = crate::Reg<dtseseqcfg20::DTSESEQCFG20_SPEC>;
#[doc = "DTSE Sequence Config 20"]
pub mod dtseseqcfg20;
#[doc = "DTSESEQCFG21 register accessor: an alias for `Reg<DTSESEQCFG21_SPEC>`"]
pub type DTSESEQCFG21 = crate::Reg<dtseseqcfg21::DTSESEQCFG21_SPEC>;
#[doc = "DTSE Sequence Config 21"]
pub mod dtseseqcfg21;
#[doc = "DTSESEQCFG22 register accessor: an alias for `Reg<DTSESEQCFG22_SPEC>`"]
pub type DTSESEQCFG22 = crate::Reg<dtseseqcfg22::DTSESEQCFG22_SPEC>;
#[doc = "DTSE Sequence Config 22"]
pub mod dtseseqcfg22;
#[doc = "DTSESEQCFG23 register accessor: an alias for `Reg<DTSESEQCFG23_SPEC>`"]
pub type DTSESEQCFG23 = crate::Reg<dtseseqcfg23::DTSESEQCFG23_SPEC>;
#[doc = "DTSE Sequence Config 23"]
pub mod dtseseqcfg23;
#[doc = "DTSERES0 register accessor: an alias for `Reg<DTSERES0_SPEC>`"]
pub type DTSERES0 = crate::Reg<dtseres0::DTSERES0_SPEC>;
#[doc = "DTSE Result 0"]
pub mod dtseres0;
#[doc = "DTSERES1 register accessor: an alias for `Reg<DTSERES1_SPEC>`"]
pub type DTSERES1 = crate::Reg<dtseres1::DTSERES1_SPEC>;
#[doc = "DTSE Result 1"]
pub mod dtseres1;
#[doc = "DTSERES2 register accessor: an alias for `Reg<DTSERES2_SPEC>`"]
pub type DTSERES2 = crate::Reg<dtseres2::DTSERES2_SPEC>;
#[doc = "DTSE Result 2"]
pub mod dtseres2;
#[doc = "DTSERES3 register accessor: an alias for `Reg<DTSERES3_SPEC>`"]
pub type DTSERES3 = crate::Reg<dtseres3::DTSERES3_SPEC>;
#[doc = "DTSE Result 3"]
pub mod dtseres3;
#[doc = "DTSERES4 register accessor: an alias for `Reg<DTSERES4_SPEC>`"]
pub type DTSERES4 = crate::Reg<dtseres4::DTSERES4_SPEC>;
#[doc = "DTSE Result 4"]
pub mod dtseres4;
#[doc = "DTSERES5 register accessor: an alias for `Reg<DTSERES5_SPEC>`"]
pub type DTSERES5 = crate::Reg<dtseres5::DTSERES5_SPEC>;
#[doc = "DTSE Result 5"]
pub mod dtseres5;
#[doc = "DTSERES6 register accessor: an alias for `Reg<DTSERES6_SPEC>`"]
pub type DTSERES6 = crate::Reg<dtseres6::DTSERES6_SPEC>;
#[doc = "DTSE Result 6"]
pub mod dtseres6;
#[doc = "DTSERES7 register accessor: an alias for `Reg<DTSERES7_SPEC>`"]
pub type DTSERES7 = crate::Reg<dtseres7::DTSERES7_SPEC>;
#[doc = "DTSE Result 7"]
pub mod dtseres7;
#[doc = "DTSERES8 register accessor: an alias for `Reg<DTSERES8_SPEC>`"]
pub type DTSERES8 = crate::Reg<dtseres8::DTSERES8_SPEC>;
#[doc = "DTSE Result 8"]
pub mod dtseres8;
#[doc = "DTSERES9 register accessor: an alias for `Reg<DTSERES9_SPEC>`"]
pub type DTSERES9 = crate::Reg<dtseres9::DTSERES9_SPEC>;
#[doc = "DTSE Result 9"]
pub mod dtseres9;
#[doc = "DTSERES10 register accessor: an alias for `Reg<DTSERES10_SPEC>`"]
pub type DTSERES10 = crate::Reg<dtseres10::DTSERES10_SPEC>;
#[doc = "DTSE Result 10"]
pub mod dtseres10;
#[doc = "DTSERES11 register accessor: an alias for `Reg<DTSERES11_SPEC>`"]
pub type DTSERES11 = crate::Reg<dtseres11::DTSERES11_SPEC>;
#[doc = "DTSE Result 11"]
pub mod dtseres11;
#[doc = "DTSERES12 register accessor: an alias for `Reg<DTSERES12_SPEC>`"]
pub type DTSERES12 = crate::Reg<dtseres12::DTSERES12_SPEC>;
#[doc = "DTSE Result 12"]
pub mod dtseres12;
#[doc = "DTSERES13 register accessor: an alias for `Reg<DTSERES13_SPEC>`"]
pub type DTSERES13 = crate::Reg<dtseres13::DTSERES13_SPEC>;
#[doc = "DTSE Result 13"]
pub mod dtseres13;
#[doc = "DTSERES14 register accessor: an alias for `Reg<DTSERES14_SPEC>`"]
pub type DTSERES14 = crate::Reg<dtseres14::DTSERES14_SPEC>;
#[doc = "DTSE Result 14"]
pub mod dtseres14;
#[doc = "DTSERES15 register accessor: an alias for `Reg<DTSERES15_SPEC>`"]
pub type DTSERES15 = crate::Reg<dtseres15::DTSERES15_SPEC>;
#[doc = "DTSE Result 15"]
pub mod dtseres15;
#[doc = "DTSERES16 register accessor: an alias for `Reg<DTSERES16_SPEC>`"]
pub type DTSERES16 = crate::Reg<dtseres16::DTSERES16_SPEC>;
#[doc = "DTSE Result 16"]
pub mod dtseres16;
#[doc = "DTSERES17 register accessor: an alias for `Reg<DTSERES17_SPEC>`"]
pub type DTSERES17 = crate::Reg<dtseres17::DTSERES17_SPEC>;
#[doc = "DTSE Result 17"]
pub mod dtseres17;
#[doc = "DTSERES18 register accessor: an alias for `Reg<DTSERES18_SPEC>`"]
pub type DTSERES18 = crate::Reg<dtseres18::DTSERES18_SPEC>;
#[doc = "DTSE Result 18"]
pub mod dtseres18;
#[doc = "DTSERES19 register accessor: an alias for `Reg<DTSERES19_SPEC>`"]
pub type DTSERES19 = crate::Reg<dtseres19::DTSERES19_SPEC>;
#[doc = "DTSE Result 19"]
pub mod dtseres19;
#[doc = "DTSERES20 register accessor: an alias for `Reg<DTSERES20_SPEC>`"]
pub type DTSERES20 = crate::Reg<dtseres20::DTSERES20_SPEC>;
#[doc = "DTSE Result 20"]
pub mod dtseres20;
#[doc = "DTSERES21 register accessor: an alias for `Reg<DTSERES21_SPEC>`"]
pub type DTSERES21 = crate::Reg<dtseres21::DTSERES21_SPEC>;
#[doc = "DTSE Result 21"]
pub mod dtseres21;
#[doc = "DTSERES22 register accessor: an alias for `Reg<DTSERES22_SPEC>`"]
pub type DTSERES22 = crate::Reg<dtseres22::DTSERES22_SPEC>;
#[doc = "DTSE Result 22"]
pub mod dtseres22;
#[doc = "DTSERES23 register accessor: an alias for `Reg<DTSERES23_SPEC>`"]
pub type DTSERES23 = crate::Reg<dtseres23::DTSERES23_SPEC>;
#[doc = "DTSE Result 23"]
pub mod dtseres23;
