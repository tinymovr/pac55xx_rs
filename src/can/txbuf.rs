#[doc = "Register `TXBUF` reader"]
pub struct R(crate::R<TXBUF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXBUF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXBUF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXBUF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXBUF` writer"]
pub struct W(crate::W<TXBUF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXBUF_SPEC>;
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
impl From<crate::W<TXBUF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXBUF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXBUF0` reader - CAN Transmit Buffer 0"]
pub struct TXBUF0_R(crate::FieldReader<u8, u8>);
impl TXBUF0_R {
    pub(crate) fn new(bits: u8) -> Self {
        TXBUF0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXBUF0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXBUF0` writer - CAN Transmit Buffer 0"]
pub struct TXBUF0_W<'a> {
    w: &'a mut W,
}
impl<'a> TXBUF0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `TXBUF1` reader - CAN Transmit Buffer 1"]
pub struct TXBUF1_R(crate::FieldReader<u8, u8>);
impl TXBUF1_R {
    pub(crate) fn new(bits: u8) -> Self {
        TXBUF1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXBUF1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXBUF1` writer - CAN Transmit Buffer 1"]
pub struct TXBUF1_W<'a> {
    w: &'a mut W,
}
impl<'a> TXBUF1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `TXBUF2` reader - CAN Transmit Buffer 2"]
pub struct TXBUF2_R(crate::FieldReader<u8, u8>);
impl TXBUF2_R {
    pub(crate) fn new(bits: u8) -> Self {
        TXBUF2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXBUF2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXBUF2` writer - CAN Transmit Buffer 2"]
pub struct TXBUF2_W<'a> {
    w: &'a mut W,
}
impl<'a> TXBUF2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `TXBUF3` reader - CAN Transmit Buffer 3"]
pub struct TXBUF3_R(crate::FieldReader<u8, u8>);
impl TXBUF3_R {
    pub(crate) fn new(bits: u8) -> Self {
        TXBUF3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXBUF3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXBUF3` writer - CAN Transmit Buffer 3"]
pub struct TXBUF3_W<'a> {
    w: &'a mut W,
}
impl<'a> TXBUF3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - CAN Transmit Buffer 0"]
    #[inline(always)]
    pub fn txbuf0(&self) -> TXBUF0_R {
        TXBUF0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - CAN Transmit Buffer 1"]
    #[inline(always)]
    pub fn txbuf1(&self) -> TXBUF1_R {
        TXBUF1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - CAN Transmit Buffer 2"]
    #[inline(always)]
    pub fn txbuf2(&self) -> TXBUF2_R {
        TXBUF2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - CAN Transmit Buffer 3"]
    #[inline(always)]
    pub fn txbuf3(&self) -> TXBUF3_R {
        TXBUF3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CAN Transmit Buffer 0"]
    #[inline(always)]
    pub fn txbuf0(&mut self) -> TXBUF0_W {
        TXBUF0_W { w: self }
    }
    #[doc = "Bits 8:15 - CAN Transmit Buffer 1"]
    #[inline(always)]
    pub fn txbuf1(&mut self) -> TXBUF1_W {
        TXBUF1_W { w: self }
    }
    #[doc = "Bits 16:23 - CAN Transmit Buffer 2"]
    #[inline(always)]
    pub fn txbuf2(&mut self) -> TXBUF2_W {
        TXBUF2_W { w: self }
    }
    #[doc = "Bits 24:31 - CAN Transmit Buffer 3"]
    #[inline(always)]
    pub fn txbuf3(&mut self) -> TXBUF3_W {
        TXBUF3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Combined CAN Transmit Buffer registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbuf](index.html) module"]
pub struct TXBUF_SPEC;
impl crate::RegisterSpec for TXBUF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txbuf::R](R) reader structure"]
impl crate::Readable for TXBUF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txbuf::W](W) writer structure"]
impl crate::Writable for TXBUF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXBUF to value 0"]
impl crate::Resettable for TXBUF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
