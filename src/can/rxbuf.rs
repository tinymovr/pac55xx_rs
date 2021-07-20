#[doc = "Register `RXBUF` reader"]
pub struct R(crate::R<RXBUF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXBUF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXBUF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXBUF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXBUF` writer"]
pub struct W(crate::W<RXBUF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXBUF_SPEC>;
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
impl From<crate::W<RXBUF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXBUF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXBUF0` reader - CAN Receive Buffer 0"]
pub struct RXBUF0_R(crate::FieldReader<u8, u8>);
impl RXBUF0_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXBUF0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXBUF0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXBUF0` writer - CAN Receive Buffer 0"]
pub struct RXBUF0_W<'a> {
    w: &'a mut W,
}
impl<'a> RXBUF0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `RXBUF1` reader - CAN Receive Buffer 1"]
pub struct RXBUF1_R(crate::FieldReader<u8, u8>);
impl RXBUF1_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXBUF1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXBUF1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXBUF1` writer - CAN Receive Buffer 1"]
pub struct RXBUF1_W<'a> {
    w: &'a mut W,
}
impl<'a> RXBUF1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `RXBUF2` reader - CAN Receive Buffer 2"]
pub struct RXBUF2_R(crate::FieldReader<u8, u8>);
impl RXBUF2_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXBUF2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXBUF2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXBUF2` writer - CAN Receive Buffer 2"]
pub struct RXBUF2_W<'a> {
    w: &'a mut W,
}
impl<'a> RXBUF2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `RXBUF3` reader - CAN Receive Buffer 3"]
pub struct RXBUF3_R(crate::FieldReader<u8, u8>);
impl RXBUF3_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXBUF3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXBUF3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXBUF3` writer - CAN Receive Buffer 3"]
pub struct RXBUF3_W<'a> {
    w: &'a mut W,
}
impl<'a> RXBUF3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - CAN Receive Buffer 0"]
    #[inline(always)]
    pub fn rxbuf0(&self) -> RXBUF0_R {
        RXBUF0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - CAN Receive Buffer 1"]
    #[inline(always)]
    pub fn rxbuf1(&self) -> RXBUF1_R {
        RXBUF1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - CAN Receive Buffer 2"]
    #[inline(always)]
    pub fn rxbuf2(&self) -> RXBUF2_R {
        RXBUF2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - CAN Receive Buffer 3"]
    #[inline(always)]
    pub fn rxbuf3(&self) -> RXBUF3_R {
        RXBUF3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CAN Receive Buffer 0"]
    #[inline(always)]
    pub fn rxbuf0(&mut self) -> RXBUF0_W {
        RXBUF0_W { w: self }
    }
    #[doc = "Bits 8:15 - CAN Receive Buffer 1"]
    #[inline(always)]
    pub fn rxbuf1(&mut self) -> RXBUF1_W {
        RXBUF1_W { w: self }
    }
    #[doc = "Bits 16:23 - CAN Receive Buffer 2"]
    #[inline(always)]
    pub fn rxbuf2(&mut self) -> RXBUF2_W {
        RXBUF2_W { w: self }
    }
    #[doc = "Bits 24:31 - CAN Receive Buffer 3"]
    #[inline(always)]
    pub fn rxbuf3(&mut self) -> RXBUF3_W {
        RXBUF3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Combined CAN Receive Buffer registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxbuf](index.html) module"]
pub struct RXBUF_SPEC;
impl crate::RegisterSpec for RXBUF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxbuf::R](R) reader structure"]
impl crate::Readable for RXBUF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxbuf::W](W) writer structure"]
impl crate::Writable for RXBUF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXBUF to value 0"]
impl crate::Resettable for RXBUF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
