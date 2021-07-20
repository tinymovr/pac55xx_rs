#[doc = "Register `CCTR0` reader"]
pub struct R(crate::R<CCTR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCTR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCTR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCTR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCTR0` writer"]
pub struct W(crate::W<CCTR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCTR0_SPEC>;
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
impl From<crate::W<CCTR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCTR0_SPEC>) -> Self {
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
#[doc = "Timer C CC Counter 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cctr0](index.html) module"]
pub struct CCTR0_SPEC;
impl crate::RegisterSpec for CCTR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cctr0::R](R) reader structure"]
impl crate::Readable for CCTR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cctr0::W](W) writer structure"]
impl crate::Writable for CCTR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCTR0 to value 0"]
impl crate::Resettable for CCTR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
