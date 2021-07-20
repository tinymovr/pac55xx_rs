#[doc = "Register `ACR` reader"]
pub struct R(crate::R<ACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACR` writer"]
pub struct W(crate::W<ACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACR_SPEC>;
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
impl From<crate::W<ACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACR0` reader - CAN Acceptance Code 0"]
pub struct ACR0_R(crate::FieldReader<u8, u8>);
impl ACR0_R {
    pub(crate) fn new(bits: u8) -> Self {
        ACR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACR0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACR0` writer - CAN Acceptance Code 0"]
pub struct ACR0_W<'a> {
    w: &'a mut W,
}
impl<'a> ACR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `ACR1` reader - CAN Acceptance Code 1"]
pub struct ACR1_R(crate::FieldReader<u8, u8>);
impl ACR1_R {
    pub(crate) fn new(bits: u8) -> Self {
        ACR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACR1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACR1` writer - CAN Acceptance Code 1"]
pub struct ACR1_W<'a> {
    w: &'a mut W,
}
impl<'a> ACR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `ACR2` reader - CAN Acceptance Code 2"]
pub struct ACR2_R(crate::FieldReader<u8, u8>);
impl ACR2_R {
    pub(crate) fn new(bits: u8) -> Self {
        ACR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACR2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACR2` writer - CAN Acceptance Code 2"]
pub struct ACR2_W<'a> {
    w: &'a mut W,
}
impl<'a> ACR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `ACR3` reader - CAN Acceptance Code 3"]
pub struct ACR3_R(crate::FieldReader<u8, u8>);
impl ACR3_R {
    pub(crate) fn new(bits: u8) -> Self {
        ACR3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACR3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACR3` writer - CAN Acceptance Code 3"]
pub struct ACR3_W<'a> {
    w: &'a mut W,
}
impl<'a> ACR3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - CAN Acceptance Code 0"]
    #[inline(always)]
    pub fn acr0(&self) -> ACR0_R {
        ACR0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - CAN Acceptance Code 1"]
    #[inline(always)]
    pub fn acr1(&self) -> ACR1_R {
        ACR1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - CAN Acceptance Code 2"]
    #[inline(always)]
    pub fn acr2(&self) -> ACR2_R {
        ACR2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - CAN Acceptance Code 3"]
    #[inline(always)]
    pub fn acr3(&self) -> ACR3_R {
        ACR3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CAN Acceptance Code 0"]
    #[inline(always)]
    pub fn acr0(&mut self) -> ACR0_W {
        ACR0_W { w: self }
    }
    #[doc = "Bits 8:15 - CAN Acceptance Code 1"]
    #[inline(always)]
    pub fn acr1(&mut self) -> ACR1_W {
        ACR1_W { w: self }
    }
    #[doc = "Bits 16:23 - CAN Acceptance Code 2"]
    #[inline(always)]
    pub fn acr2(&mut self) -> ACR2_W {
        ACR2_W { w: self }
    }
    #[doc = "Bits 24:31 - CAN Acceptance Code 3"]
    #[inline(always)]
    pub fn acr3(&mut self) -> ACR3_W {
        ACR3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Combined CAN Acceptance Code registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acr](index.html) module"]
pub struct ACR_SPEC;
impl crate::RegisterSpec for ACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acr::R](R) reader structure"]
impl crate::Readable for ACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acr::W](W) writer structure"]
impl crate::Writable for ACR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ACR to value 0"]
impl crate::Resettable for ACR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
