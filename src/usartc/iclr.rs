#[doc = "Register `ICLR` reader"]
pub struct R(crate::R<ICLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICLR` writer"]
pub struct W(crate::W<ICLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICLR_SPEC>;
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
impl From<crate::W<ICLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICLR_SPEC>) -> Self {
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
#[doc = "SSP C Interrupt Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iclr](index.html) module"]
pub struct ICLR_SPEC;
impl crate::RegisterSpec for ICLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iclr::R](R) reader structure"]
impl crate::Readable for ICLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iclr::W](W) writer structure"]
impl crate::Writable for ICLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICLR to value 0"]
impl crate::Resettable for ICLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
