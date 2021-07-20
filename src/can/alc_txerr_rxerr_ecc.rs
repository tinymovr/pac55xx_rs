#[doc = "Register `ALC_TXERR_RXERR_ECC` reader"]
pub struct R(crate::R<ALC_TXERR_RXERR_ECC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALC_TXERR_RXERR_ECC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALC_TXERR_RXERR_ECC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALC_TXERR_RXERR_ECC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALC_TXERR_RXERR_ECC` writer"]
pub struct W(crate::W<ALC_TXERR_RXERR_ECC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALC_TXERR_RXERR_ECC_SPEC>;
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
impl From<crate::W<ALC_TXERR_RXERR_ECC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALC_TXERR_RXERR_ECC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ECC` reader - CAN Error Code Capture"]
pub struct ECC_R(crate::FieldReader<u8, u8>);
impl ECC_R {
    pub(crate) fn new(bits: u8) -> Self {
        ECC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECC` writer - CAN Error Code Capture"]
pub struct ECC_W<'a> {
    w: &'a mut W,
}
impl<'a> ECC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `RXERR` reader - CAN RX Error Counter"]
pub struct RXERR_R(crate::FieldReader<u8, u8>);
impl RXERR_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXERR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXERR` writer - CAN RX Error Counter"]
pub struct RXERR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXERR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `TXERR` reader - CAN TX Error Counter"]
pub struct TXERR_R(crate::FieldReader<u8, u8>);
impl TXERR_R {
    pub(crate) fn new(bits: u8) -> Self {
        TXERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXERR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXERR` writer - CAN TX Error Counter"]
pub struct TXERR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXERR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `ALC` reader - CAN Arbitration Lost Code Capture"]
pub struct ALC_R(crate::FieldReader<u8, u8>);
impl ALC_R {
    pub(crate) fn new(bits: u8) -> Self {
        ALC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALC` writer - CAN Arbitration Lost Code Capture"]
pub struct ALC_W<'a> {
    w: &'a mut W,
}
impl<'a> ALC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - CAN Error Code Capture"]
    #[inline(always)]
    pub fn ecc(&self) -> ECC_R {
        ECC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - CAN RX Error Counter"]
    #[inline(always)]
    pub fn rxerr(&self) -> RXERR_R {
        RXERR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - CAN TX Error Counter"]
    #[inline(always)]
    pub fn txerr(&self) -> TXERR_R {
        TXERR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - CAN Arbitration Lost Code Capture"]
    #[inline(always)]
    pub fn alc(&self) -> ALC_R {
        ALC_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CAN Error Code Capture"]
    #[inline(always)]
    pub fn ecc(&mut self) -> ECC_W {
        ECC_W { w: self }
    }
    #[doc = "Bits 8:15 - CAN RX Error Counter"]
    #[inline(always)]
    pub fn rxerr(&mut self) -> RXERR_W {
        RXERR_W { w: self }
    }
    #[doc = "Bits 16:23 - CAN TX Error Counter"]
    #[inline(always)]
    pub fn txerr(&mut self) -> TXERR_W {
        TXERR_W { w: self }
    }
    #[doc = "Bits 24:31 - CAN Arbitration Lost Code Capture"]
    #[inline(always)]
    pub fn alc(&mut self) -> ALC_W {
        ALC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Combined CAN ECC, RXERR, TXERR, and ALC registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alc_txerr_rxerr_ecc](index.html) module"]
pub struct ALC_TXERR_RXERR_ECC_SPEC;
impl crate::RegisterSpec for ALC_TXERR_RXERR_ECC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [alc_txerr_rxerr_ecc::R](R) reader structure"]
impl crate::Readable for ALC_TXERR_RXERR_ECC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [alc_txerr_rxerr_ecc::W](W) writer structure"]
impl crate::Writable for ALC_TXERR_RXERR_ECC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALC_TXERR_RXERR_ECC to value 0"]
impl crate::Resettable for ALC_TXERR_RXERR_ECC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
