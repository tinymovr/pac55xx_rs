#[doc = "Register `FCR_RIS` reader"]
pub struct R(crate::R<FCR_RIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCR_RIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCR_RIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCR_RIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCR_RIS` writer"]
pub struct W(crate::W<FCR_RIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCR_RIS_SPEC>;
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
impl From<crate::W<FCR_RIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCR_RIS_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART C FIFO Control / SSP C Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcr_ris](index.html) module"]
pub struct FCR_RIS_SPEC;
impl crate::RegisterSpec for FCR_RIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fcr_ris::R](R) reader structure"]
impl crate::Readable for FCR_RIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcr_ris::W](W) writer structure"]
impl crate::Writable for FCR_RIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCR_RIS to value 0"]
impl crate::Resettable for FCR_RIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
