#[doc = "Register `LCR_MIS` reader"]
pub struct R(crate::R<LCR_MIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCR_MIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCR_MIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCR_MIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCR_MIS` writer"]
pub struct W(crate::W<LCR_MIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCR_MIS_SPEC>;
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
impl From<crate::W<LCR_MIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCR_MIS_SPEC>) -> Self {
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
#[doc = "UART B Line Control / SSP B Masked Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcr_mis](index.html) module"]
pub struct LCR_MIS_SPEC;
impl crate::RegisterSpec for LCR_MIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcr_mis::R](R) reader structure"]
impl crate::Readable for LCR_MIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcr_mis::W](W) writer structure"]
impl crate::Writable for LCR_MIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCR_MIS to value 0"]
impl crate::Resettable for LCR_MIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
