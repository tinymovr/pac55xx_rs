#[doc = "Register `DTSETRIGENT0TO3` reader"]
pub struct R(crate::R<DTSETRIGENT0TO3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTSETRIGENT0TO3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTSETRIGENT0TO3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTSETRIGENT0TO3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTSETRIGENT0TO3` writer"]
pub struct W(crate::W<DTSETRIGENT0TO3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTSETRIGENT0TO3_SPEC>;
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
impl From<crate::W<DTSETRIGENT0TO3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTSETRIGENT0TO3_SPEC>) -> Self {
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
#[doc = "DTSE Trigger Entry 0 to 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtsetrigent0to3](index.html) module"]
pub struct DTSETRIGENT0TO3_SPEC;
impl crate::RegisterSpec for DTSETRIGENT0TO3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dtsetrigent0to3::R](R) reader structure"]
impl crate::Readable for DTSETRIGENT0TO3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dtsetrigent0to3::W](W) writer structure"]
impl crate::Writable for DTSETRIGENT0TO3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DTSETRIGENT0TO3 to value 0"]
impl crate::Resettable for DTSETRIGENT0TO3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
