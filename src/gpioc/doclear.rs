#[doc = "Register `DOCLEAR` reader"]
pub struct R(crate::R<DOCLEAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOCLEAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOCLEAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOCLEAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOCLEAR` writer"]
pub struct W(crate::W<DOCLEAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOCLEAR_SPEC>;
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
impl From<crate::W<DOCLEAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOCLEAR_SPEC>) -> Self {
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
#[doc = "GPIO C Data Output Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doclear](index.html) module"]
pub struct DOCLEAR_SPEC;
impl crate::RegisterSpec for DOCLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doclear::R](R) reader structure"]
impl crate::Readable for DOCLEAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doclear::W](W) writer structure"]
impl crate::Writable for DOCLEAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DOCLEAR to value 0"]
impl crate::Resettable for DOCLEAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
