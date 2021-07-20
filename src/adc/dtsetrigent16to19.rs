#[doc = "Register `DTSETRIGENT16TO19` reader"]
pub struct R(crate::R<DTSETRIGENT16TO19_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTSETRIGENT16TO19_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTSETRIGENT16TO19_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTSETRIGENT16TO19_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTSETRIGENT16TO19` writer"]
pub struct W(crate::W<DTSETRIGENT16TO19_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTSETRIGENT16TO19_SPEC>;
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
impl From<crate::W<DTSETRIGENT16TO19_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTSETRIGENT16TO19_SPEC>) -> Self {
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
#[doc = "DTSE Trigger Entry 16 to 19\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtsetrigent16to19](index.html) module"]
pub struct DTSETRIGENT16TO19_SPEC;
impl crate::RegisterSpec for DTSETRIGENT16TO19_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dtsetrigent16to19::R](R) reader structure"]
impl crate::Readable for DTSETRIGENT16TO19_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dtsetrigent16to19::W](W) writer structure"]
impl crate::Writable for DTSETRIGENT16TO19_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DTSETRIGENT16TO19 to value 0"]
impl crate::Resettable for DTSETRIGENT16TO19_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
