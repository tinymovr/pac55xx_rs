#[doc = "Register `DTSETRIGENT28TO31` reader"]
pub struct R(crate::R<DTSETRIGENT28TO31_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTSETRIGENT28TO31_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTSETRIGENT28TO31_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTSETRIGENT28TO31_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTSETRIGENT28TO31` writer"]
pub struct W(crate::W<DTSETRIGENT28TO31_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTSETRIGENT28TO31_SPEC>;
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
impl From<crate::W<DTSETRIGENT28TO31_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTSETRIGENT28TO31_SPEC>) -> Self {
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
#[doc = "DTSE Trigger Entry 28 to 31\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtsetrigent28to31](index.html) module"]
pub struct DTSETRIGENT28TO31_SPEC;
impl crate::RegisterSpec for DTSETRIGENT28TO31_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dtsetrigent28to31::R](R) reader structure"]
impl crate::Readable for DTSETRIGENT28TO31_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dtsetrigent28to31::W](W) writer structure"]
impl crate::Writable for DTSETRIGENT28TO31_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DTSETRIGENT28TO31 to value 0"]
impl crate::Resettable for DTSETRIGENT28TO31_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
