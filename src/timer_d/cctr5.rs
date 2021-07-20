#[doc = "Register `CCTR5` reader"]
pub struct R(crate::R<CCTR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCTR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCTR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCTR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCTR5` writer"]
pub struct W(crate::W<CCTR5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCTR5_SPEC>;
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
impl From<crate::W<CCTR5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCTR5_SPEC>) -> Self {
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
#[doc = "Timer D CC Counter 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cctr5](index.html) module"]
pub struct CCTR5_SPEC;
impl crate::RegisterSpec for CCTR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cctr5::R](R) reader structure"]
impl crate::Readable for CCTR5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cctr5::W](W) writer structure"]
impl crate::Writable for CCTR5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCTR5 to value 0"]
impl crate::Resettable for CCTR5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
