#[doc = "Register `CCTR1` reader"]
pub struct R(crate::R<CCTR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCTR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCTR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCTR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCTR1` writer"]
pub struct W(crate::W<CCTR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCTR1_SPEC>;
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
impl From<crate::W<CCTR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCTR1_SPEC>) -> Self {
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
#[doc = "Timer A CC Counter 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cctr1](index.html) module"]
pub struct CCTR1_SPEC;
impl crate::RegisterSpec for CCTR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cctr1::R](R) reader structure"]
impl crate::Readable for CCTR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cctr1::W](W) writer structure"]
impl crate::Writable for CCTR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCTR1 to value 0"]
impl crate::Resettable for CCTR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
