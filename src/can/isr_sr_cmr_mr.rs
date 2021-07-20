#[doc = "Register `ISR_SR_CMR_MR` reader"]
pub struct R(crate::R<ISR_SR_CMR_MR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR_SR_CMR_MR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR_SR_CMR_MR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR_SR_CMR_MR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISR_SR_CMR_MR` writer"]
pub struct W(crate::W<ISR_SR_CMR_MR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISR_SR_CMR_MR_SPEC>;
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
impl From<crate::W<ISR_SR_CMR_MR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISR_SR_CMR_MR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MR` reader - CAN Mode"]
pub struct MR_R(crate::FieldReader<u8, u8>);
impl MR_R {
    pub(crate) fn new(bits: u8) -> Self {
        MR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MR` writer - CAN Mode"]
pub struct MR_W<'a> {
    w: &'a mut W,
}
impl<'a> MR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `CMR` reader - CAN Command"]
pub struct CMR_R(crate::FieldReader<u8, u8>);
impl CMR_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMR` writer - CAN Command"]
pub struct CMR_W<'a> {
    w: &'a mut W,
}
impl<'a> CMR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `SR` reader - CAN Status"]
pub struct SR_R(crate::FieldReader<u8, u8>);
impl SR_R {
    pub(crate) fn new(bits: u8) -> Self {
        SR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SR` writer - CAN Status"]
pub struct SR_W<'a> {
    w: &'a mut W,
}
impl<'a> SR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `ISR` reader - CAN Interrupt Status/Acknowledge"]
pub struct ISR_R(crate::FieldReader<u8, u8>);
impl ISR_R {
    pub(crate) fn new(bits: u8) -> Self {
        ISR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISR` writer - CAN Interrupt Status/Acknowledge"]
pub struct ISR_W<'a> {
    w: &'a mut W,
}
impl<'a> ISR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - CAN Mode"]
    #[inline(always)]
    pub fn mr(&self) -> MR_R {
        MR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - CAN Command"]
    #[inline(always)]
    pub fn cmr(&self) -> CMR_R {
        CMR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - CAN Status"]
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - CAN Interrupt Status/Acknowledge"]
    #[inline(always)]
    pub fn isr(&self) -> ISR_R {
        ISR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CAN Mode"]
    #[inline(always)]
    pub fn mr(&mut self) -> MR_W {
        MR_W { w: self }
    }
    #[doc = "Bits 8:15 - CAN Command"]
    #[inline(always)]
    pub fn cmr(&mut self) -> CMR_W {
        CMR_W { w: self }
    }
    #[doc = "Bits 16:23 - CAN Status"]
    #[inline(always)]
    pub fn sr(&mut self) -> SR_W {
        SR_W { w: self }
    }
    #[doc = "Bits 24:31 - CAN Interrupt Status/Acknowledge"]
    #[inline(always)]
    pub fn isr(&mut self) -> ISR_W {
        ISR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Combined CAN MR, CMR, SR, and ISR registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr_sr_cmr_mr](index.html) module"]
pub struct ISR_SR_CMR_MR_SPEC;
impl crate::RegisterSpec for ISR_SR_CMR_MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr_sr_cmr_mr::R](R) reader structure"]
impl crate::Readable for ISR_SR_CMR_MR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [isr_sr_cmr_mr::W](W) writer structure"]
impl crate::Writable for ISR_SR_CMR_MR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ISR_SR_CMR_MR to value 0"]
impl crate::Resettable for ISR_SR_CMR_MR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
