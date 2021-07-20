#[doc = "Register `DTSETRIGENT8TO11` reader"]
pub struct R(crate::R<DTSETRIGENT8TO11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTSETRIGENT8TO11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTSETRIGENT8TO11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTSETRIGENT8TO11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTSETRIGENT8TO11` writer"]
pub struct W(crate::W<DTSETRIGENT8TO11_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTSETRIGENT8TO11_SPEC>;
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
impl From<crate::W<DTSETRIGENT8TO11_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTSETRIGENT8TO11_SPEC>) -> Self {
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
#[doc = "DTSE Trigger Entry 8 to 11\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtsetrigent8to11](index.html) module"]
pub struct DTSETRIGENT8TO11_SPEC;
impl crate::RegisterSpec for DTSETRIGENT8TO11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dtsetrigent8to11::R](R) reader structure"]
impl crate::Readable for DTSETRIGENT8TO11_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dtsetrigent8to11::W](W) writer structure"]
impl crate::Writable for DTSETRIGENT8TO11_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DTSETRIGENT8TO11 to value 0"]
impl crate::Resettable for DTSETRIGENT8TO11_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
