#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO C Pin Mode Select"]
    pub mode: crate::Reg<mode::MODE_SPEC>,
    #[doc = "0x04 - GPIO C Data Output Write Mask"]
    pub outmask: crate::Reg<outmask::OUTMASK_SPEC>,
    #[doc = "0x08 - GPIO C Data Output Value"]
    pub out: crate::Reg<out::OUT_SPEC>,
    #[doc = "0x0c - GPIO C Data Input Value"]
    pub in_: crate::Reg<in_::IN_SPEC>,
    #[doc = "0x10 - GPIO C Interrupt Enable"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    #[doc = "0x14 - GPIO C Interrupt Flag Raw"]
    pub intflagraw: crate::Reg<intflagraw::INTFLAGRAW_SPEC>,
    #[doc = "0x18 - GPIO C Interrupt Flag Masked"]
    pub intflagmasked: crate::Reg<intflagmasked::INTFLAGMASKED_SPEC>,
    #[doc = "0x1c - GPIO C Interrupt Clear"]
    pub intclear: crate::Reg<intclear::INTCLEAR_SPEC>,
    #[doc = "0x20 - GPIO C Interrupt Type"]
    pub inttype: crate::Reg<inttype::INTTYPE_SPEC>,
    #[doc = "0x24 - GPIO C Interrupt Value"]
    pub intvalue: crate::Reg<intvalue::INTVALUE_SPEC>,
    #[doc = "0x28 - GPIO C Interrupt Edge Both"]
    pub intedgeboth: crate::Reg<intedgeboth::INTEDGEBOTH_SPEC>,
    #[doc = "0x2c - GPIO C De-bounce Filter"]
    pub debounce: crate::Reg<debounce::DEBOUNCE_SPEC>,
    #[doc = "0x30 - GPIO C Data Output Set"]
    pub doset: crate::Reg<doset::DOSET_SPEC>,
    #[doc = "0x34 - GPIO C Data Output Clear"]
    pub doclear: crate::Reg<doclear::DOCLEAR_SPEC>,
}
#[doc = "MODE register accessor: an alias for `Reg<MODE_SPEC>`"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "GPIO C Pin Mode Select"]
pub mod mode;
#[doc = "OUTMASK register accessor: an alias for `Reg<OUTMASK_SPEC>`"]
pub type OUTMASK = crate::Reg<outmask::OUTMASK_SPEC>;
#[doc = "GPIO C Data Output Write Mask"]
pub mod outmask;
#[doc = "OUT register accessor: an alias for `Reg<OUT_SPEC>`"]
pub type OUT = crate::Reg<out::OUT_SPEC>;
#[doc = "GPIO C Data Output Value"]
pub mod out;
#[doc = "IN register accessor: an alias for `Reg<IN_SPEC>`"]
pub type IN = crate::Reg<in_::IN_SPEC>;
#[doc = "GPIO C Data Input Value"]
pub mod in_;
#[doc = "INTEN register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "GPIO C Interrupt Enable"]
pub mod inten;
#[doc = "INTFLAGRAW register accessor: an alias for `Reg<INTFLAGRAW_SPEC>`"]
pub type INTFLAGRAW = crate::Reg<intflagraw::INTFLAGRAW_SPEC>;
#[doc = "GPIO C Interrupt Flag Raw"]
pub mod intflagraw;
#[doc = "INTFLAGMASKED register accessor: an alias for `Reg<INTFLAGMASKED_SPEC>`"]
pub type INTFLAGMASKED = crate::Reg<intflagmasked::INTFLAGMASKED_SPEC>;
#[doc = "GPIO C Interrupt Flag Masked"]
pub mod intflagmasked;
#[doc = "INTCLEAR register accessor: an alias for `Reg<INTCLEAR_SPEC>`"]
pub type INTCLEAR = crate::Reg<intclear::INTCLEAR_SPEC>;
#[doc = "GPIO C Interrupt Clear"]
pub mod intclear;
#[doc = "INTTYPE register accessor: an alias for `Reg<INTTYPE_SPEC>`"]
pub type INTTYPE = crate::Reg<inttype::INTTYPE_SPEC>;
#[doc = "GPIO C Interrupt Type"]
pub mod inttype;
#[doc = "INTVALUE register accessor: an alias for `Reg<INTVALUE_SPEC>`"]
pub type INTVALUE = crate::Reg<intvalue::INTVALUE_SPEC>;
#[doc = "GPIO C Interrupt Value"]
pub mod intvalue;
#[doc = "INTEDGEBOTH register accessor: an alias for `Reg<INTEDGEBOTH_SPEC>`"]
pub type INTEDGEBOTH = crate::Reg<intedgeboth::INTEDGEBOTH_SPEC>;
#[doc = "GPIO C Interrupt Edge Both"]
pub mod intedgeboth;
#[doc = "DEBOUNCE register accessor: an alias for `Reg<DEBOUNCE_SPEC>`"]
pub type DEBOUNCE = crate::Reg<debounce::DEBOUNCE_SPEC>;
#[doc = "GPIO C De-bounce Filter"]
pub mod debounce;
#[doc = "DOSET register accessor: an alias for `Reg<DOSET_SPEC>`"]
pub type DOSET = crate::Reg<doset::DOSET_SPEC>;
#[doc = "GPIO C Data Output Set"]
pub mod doset;
#[doc = "DOCLEAR register accessor: an alias for `Reg<DOCLEAR_SPEC>`"]
pub type DOCLEAR = crate::Reg<doclear::DOCLEAR_SPEC>;
#[doc = "GPIO C Data Output Clear"]
pub mod doclear;
