#[doc = "Register `BTR1_BTR0_RMC_IMR` reader"]
pub struct R(crate::R<BTR1_BTR0_RMC_IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BTR1_BTR0_RMC_IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BTR1_BTR0_RMC_IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BTR1_BTR0_RMC_IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BTR1_BTR0_RMC_IMR` writer"]
pub struct W(crate::W<BTR1_BTR0_RMC_IMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BTR1_BTR0_RMC_IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<BTR1_BTR0_RMC_IMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BTR1_BTR0_RMC_IMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IMR` reader - CAN Interrupt Mask"]
pub struct IMR_R(crate::FieldReader<u8, u8>);
impl IMR_R {
    pub(crate) fn new(bits: u8) -> Self {
        IMR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IMR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IMR` writer - CAN Interrupt Mask"]
pub struct IMR_W<'a> {
    w: &'a mut W,
}
impl<'a> IMR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `RMC` reader - CAN Receive Message Counter"]
pub struct RMC_R(crate::FieldReader<u8, u8>);
impl RMC_R {
    pub(crate) fn new(bits: u8) -> Self {
        RMC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RMC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RMC` writer - CAN Receive Message Counter"]
pub struct RMC_W<'a> {
    w: &'a mut W,
}
impl<'a> RMC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `BTR0` reader - CAN Bus Timing Register 0"]
pub struct BTR0_R(crate::FieldReader<u8, u8>);
impl BTR0_R {
    pub(crate) fn new(bits: u8) -> Self {
        BTR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BTR0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BTR0` writer - CAN Bus Timing Register 0"]
pub struct BTR0_W<'a> {
    w: &'a mut W,
}
impl<'a> BTR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `BTR1` reader - CAN Bus Timing Register 1"]
pub struct BTR1_R(crate::FieldReader<u8, u8>);
impl BTR1_R {
    pub(crate) fn new(bits: u8) -> Self {
        BTR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BTR1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BTR1` writer - CAN Bus Timing Register 1"]
pub struct BTR1_W<'a> {
    w: &'a mut W,
}
impl<'a> BTR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - CAN Interrupt Mask"]
    #[inline(always)]
    pub fn imr(&self) -> IMR_R {
        IMR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - CAN Receive Message Counter"]
    #[inline(always)]
    pub fn rmc(&self) -> RMC_R {
        RMC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - CAN Bus Timing Register 0"]
    #[inline(always)]
    pub fn btr0(&self) -> BTR0_R {
        BTR0_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - CAN Bus Timing Register 1"]
    #[inline(always)]
    pub fn btr1(&self) -> BTR1_R {
        BTR1_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CAN Interrupt Mask"]
    #[inline(always)]
    pub fn imr(&mut self) -> IMR_W {
        IMR_W { w: self }
    }
    #[doc = "Bits 8:15 - CAN Receive Message Counter"]
    #[inline(always)]
    pub fn rmc(&mut self) -> RMC_W {
        RMC_W { w: self }
    }
    #[doc = "Bits 16:23 - CAN Bus Timing Register 0"]
    #[inline(always)]
    pub fn btr0(&mut self) -> BTR0_W {
        BTR0_W { w: self }
    }
    #[doc = "Bits 24:31 - CAN Bus Timing Register 1"]
    #[inline(always)]
    pub fn btr1(&mut self) -> BTR1_W {
        BTR1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Combined CAN IMR, RMC, BTR0, and BTR1 registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [btr1_btr0_rmc_imr](index.html) module"]
pub struct BTR1_BTR0_RMC_IMR_SPEC;
impl crate::RegisterSpec for BTR1_BTR0_RMC_IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [btr1_btr0_rmc_imr::R](R) reader structure"]
impl crate::Readable for BTR1_BTR0_RMC_IMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [btr1_btr0_rmc_imr::W](W) writer structure"]
impl crate::Writable for BTR1_BTR0_RMC_IMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BTR1_BTR0_RMC_IMR to value 0"]
impl crate::Resettable for BTR1_BTR0_RMC_IMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
