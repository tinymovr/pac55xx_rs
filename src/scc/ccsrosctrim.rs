#[doc = "Register `CCSROSCTRIM` reader"]
pub struct R(crate::R<CCSROSCTRIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCSROSCTRIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCSROSCTRIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCSROSCTRIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCSROSCTRIM` writer"]
pub struct W(crate::W<CCSROSCTRIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCSROSCTRIM_SPEC>;
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
impl From<crate::W<CCSROSCTRIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCSROSCTRIM_SPEC>) -> Self {
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
#[doc = "DESC\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccsrosctrim](index.html) module"]
pub struct CCSROSCTRIM_SPEC;
impl crate::RegisterSpec for CCSROSCTRIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccsrosctrim::R](R) reader structure"]
impl crate::Readable for CCSROSCTRIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccsrosctrim::W](W) writer structure"]
impl crate::Writable for CCSROSCTRIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCSROSCTRIM to value 0"]
impl crate::Resettable for CCSROSCTRIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
